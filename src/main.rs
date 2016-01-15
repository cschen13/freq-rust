#[doc="
Counts the frequencies of words read from the standard input, and print
a sorted frequency table.

Assumptions:
-Entries in the frequency table are case-insensitive (or rather, all
 converted to lowercase).

-Numbers are words.

-Words with apostrophes are only words if the apostrophe appears as the
 second-to-last character (to cover contractions). If a contraction
 appears in the beginning or end of the word, it is removed and the 
 remainder of the word is added to the frequency table.

-'a' and 'i' are the only one-letter words that will be recorded.
 Effectively, this means acronymns that are denoted with periods 
 between each letter (U.S.A.) are also not recorded.
"]
use std::io::{BufRead, BufReader, Read, stdin};

fn main() {
    let mut map = CountTable::new();
    read_words(stdin(), &mut map);

    let mut sorted_vec: Vec<_> = map.iter().collect();
    sorted_vec.sort_by(|a, b| b.1.cmp(a.1));

    for word in sorted_vec.iter() {
        println!("{}: {}", word.0, word.1);
    }
}

//Used a BTreeMap instead of HashMap so that words would be sorted
//alphabetically at each frequency.
type CountTable = std::collections::BTreeMap<String, usize>;

fn read_words<R: Read>(reader: R, mut map: &mut CountTable) {
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
        // if let Ok(unclean_line) = line.parse::<String>() {
        //     let words: Vec<&str> = unclean_line.split(' ').collect();
        if let Ok(unclean_line) = line.parse::<String>() {
            //Initial "Filter": Separate the line into string slices by non-alphanumeric characters that AREN'T apostrophes.
            let words: Vec<&str> = unclean_line.splitn(unclean_line.len() + 1, |c: char| !(c.is_alphanumeric()) && c != '\'').collect();

            for mut word in words {
                match clean_word(&mut word) {
                    Some(cleaned_word) => {
                        increment_word(map, String::from(cleaned_word).to_lowercase());
                    }
                    None => {
                        continue;
                    }
                }
            }
        }
    }
}

fn clean_word(mut word: &str) -> Option<&str> {
    if word.is_empty() {
        None
    }
    else {
        // //word = word.to_lowercase();
        // let chars: Vec<_> = word.char_indices().collect();
        // //if chars.len() == 1 && chars[0].0 != 'a' && chars[0].0 != 'i' { None }

        // let firstChar = false;
        // let lastChar = false;
        // let badApostrophe = false;
        // let lastCharPosn = word.len() - 1;
        // let secondLastCharPosn = word.len() - 2;
         
        // for char in chars {
        //     if char.1 == '\'' {
        //         match char.0 {
        //             0 => { firstChar = true; },
        //             secondLastCharPosn => { continue; },
        //             lastCharPosn => {lastChar = true;},
        //             _ => badApostrophe = true, //bad apostrophe
        //         }
        //     }
        // }

        // if firstChar { word = word.split_at_mut(chars[1].0).1; }
        // // if lastChar { (,word) = word.}

        Some(word)
    }
}

fn increment_word(mut map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}


#[cfg(test)]
mod read_words_tests {
    use super::{CountTable, read_words};
    use std::io::{Read, Result};

    #[test]
    fn one_word_per_line() {
        let input = StringReader::new("Hello  &&\nWorld".to_owned());
        let mut under_test = CountTable::new();
        read_words(input, &mut under_test);

        let mut expected = CountTable::new();
        expected.insert("hello".to_owned(), 1);
        expected.insert("world".to_owned(), 1);

        assert_eq!(expected, under_test);
    }

    struct StringReader {
        contents: Vec<u8>,
        position: usize,
    }
    
    impl StringReader {
        fn new(s: String) -> Self {
            StringReader {
                contents: s.into_bytes(),
                position: 0,
            }
        }
    }

    impl Read for StringReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count = 0;

            while self.position < self.contents.len() && count < buf.len() {
                buf[count] = self.contents[self.position];
                count += 1;
                self.position += 1;
            }
            return Ok(count);
        }
    }
}

#[cfg(test)]
mod increment_word_tests {
    use super::{increment_word, CountTable};

    #[test]
    fn inserts_if_absent() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "one".to_owned());
        expected.insert("one".to_owned(), 1);

        assert_eq!(expected, under_test);
    }

    #[test]
    fn increments_if_present() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "two".to_owned());
        expected.insert("two".to_owned(), 3);

        assert_eq!(expected, under_test);
    }

    #[test]
    fn inserts_if_empty() {
        let mut under_test = CountTable::new();
        let mut expected   = CountTable::new();

        increment_word(&mut under_test, "one".to_owned());
        expected.insert("one".to_owned(), 1);

        assert_eq!(expected, under_test);
    }


    fn fixture() -> CountTable {
        let mut h = CountTable::new();
        h.insert("two".to_owned(), 2);
        h.insert("three".to_owned(), 3);

        assert_eq!(None, h.get("one"));
        assert_eq!(Some(&2), h.get("two"));
        assert_eq!(Some(&3), h.get("three"));
        assert_eq!(2, h.len());

        return h;
    }
}

