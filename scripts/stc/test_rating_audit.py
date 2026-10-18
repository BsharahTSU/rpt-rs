import unittest
import xml.etree.ElementTree as ET
from rating_audit import extract_column_tokens, compare_ratings


class RatingAuditTests(unittest.TestCase):
    def test_symbols_and_unknown_aliases_survive_column_extraction(self):
        xml = ET.fromstring('''<html xmlns="http://www.w3.org/1999/xhtml"><page>
          <word xMin="282" xMax="310" yMin="210">Rating*</word>
          <word xMin="317" xMax="337" yMin="210">rating-heading</word>
          <word xMin="318" xMax="322" yMin="500">(</word>
          <word xMin="309" xMax="315" yMin="250">O</word>
          <word xMin="309" xMax="315" yMin="275">*</word>
          <word xMin="309" xMax="315" yMin="300">?</word>
          <word xMin="45" xMax="49" yMin="700">*</word>
          <word xMin="309" xMax="315" yMin="780">N</word>
        </page><page><word xMin="309" xMax="315" yMin="100">C</word></page></html>''')
        self.assertEqual(extract_column_tokens(xml, x_min=300, x_max=320, y_max=760),
                         [['O', '*', '?'], ['C']])

    def test_asterisks_match_snapshot_even_when_grouping_reorders_rows(self):
        pages = [['O', '*', '*'], ['C']]
        result = compare_ratings(pages, pages, ['*', 'C', 'O', '*'])
        self.assertTrue(result['ratings_passed'])
        self.assertEqual(result['ratings_per_page'], [3, 1])
        self.assertEqual(result['rating_symbols']['*'], 2)

    def test_identical_engines_cannot_hide_missing_duplicate_or_unknown_rating(self):
        for wrong in ([['O']], [['O', '*', '*']], [['O', '?']]):
            with self.subTest(wrong=wrong):
                result = compare_ratings(wrong, wrong, ['O', '*'])
                self.assertTrue(result['ratings_equal'])
                self.assertFalse(result['ratings_match_snapshot'])
                self.assertFalse(result['ratings_passed'])

    def test_order_page_movement_and_empty_data_fail(self):
        expected = ['O', '*', 'C']
        native = [['O', '*'], ['C']]
        for rust in ([['*', 'O'], ['C']], [['O'], ['*', 'C']], native + [[]]):
            result = compare_ratings(native, rust, expected)
            self.assertTrue(result['ratings_match_snapshot'])
            self.assertFalse(result['ratings_passed'])
        self.assertFalse(compare_ratings([], [], [])['ratings_passed'])


if __name__ == '__main__':
    unittest.main()
