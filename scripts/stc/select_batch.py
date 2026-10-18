"""Deterministic selection for bounded private report comparisons; no SQL or file I/O."""


def select_candidate(candidates, previous, template):
    """Prefer unseen row counts, then larger reports and actual Arabic-name coverage.

    Previous contains every captured student, including rejected comparisons. IDs are
    excluded globally; row-count novelty is specific to the requested template.
    Records are caller-normalized dictionaries; private identifiers never need logging.
    """
    excluded = {row['student_id'] for row in previous}
    seen_counts = {row['row_count'] for row in previous if row['template'] == template}
    pool = [row for row in candidates if row['template'] == template and row['ready']
            and row['student_id'] not in excluded]
    if not pool:
        return None
    if len({row['student_id'] for row in pool}) != len(pool):
        raise ValueError('Ambiguous duplicate student candidates')
    if any(row['row_count'] <= 0 for row in pool):
        raise ValueError('Ready candidate must contain report rows')
    return min(pool, key=lambda row: (
        row['row_count'] in seen_counts, -row['row_count'],
        -bool(row['has_arabic_name']), -row['arabic_name_length'],
        -row['english_name_length'], row['student_id']))
