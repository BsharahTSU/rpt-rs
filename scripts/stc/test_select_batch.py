import unittest
from select_batch import select_candidate


def candidate(student_id, rows=20, template='A', ready=True, arabic=True, ar_length=10, en_length=10):
    return dict(student_id=student_id, row_count=rows, template=template, ready=ready,
                has_arabic_name=arabic, arabic_name_length=ar_length, english_name_length=en_length)


class BatchSelectionTests(unittest.TestCase):
    def test_prior_rejection_and_other_template_student_are_not_reselected(self):
        previous = [dict(candidate(1), status='rejected'), candidate(2, template='B')]
        selected = select_candidate([candidate(1, 90), candidate(2, 80), candidate(3)], previous, 'A')
        self.assertEqual(selected['student_id'], 3)

    def test_new_row_count_precedes_larger_repeat_and_novelty_is_template_local(self):
        previous = [candidate(1, 50), candidate(2, 30, template='B')]
        selected = select_candidate([candidate(3, 50), candidate(4, 20), candidate(5, 30)], previous, 'A')
        self.assertEqual(selected['student_id'], 5)

    def test_actual_arabic_precedes_long_latin_text_in_arabic_name_field(self):
        previous = [candidate(1, 35)]
        selected = select_candidate([candidate(2, 35, arabic=False, ar_length=60),
                                     candidate(3, 35, ar_length=15)], previous, 'A')
        self.assertEqual(selected['student_id'], 3)

    def test_ineligible_and_wrong_template_cannot_fill_an_empty_batch(self):
        self.assertIsNone(select_candidate([candidate(1, ready=False), candidate(2, template='B')], [], 'A'))
        self.assertIsNone(select_candidate([candidate(1)], [candidate(1)], 'A'))

    def test_equal_features_use_stable_id_tie_break(self):
        rows = [candidate(9), candidate(8)]
        self.assertEqual(select_candidate(rows, [], 'A')['student_id'], 8)
        self.assertEqual(select_candidate(list(reversed(rows)), [], 'A')['student_id'], 8)

    def test_ambiguous_or_empty_ready_candidates_fail(self):
        for rows in ([candidate(1), candidate(1)], [candidate(1, 0)]):
            with self.assertRaises(ValueError):
                select_candidate(rows, [], 'A')


    def test_exhausted_row_novelty_still_selects_largest_untested_student(self):
        previous = [candidate(1, 20), candidate(2, 35)]
        pool = [candidate(1, 90), candidate(3, 20), candidate(4, 35)]
        self.assertEqual(select_candidate(pool, previous, 'A')['student_id'], 4)

    def test_repaired_capture_history_does_not_reselect_or_change_novelty(self):
        previous = [dict(candidate(1, 35), status='rejected'),
                    dict(candidate(1, 35), status='accepted-after-repair')]
        pool = [candidate(1, 90), candidate(2, 35), candidate(3, 20)]
        self.assertEqual(select_candidate(pool, previous, 'A')['student_id'], 3)

    def test_reserving_first_choice_excludes_same_student_from_next_template(self):
        pool = [candidate(1, 40, 'A'), candidate(1, 50, 'B'), candidate(2, 20, 'B')]
        first = select_candidate(pool, [], 'A')
        second = select_candidate(pool, [first], 'B')
        self.assertEqual([first['student_id'], second['student_id']], [1, 2])


if __name__ == '__main__':
    unittest.main()
