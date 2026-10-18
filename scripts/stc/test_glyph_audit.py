import json
import struct
import unittest
from glyph_audit import simple_glyph_outline, compare_outline_pages


def glyph(points, instructions=b''):
    # Deliberately use an uninformative bbox: actual decoded coordinates decide.
    header = struct.pack('>hhhhhHH', 1, 0, 0, 0, 0, len(points) - 1, len(instructions))
    axes = []
    for dimension in (0, 1):
        previous = 0
        for point in points:
            axes.append(struct.pack('>h', point[dimension] - previous))
            previous = point[dimension]
    return header + instructions + bytes([1] * len(points)) + b''.join(axes)


class GlyphAuditTests(unittest.TestCase):
    def test_single_point_space_is_nonpainting(self):
        # Independent valid TrueType fixture: one contour, one point at (0,0).
        self.assertIsNone(simple_glyph_outline(bytes.fromhex('00010000000000000000000000003100')))
        self.assertIsNone(simple_glyph_outline(glyph([(17, 23)])))

    def test_empty_and_axis_degenerate_glyphs_are_nonpainting(self):
        for data in (b'', bytes(10), glyph([(0, 0), (0, 30)]), glyph([(1, 9), (20, 9)])):
            self.assertIsNone(simple_glyph_outline(data))

    def test_actual_coordinates_keep_visible_outline(self):
        self.assertEqual(json.loads(simple_glyph_outline(glyph([(0, 0), (20, 0), (0, 30)]))),
                         [[2], [[0, 0, 1], [20, 0, 1], [0, 30, 1]]])

    def test_hints_do_not_change_outline(self):
        points = [(0, 0), (20, 0), (0, 30)]
        self.assertEqual(simple_glyph_outline(glyph(points)), simple_glyph_outline(glyph(points, b'\x01\x02')))

    def test_malformed_or_composite_data_fails_explicitly(self):
        for data in (b'\x00', b'\xff\xff' + bytes(8), glyph([(0, 0), (20, 30)])[:-1],
                     bytes.fromhex('00010000000000000000000000003902')):
            with self.assertRaises(ValueError):
                simple_glyph_outline(data)

    def row(self, text='学生', x=10, y=20, outline='visible', size=9):
        return {'t': text, 'x': x, 'y': y, 'outline': outline, 'size': size}

    def test_equivalent_outline_with_unicode_alias_and_rounding(self):
        result = compare_outline_pages([[self.row('ع')]], [[self.row('عـ', 10.06, 20.03)]])
        self.assertTrue(result['geometry_passed'])
        self.assertEqual(result['pages'][0]['different_unicode_mappings'], 1)

    def test_chinese_missing_duplicate_or_substituted_glyph_fails(self):
        left = [[self.row(), self.row(x=20)]]
        for right in ([[self.row()]], [[self.row(), self.row()]],
                      [[self.row(), self.row(x=20, outline='different')]]):
            self.assertFalse(compare_outline_pages(left, right)['geometry_passed'])

    def test_wrap_page_font_and_mark_shift_fail(self):
        left = [[self.row()]]
        for right in ([[self.row(y=32)]], [[], [self.row()]],
                      [[self.row(size=10)]], [[self.row(y=20.04)]]):
            self.assertFalse(compare_outline_pages(left, right)['geometry_passed'])

    def test_empty_report_does_not_pass(self):
        self.assertFalse(compare_outline_pages([], [])['geometry_passed'])

    def test_nonfinite_coordinates_and_tolerances_do_not_pass(self):
        self.assertFalse(compare_outline_pages([[self.row()]], [[self.row(x=float('nan'))]])['geometry_passed'])
        with self.assertRaises(ValueError):
            compare_outline_pages([], [], x_tolerance=float('inf'))


if __name__ == '__main__':
    unittest.main()
