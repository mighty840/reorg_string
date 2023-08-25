# Reorganize String

## Intuition

First checking if there is a letter which is more frequent than half the length of the string. If yes, there is no solution. If no, it can be solved.

Then performed a check if it is a single length or double length string. Both cases are easy to check.

For strings longer than 2, the idea is to swap the next neighbor from the right  till a different character is found and continue doing so treating the string as a ring buffer. The number of repetitions are determined by the number of occurences of the repeating letter in the string.

In the end check if there is still a solution coming out, otherwise return an empty string.

This is based on modelling it on a state-machine / turing machine solution.

## Approach

Create a vector from string.

Count the number of repetitions per letter recursively (might contain repetitions) and put them in another tuple vector.

This can be improved by checking if the result vector already contains the letter and its count.

Deduplicate them (can be saved by above checking before adding duplicates) and sort them and pick the most frequent element and its count as the first tuple element.

Check if the most frequent element count is more than half the length and return empty string

Otherwise start performing swaps and check in the end if it was successfull, otherwise return empty string

## Complexity

- Time complexity:
Since we are carrying our nested for loops for the entire array the time complexity becomes $$0(n²)$$ and there is another loop which is running for number of repetitions of elements, increasing the complexity to $$0(n³)$$

- Space complexity:
The space complexity is $$O(1)$$ since we are reusing the same space for swapping elements

## Code

```Rust
impl Solution {
    #[allow(unused)]
    fn reorganize_string(s: String) -> String {
        let mut s1 = s.as_bytes().to_vec();
        let default_string = "".to_string();
        let len = s1.len();

        // check different types and their repetitions
        let mut s_reps = Vec::<usize>::new();

        for index in 0..len {
            let mut elements = 0;
            for jindex in 0..len {
                if s1[index] == s1[jindex] {
                    elements = elements + 1
                }
            }
            s_reps.insert(index, elements)
        }
        s_reps.dedup();
        s_reps.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let max_elem = s_reps.get(0).unwrap();
        if *max_elem > ((len / 2) + 1) {
            return default_string;
        }

        match len {
            1 => s,
            2 => {
                if s1[0] == s1[1] {
                    default_string
                } else {
                    s
                }
            }
            _ => {
                for swap_iter in 0..*max_elem {
                    let mut swap = false;
                    for index in 0..(len - 1) {
                        if s1[index] == s1[index + 1] {
                            swap = true;
                            break;
                        }
                    }
                    for index in 0..len {
                        for jindex in index..len {
                            if jindex == index {
                                continue;
                            } else if s1[index] != s1[jindex - 1] {
                                break;
                            } else {
                                let mut start_index = jindex % len;
                                while start_index != index {
                                    if s1[index] == s1[jindex] {
                                        s1.swap(jindex, start_index);
                                    } else {
                                        break;
                                    }
                                    start_index = (start_index + 1) % len;
                                }
                            }
                        }
                    }
                }
                // check if swapping was successful
                let mut swap = true;
                for index in 0..(len - 1) {
                    if s1[index] == s1[index + 1] {
                        swap = false;
                        break;
                    }
                }
                if swap {
                    String::from_utf8(s1).unwrap_or(default_string)
                } else {
                    default_string
                }
            }
        }
    }
}
```
