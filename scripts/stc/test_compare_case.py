import importlib.util
from pathlib import Path
import unittest
import tempfile
import json
import xml.etree.ElementTree as ET
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('compare_case', Path(__file__).with_name('compare_case.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)

class ComparisonTests(unittest.TestCase):
    def facts(self, words):
        return {'pages': len(words), 'page_boxes': [[595, 841]] * len(words), 'words': words, 'dates': [[]] * len(words)}

    def test_duplicate_and_page_movement_are_not_hidden(self):
        a = self.facts([['A', 'A'], ['B']])
        b = self.facts([['A', 'B'], ['A']])
        result = module.compare(a, b)
        self.assertTrue(result['word_multiset_equal'])
        self.assertFalse(result['ordered_words_equal'])
        self.assertEqual(result['visual_acceptance'], 'not-assessed-by-script')
        self.assertFalse(module.compare(a, self.facts([['A'], ['B']]))['word_multiset_equal'])

    def test_unicode_keeps_chinese_and_arabic_letters(self):
        self.assertEqual(module.normalize('\u202b学生 學生 العربية\u202c'), '学生 學生 العربية')
        self.assertEqual(module.normalize('e\u0301'), 'é')

    def test_different_paper_or_date_is_visible(self):
        a, b = self.facts([['A']]), self.facts([['A']])
        b['page_boxes'][0] = [612, 792]
        b['dates'][0] = ['17/10/2026']
        result = module.compare(a, b)
        self.assertFalse(result['page_boxes_equal'])
        self.assertFalse(result['dates_per_page_equal'])

    def test_html_ignores_stale_trailing_pngs(self):
        with tempfile.TemporaryDirectory() as folder:
            directory = Path(folder)
            (directory / 'evidence.json').write_text(json.dumps({'caseId': 'synthetic', 'stages': {'native-frozen': 'passed'}}))
            (directory / 'native-frozen-99.png').write_bytes(b'stale')
            with patch.object(module, 'inspect', return_value=self.facts([['A'], ['B']])), patch('builtins.print'):
                module.run(directory)
            output = (directory / 'comparison.html').read_text()
            self.assertIn('native-frozen-1.png', output)
            self.assertIn('native-frozen-2.png', output)
            self.assertNotIn('native-frozen-99.png', output)
            self.assertIn('native-frozen.pdf', output)

class PrintedPageLabelTests(unittest.TestCase):
    def page(self, lines):
        page = ET.Element(module.NS + 'page')
        for words in lines:
            line = ET.SubElement(page, module.NS + 'line')
            for word in words:
                ET.SubElement(line, module.NS + 'word').text = word
        return page

    def facts(self, labels):
        result = ComparisonTests().facts([['unchanged']] * len(labels))
        result['printed_page_labels'] = labels
        return result

    def test_bbox_labels_preserve_reset_sequences_and_bidi_controls(self):
        pages = [self.page([['\u202bPage', str(n), 'of', str(total) + '\u202c']])
                 for n, total in [(1, 2), (2, 2), (1, 1)]]
        labels = [module.printed_page_labels(page) for page in pages]
        self.assertEqual(labels, [[[1, 2]], [[2, 2]], [[1, 1]]])
        result = module.compare(self.facts(labels), self.facts(labels))
        self.assertTrue(result['printed_page_labels_equal'])
        self.assertTrue(result['printed_page_labels_complete'])

    def test_same_physical_count_does_not_hide_wrong_logical_totals(self):
        native = self.facts([[[1, 2]], [[2, 2]], [[1, 1]]])
        rust = self.facts([[[1, 3]], [[2, 3]], [[1, 3]]])
        result = module.compare(native, rust)
        self.assertTrue(result['page_count_equal'])
        self.assertTrue(result['printed_page_labels_complete'])
        self.assertFalse(result['printed_page_labels_equal'])

    def test_missing_duplicate_or_unavailable_labels_do_not_establish_coverage(self):
        reference = self.facts([[[1, 1]]])
        for labels in [[], [[]], [[[1, 1], [1, 1]]]]:
            with self.subTest(labels=labels):
                facts = self.facts(labels)
                self.assertFalse(module.compare(reference, facts)['printed_page_labels_complete'])
                self.assertFalse(module.compare(reference, facts)['printed_page_labels_equal'])
        unknown = ComparisonTests().facts([['Page', '1', 'of', '1']])
        result = module.compare(reference, unknown)
        self.assertIsNone(result['printed_page_labels_equal'])
        self.assertFalse(result['printed_page_labels_complete'])

    def test_labels_are_not_invented_across_lines_and_duplicates_are_retained(self):
        self.assertEqual(module.printed_page_labels(self.page([['Page', '1'], ['of', '2']])), [])
        self.assertEqual(module.printed_page_labels(self.page([
            ['Page 1 of 2'], ['Page', '1', 'of', '2'], ['學生', 'العربية']])), [[1, 2], [1, 2]])


if __name__ == '__main__':
    unittest.main()
