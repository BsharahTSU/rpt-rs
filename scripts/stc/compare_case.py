#!/usr/bin/env python3
"""Inspect isolated private PDF runs; never promote text metrics to visual acceptance."""
import argparse
import collections
import hashlib
import html
import json
from pathlib import Path
import re
import subprocess
import unicodedata
import xml.etree.ElementTree as ET

NS = '{http://www.w3.org/1999/xhtml}'
ENGINES = ('native-live', 'native-frozen', 'rust-frozen')


def normalize(text):
    return ''.join(c for c in unicodedata.normalize('NFC', text) if unicodedata.category(c) != 'Cf')


def compare(left, right):
    """Retain order and multiplicity; coordinate-free counters are only diagnostics."""
    a, b = left['words'], right['words']
    ca, cb = collections.Counter(w for page in a for w in page), collections.Counter(w for page in b for w in page)
    return {
        'page_count_equal': left['pages'] == right['pages'],
        'page_boxes_equal': left['page_boxes'] == right['page_boxes'],
        'ordered_words_equal': a == b,
        'word_multiset_equal': ca == cb,
        'only_left': dict(ca - cb), 'only_right': dict(cb - ca),
        'dates_per_page_equal': left['dates'] == right['dates'],
        'visual_acceptance': 'not-assessed-by-script',
    }


def inspect(directory, engine):
    path = directory / f'{engine}.pdf'
    check = subprocess.run(['qpdf', '--check', str(path)], capture_output=True, text=True, timeout=60)
    (directory / f'{engine}.qpdf.log').write_text(check.stdout + check.stderr, encoding='utf-8')
    if check.returncode:
        raise ValueError(f'{engine}: qpdf rejected the PDF (including repair warnings)')
    xml = subprocess.check_output(['pdftotext', '-bbox-layout', str(path), '-'], timeout=60)
    (directory / f'{engine}.bbox.xml').write_bytes(xml)
    pages = list(ET.fromstring(xml).iter(NS + 'page'))
    if not pages:
        raise ValueError(f'{engine}: no pages')
    words = [[normalize(w.text) for w in page.iter(NS + 'word') if w.text and normalize(w.text)] for page in pages]
    subprocess.run(['pdftoppm', '-r', '96', '-png', str(path), str(directory / engine)], check=True, timeout=120, capture_output=True)
    return {
        'pages': len(pages), 'page_boxes': [[float(p.attrib['width']), float(p.attrib['height'])] for p in pages],
        'words': words,
        'dates': [[w for w in row if re.fullmatch(r'\d{1,2}[/.\-]\d{1,2}[/.\-]\d{2,4}', w)] for row in words],
        'bytes': path.stat().st_size, 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
        'integrity': 'passed',
    }


def run(directory):
    evidence = json.loads((directory / 'evidence.json').read_text(encoding='utf-8-sig'))
    result = {'caseId': evidence['caseId'], 'engines': {}, 'comparisons': {}, 'visual': 'pending-human-review'}
    for engine in ENGINES:
        if evidence['stages'].get(engine) != 'passed':
            continue  # Never reuse a file from a failed stage.
        result['engines'][engine] = inspect(directory, engine)
    for a, b in [('native-live', 'native-frozen'), ('native-frozen', 'rust-frozen')]:
        if a in result['engines'] and b in result['engines']:
            result['comparisons'][a + '_vs_' + b] = compare(result['engines'][a], result['engines'][b])
    (directory / 'comparison.json').write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')
    panes = []
    for engine, facts in result['engines'].items():
        # Poppler zero-pads page numbers to the total page count's width. Select only current
        # pages; a previous longer render must not leak stale trailing images into this view.
        width = len(str(facts['pages']))
        images = [directory / f'{engine}-{i:0{width}d}.png' for i in range(1, facts['pages'] + 1)]
        panes.append('<section><h2>' + html.escape(engine) + f" ({facts['pages']} pages)</h2><p><a href=\"{engine}.pdf\">Open PDF</a></p>" + ''.join(
            f'<figure><figcaption>Page {i}</figcaption><img src="{html.escape(p.name, quote=True)}" alt="Page {i}"></figure>'
            for i, p in enumerate(images, 1)) + '</section>')
    document = '<!doctype html><meta charset="utf-8"><title>Crystal comparison</title><style>body{font:16px system-ui;margin:24px;background:#eef1f5;color:#182337}main{display:flex;gap:20px;align-items:flex-start}section{min-width:0;flex:1}h2{position:sticky;top:0;background:#eef1f5;padding:12px}figure{margin:0 0 24px}img{width:100%;box-shadow:0 2px 10px #bcc5cf}figcaption{padding:8px}</style><h1>' + html.escape(evidence['caseId']) + '</h1><p>Private local comparison. Structural/text checks do not establish visual parity. Review every page and record findings separately.</p><main>' + ''.join(panes) + '</main>'
    (directory / 'comparison.html').write_text(document, encoding='utf-8')
    print(json.dumps({'caseId': result['caseId'], 'pages': {k: v['pages'] for k, v in result['engines'].items()},
        'comparisons': {k: {name: value for name, value in v.items() if name not in ('only_left', 'only_right')} for k, v in result['comparisons'].items()}}))


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('directory', type=Path)
    run(parser.parse_args().directory)
