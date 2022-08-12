mod words;
use std::collections::HashSet;
use std::borrow::Borrow;
use std::thread;

fn remove_anagrams<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let dedupe: HashSet<String> = HashSet::new();
    let mut new_words: Vec<&'a str> = Vec::new();

    for word in words {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();

        let found_dupe = chars.windows(2).into_iter()
            .fold(false, |p, i| {
                match i {
                    [a,b] => p || a == b,
                    [..] => p,
                }
            });

        if found_dupe {
            continue
        }
        let sorted = String::from_iter(chars.into_iter());
        if dedupe.contains(sorted.as_str()) {
            continue
        }
        new_words.push(word)
    }
    return new_words
}

fn filter_words<'a>(word: &str, remaining_words: &Vec<&'a str>) -> Vec<&'a str> {
    let a: HashSet<char> = HashSet::from_iter(word.chars().into_iter());
    let mut new_words: Vec<&'a str> = Vec::new();

    for word in remaining_words {
        let mut found_match = false;
        for l in word.chars() {
            if a.contains(l.borrow()) {
                found_match = true;
            }
        }

        if !found_match {
            new_words.push(word)
        }
    }
    return new_words
}

fn recurse(current_list: Vec<&str>, remaining_words: Vec<&str>) {
    if current_list.len() > 4 {
        println!("{:?}", current_list)
    }

    let mut i: usize = 0;
    while remaining_words.len() > i {
        let word = remaining_words[i];

        if current_list.len() == 0 {
            println!("{}", i);
        }
        let new_remaining_words = filter_words(word, &remaining_words);

        let mut new_current_list: Vec<&str> = current_list.clone();
        new_current_list.push(word);

        recurse(new_current_list, new_remaining_words);
        i += 1;
    }
}

fn main() {
    let all_words = words::ALL_WORDS;
    let words = remove_anagrams(all_words);
    println!("{}", words.len());
    recurse(vec![], words)
}
