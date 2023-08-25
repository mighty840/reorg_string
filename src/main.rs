#[allow(unused)]
struct Solution {}

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

fn main() {
    println!("Hello, World");
    let test_string = "aabbb".to_string();
    Solution::reorganize_string(test_string);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_l_string() {
        // Arrange
        let test_string = "a".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        // Assert
        assert_eq!(result_string, "".to_string());
    }
    #[test]
    fn test_2_l_string() {
        // Arrange
        let test_string = "aa".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        // Assert
        assert_eq!(result_string, "".to_string());
    }
    #[test]
    fn test_2_l_1_d_string() {
        // Arrange
        let test_string = "ab".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        // Assert
        assert_eq!(result_string, test_string);
    }
    #[test]
    fn test_3_l_string() {
        // Arrange
        let test_string = "aaa".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        // Assert
        assert_eq!(result_string, "".to_string());
    }
    #[test]
    fn test_3_l_2_d_string() {
        // Arrange
        let test_string = "abc".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        // Assert
        assert_eq!(result_string, test_string);
    }
    #[test]
    fn test_3_l_1_d_string() {
        // Arrange
        let test_string = "aac".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        // Assert
        assert_eq!(result_string, "aca".to_string());
    }
    #[test]
    fn test_4_l_string() {
        // Arrange
        let test_string = "aaaa".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        // Assert
        assert_eq!(result_string, "".to_string());
    }
    #[test]
    fn test_4_l_1_d_string() {
        // Arrange
        let test_string = "aaac".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        // Assert
        assert_eq!(result_string, "".to_string());
    }
    #[test]
    fn test_4_l_2_d_string() {
        // Arrange
        let test_string = "aacc".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        println!("{result_string}");
        let mut good = true;
        let s = result_string.as_bytes().to_vec();
        for index in 0..(s.len() - 1) {
            if s[index] == s[index + 1] {
                good = false;
                break;
            }
        }
        // Assert
        assert!(good);
    }
    #[test]
    fn test_4_l_2_d_1_d_string() {
        // Arrange
        let test_string = "abcc".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        println!("{result_string}");
        let mut good = true;
        let s = result_string.as_bytes().to_vec();
        for index in 0..(s.len() - 1) {
            if s[index] == s[index + 1] {
                good = false;
                break;
            }
        }
        // Assert
        assert!(good);
    }
    #[test]
    fn test_4_l_4_d_string() {
        // Arrange
        let test_string = "abcd".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        println!("{result_string}");
        let mut good = true;
        let s = result_string.as_bytes().to_vec();
        for index in 0..(s.len() - 1) {
            if s[index] == s[index + 1] {
                good = false;
                break;
            }
        }
        // Assert
        assert!(good);
    }
    #[test]
    fn test_5_l_string() {
        // Arrange
        let test_string = "aaaaa".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        println!("{result_string}");
        // Assert
        assert_eq!(result_string, "".to_string());
    }
    #[test]
    fn test_5_l_1_d_string() {
        // Arrange
        let test_string = "aaaab".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        println!("{result_string}");
        // Assert
        assert_eq!(result_string, "".to_string());
    }
    #[test]
    fn test_5_l_2_d_string() {
        // Arrange
        let test_string = "abcabc".to_string();
        // Act
        let result_string = Solution::reorganize_string(test_string.clone());
        println!("Result string: {result_string}");
        let mut good = true;
        let s = result_string.as_bytes().to_vec();
        for index in 0..(s.len() - 1) {
            if s[index] == s[index + 1] {
                good = false;
                break;
            }
        }
        // Assert
        assert!(good);
    }
}
