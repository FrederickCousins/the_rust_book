use std::collections::VecDeque;

fn starts_with_vowel(s: &str) -> bool {
    if let Some(first_char) = s.to_lowercase().chars().next() {
        matches!(first_char, 'a' | 'e' | 'i' | 'o' | 'u')
    } else {
        unreachable!(); // Assert that the string is non-empty
    }
}

pub fn pig_latin(s: &str) -> Option<String> {
    if s.is_empty() {
        return None;
    }

    let mut v: VecDeque<char> = s.chars().collect();
    let mut res = String::new();

    if starts_with_vowel(s) {
        Some(format!("{}-hay", s))
    } else if let Some(cons) = v.pop_front() {
        res.push_str(&v.iter().collect::<String>());
        res.push('-');
        res.push(cons);
        res.push_str("ay");
        Some(res)
    } else {
        unreachable!()
    }
}
