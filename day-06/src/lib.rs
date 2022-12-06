use std::collections::HashSet;

pub fn index_after_n_unique_characters(datastream: &[u8], n: usize) -> Option<usize> {
    let mut candidate = vec![0; n];
    for (i, group) in datastream.windows(n).enumerate() {
        candidate.copy_from_slice(group);
        candidate.sort();
        if candidate.windows(2).all(|chars| chars[0] != chars[1]) {
            return Some(i + n);
        }
    }

    None
}

pub fn index_after_n_unique_characters_hashset(datastream: &[u8], n: usize) -> Option<usize> {
    for (i, group) in datastream.windows(n).enumerate() {
        let unique_items: HashSet<u8> = HashSet::from_iter(group.iter().cloned());
        if unique_items.len() == n {
            return Some(i + n);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_after_start_of_packet() {
        let test_data = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, output) in test_data {
            assert_eq!(
                index_after_n_unique_characters(input.as_bytes(), 4),
                Some(output)
            );
            assert_eq!(
                index_after_n_unique_characters_hashset(input.as_bytes(), 4),
                Some(output)
            );
        }
    }

    #[test]
    fn test_index_after_start_of_message() {
        let test_data = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, output) in test_data {
            assert_eq!(
                index_after_n_unique_characters(input.as_bytes(), 14),
                Some(output)
            );
            assert_eq!(
                index_after_n_unique_characters_hashset(input.as_bytes(), 14),
                Some(output)
            );
        }
    }
}
