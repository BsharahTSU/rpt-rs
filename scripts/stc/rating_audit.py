"""Audit an explicitly selected evaluation-rating column against captured aliases."""
from collections import Counter

NS = '{http://www.w3.org/1999/xhtml}'


def extract_column_tokens(root, *, x_min, x_max, y_max):
    """Keep every token wholly inside the column, including symbols/unknown aliases.

    The caller must establish template bounds that exclude headings and legends.
    Do not use a letter allowlist: a valid source alias may be '*'.
    """
    return [[word.text or '' for word in page.iter(NS + 'word')
             if x_min < float(word.attrib['xMin']) and float(word.attrib['xMax']) < x_max
             and float(word.attrib['yMin']) < y_max]
            for page in root.iter(NS + 'page')]


def compare_ratings(native, rust, expected_aliases):
    """Check per-page order between engines and multiplicity against captured data.

    Snapshot order is not assumed to be display order: template grouping can change it.
    This is a data gate, not a substitute for layout or visual inspection.
    """
    expected = Counter(expected_aliases)
    inventories = [Counter(value for page in pages for value in page) for pages in (native, rust)]
    ordered_equal = native == rust
    source_equal = all(values == expected for values in inventories)
    return {'ratings_equal': ordered_equal, 'ratings_match_snapshot': source_equal,
            'ratings_per_page': [len(page) for page in native],
            'rating_symbols': dict(inventories[0]), 'expected_ratings': sum(expected.values()),
            'ratings_passed': bool(expected) and bool(native) and ordered_equal and source_equal}
