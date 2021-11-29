// 题目链接：https://exercism.org/tracks/rust/exercises/parallel-letter-frequency

use std::thread;

fn parallel_letter_frequency(cnt: &mut Vec<u32>, text: &String) {
    // creat the letter table
    let mut letter_table: Vec<char> = Vec::new();
    for element in 'a'..='z' {
        letter_table.push(element);
    }
    for element in 'A'..='Z' {
        letter_table.push(element);
    }

    let mut thread_pool = Vec::with_capacity(52);
    for letter in letter_table {
        let text_ = text.chars().by_ref().collect::<String>();
        let thread_single = thread::spawn(move || {
            let mut counter: u32 = 0;
            for element in text_.chars() {
                if element == letter{
                    counter = counter + 1;
                }
            }
            // find the index of the letter
            let mut index: usize = (letter as usize) - ('A' as usize);
            if index > 26 {
                index = (letter as usize) - ('a' as usize) + 26;
            }
            let pack: (usize, u32) = (index, counter);

            // return pack
            pack
        });
        thread_pool.push(thread_single);
    }

    for thread_single in thread_pool {
        let recv = thread_single.join().unwrap();
        cnt[recv.0] = recv.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_letter_frequency_test() {
        let mut cnt: Vec<u32> = vec![0; 52];
        let text: String = String::from("this is a message for test");
        parallel_letter_frequency(&mut cnt, &text);
        let result: Vec<u32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 1, 1, 1, 2, 0, 0, 0, 1, 0, 1, 0, 0, 1, 5, 3, 0, 0, 0, 0, 0, 0];
        assert_eq!(result, cnt);
    }
}