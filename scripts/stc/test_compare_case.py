import importlib.util
from pathlib import Path
import unittest

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

if __name__ == '__main__':
    unittest.main()
