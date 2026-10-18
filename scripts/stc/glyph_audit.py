"""Pure helpers for the local PDF glyph audit; no renderer behavior is changed."""
import collections
import json
import math
import struct


def simple_glyph_outline(data):
    """Canonicalize a TrueType simple glyph, returning None for nonpainting outlines.

    Ignore hint instructions and encoded-coordinate representation. Inspect actual
    coordinates instead of Unicode text or the header bounding box: some space
    glyphs contain one contour with a single point, which paints no filled area.
    Composite glyphs must be resolved by the caller.
    """
    if not data:
        return None
    try:
        contours = struct.unpack_from('>h', data, 0)[0]
        if contours < 0:
            raise ValueError('Composite glyph requires separate resolution')
        if len(data) < 10:
            raise ValueError('Truncated glyph header')
        if contours == 0:
            return None
        ends = list(struct.unpack_from('>' + 'H' * contours, data, 10))
        if any(a >= b for a, b in zip(ends, ends[1:])):
            raise ValueError('Invalid contour endpoints')
        offset = 10 + contours * 2
        instructions = struct.unpack_from('>H', data, offset)[0]
        offset += 2 + instructions
        if offset > len(data):
            raise ValueError('Truncated glyph instructions')
        count = ends[-1] + 1
        flags = []
        while len(flags) < count:
            flag = data[offset]
            offset += 1
            repeat = 1
            if flag & 8:
                repeat += data[offset]
                offset += 1
            if len(flags) + repeat > count:
                raise ValueError('Glyph flag repeat exceeds point count')
            flags.extend([flag] * repeat)
        axes = []
        for short_bit, same_bit in ((2, 16), (4, 32)):
            values = []
            position = 0
            for flag in flags:
                if flag & short_bit:
                    delta = data[offset] * (1 if flag & same_bit else -1)
                    offset += 1
                elif flag & same_bit:
                    delta = 0
                else:
                    delta = struct.unpack_from('>h', data, offset)[0]
                    offset += 2
                position += delta
                values.append(position)
            axes.append(values)
        xs, ys = axes
        # A zero-width or zero-height outline cannot paint a filled glyph. This
        # does not exclude visible punctuation, combining marks or Unicode spaces.
        if min(xs) == max(xs) or min(ys) == max(ys):
            return None
        return json.dumps([ends, [[x, y, flag & 1] for x, y, flag in zip(xs, ys, flags)]],
                          separators=(',', ':')).encode('ascii')
    except (IndexError, struct.error) as error:
        raise ValueError('Truncated TrueType simple glyph') from error


def compare_outline_pages(native, rust, x_tolerance=0.061, y_tolerance=0.031):
    """Match every painting glyph once, per page, by outline, size and position.

    Extraction mappings may differ while the painted outline agrees. This check
    reports only glyph geometry; images, paths and all-page review are separate.
    """
    if any(not math.isfinite(v) or v < 0 for v in (x_tolerance, y_tolerance)):
        raise ValueError('Tolerances must be finite and nonnegative')
    result = {'page_count_equal': len(native) == len(rust), 'pages': [],
              'geometry_passed': bool(native) and len(native) == len(rust)}
    for index, (left, right) in enumerate(zip(native, rust), 1):
        key = lambda g: (g['outline'], g['size'])
        inventory_equal = collections.Counter(map(key, left)) == collections.Counter(map(key, right))
        groups = collections.defaultdict(list)
        for glyph in right:
            groups[key(glyph)].append(glyph)
        matches, unmatched = [], 0
        for glyph in sorted(left, key=lambda g: (key(g), g['y'], g['x'])):
            choices = groups[key(glyph)]
            candidates = [other for other in choices
                          if abs(other['x'] - glyph['x']) <= x_tolerance
                          and abs(other['y'] - glyph['y']) <= y_tolerance]
            if not candidates:
                unmatched += 1
                continue
            other = min(candidates, key=lambda g: math.hypot(g['x'] - glyph['x'], g['y'] - glyph['y']))
            choices.remove(other)
            matches.append((glyph, other))
        extra = sum(map(len, groups.values()))
        passed = inventory_equal and unmatched == 0 and extra == 0
        result['geometry_passed'] &= passed
        result['pages'].append({'page': index, 'native_glyphs': len(left), 'rust_glyphs': len(right),
            'outline_size_inventory_equal': inventory_equal, 'matched': len(matches),
            'unmatched_native': unmatched, 'unmatched_rust': extra,
            'max_x_difference': max((abs(a['x'] - b['x']) for a, b in matches), default=0),
            'max_y_difference': max((abs(a['y'] - b['y']) for a, b in matches), default=0),
            'different_unicode_mappings': sum(a.get('t') != b.get('t') for a, b in matches),
            'passed': passed})
    return result
