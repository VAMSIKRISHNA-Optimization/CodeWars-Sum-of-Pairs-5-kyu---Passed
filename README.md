# CodeWars-Sum-of-Pairs-5-kyu---Passed
Sum of Pairs
Given a list of integers and a single sum value, return the first two values (parse from the left please) in order of appearance that add up to form the sum.

If there are two or more pairs with the required sum, the pair whose second element has the smallest index is the solution.

sum_pairs([11, 3, 7, 5],         10)
#              ^--^      3 + 7 = 10
== [3, 7]

sum_pairs([4, 3, 2, 3, 4],         6)
#          ^-----^         4 + 2 = 6, indices: 0, 2 *
#             ^-----^      3 + 3 = 6, indices: 1, 3
#                ^-----^   2 + 4 = 6, indices: 2, 4
#  * the correct answer is the pair whose second value has the smallest index
== [4, 2]

sum_pairs([0, 0, -2, 3], 2)
#  there are no pairs of values that can be added to produce 2.
== None/nil/undefined/Nothing (Based on the language)

sum_pairs([10, 5, 2, 3, 7, 5],         10)
#              ^-----------^   5 + 5 = 10, indices: 1, 5
#                    ^--^      3 + 7 = 10, indices: 3, 4 *
#  * the correct answer is the pair whose second value has the smallest index
== [3, 7]
Negative numbers and duplicate numbers can and will appear.

NOTE: There will also be lists tested of lengths upwards of 10,000,000 elements. Be sure your code doesn't time out.


TEST CASES:
#[cfg(test)]
mod tests {
    use super::sum_pairs;
    use rand::prelude::*;

    fn do_test(ints: &[i32], s: i32, expected: Option<(i32, i32)>) {
        let actual = sum_pairs(ints, s);
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right), given ints = {ints:?} and s = {s}",
        );
    }

    #[test]
    fn _1_simple_test() {
        do_test(&[1, 4, 8, 7, 3, 15], 8, Some((1, 7)));
        do_test(&[1, -2, 3, 0, -6, 1], -6, Some((0, -6)));
        do_test(&[20, -13, 40], -7, None);
        do_test(&[1, 2, 3, 4, 1, 0], 2, Some((1, 1)));
        do_test(&[10, 5, 2, 3, 7, 5], 10, Some((3, 7)));
        do_test(&[4, -2, 3, 3, 4], 8, Some((4, 4)));
        do_test(&[0, 2, 0], 0, Some((0, 0)));
        do_test(&[5, 9, 13, -3], 10, Some((13, -3)));
    }

    #[test]
    fn _2_excruciatingly_long_list_tests() {
        let mut l9 = vec![1; 2_000_000];
        let len = l9.len();
        l9[len / 2 - 1] = 6;
        l9[len / 2] = 7;
        l9[len - 2] = 8;
        l9[len - 1] = -3;
        l9[0] = 13;
        l9[1] = 3;
        do_test(&l9, 13, Some((6, 7)));
        do_test(&l9, 5, Some((8, -3)));
        do_test(&l9, 16, Some((13, 3)));
        do_test(&l9, 31, None);
    }

    #[test]
    fn _3_random_tests() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let n = rng.gen_range(1..100);
            let s = rng.gen_range(1..100);
            let ints: Vec<i32> = (0..n).map(|_| rng.gen_range(1..100)).collect();
            do_test(&ints, s, refsol(&ints, s));
        }
    }

    fn refsol(ints: &[i32], s: i32) -> Option<(i32, i32)> {
        let mut h = std::collections::HashSet::new();
        ints.windows(2).find_map(|v| {
            h.insert(v[0]);
            h.contains(&(s - v[1])).then_some((s - v[1], v[1]))
        })
    }
}
