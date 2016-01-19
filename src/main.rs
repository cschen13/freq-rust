#[doc="
Counts the frequencies of words read from the standard input, and print
a sorted frequency table.

Assumptions:
-Entries in the frequency table are case-insensitive (or rather, all
 converted to lowercase).

-The program has only been tested properly with languages that 
 distinguish words with spaces or other non-alphanumeric characters
 that happen to be between them. For instance, this program won't 
 work when testing with Chinese characters... words are not separated
 with spaces, but they also aren't necessarily one character...

-Numbers are not words. If they are sandwiched between alphabetic
 characters, they are considered spaces between the two words
 surrounding them.

-Words with apostrophes are only words if the apostrophe appears as the
 second-to-last character (to cover contractions and most singular 
 possessives in English). If an apostrophe appears in the beginning or
 end of the word, it is removed and the remainder of the word is added
 to the frequency table. Uncleaned words with apostrophes in any 
 other location are not considered words.

 EXAMPLES: All of the following words map to jesse.
    Jesse
    'jesse
    'JESSE'

-'a' and 'i' are the only one-letter words that will be intentionally
 recorded. Effectively, this means acronymns that are denoted with 
 periods between each letter (U.S.A.) are also NOT recorded. One way 
 around this assumption is if there are apostrophes around the one 
 non-'a' or non-'i' letter (so, 'w will be mapped to w, for instance).
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
        if let Ok(unclean_line) = line.parse::<String>() {
            //Initial "Filter": Separate the line into string slices by
            //non-alphanumeric characters that AREN'T apostrophes.
            let words: Vec<&str> = unclean_line.splitn(unclean_line.len() + 1, |c: char| !(c.is_alphabetic()) && c != '\'').collect();

            for word in words {
                match clean_word(word) {
                    Some(cleaned_word) => {
                        increment_word(map, String::from(cleaned_word)
                            .to_lowercase());
                    }
                    None => {
                        continue;
                    }
                }
            }
        }
    }
}

fn clean_word(word: &str) -> Option<&str> {
    if word.is_empty() {
        None
    }
    else {
        let chars: Vec<_> = word.char_indices().collect();
        let char_count = chars.len();

        //if word is one character long, it had better be an 'a' or 'i'
        if char_count == 1 {
            if chars[0].1.to_lowercase().next() != Some('a') && chars[0].1.to_lowercase().next() != Some('i') { return None } //FIX THIS
            else {return Some(word)}
        }

        //apostrophe flags
        let mut first_char = false;
        let mut last_char = false;

        let last_char_posn = chars[char_count - 1].0;
        let second_last_char_posn = chars[char_count - 2].0;
         
        //apostrophe check that triggers flags
        for char in chars {
            if char.1 == '\'' {
                if      char.0 == 0                     { first_char = true; }
                else if char.0 == second_last_char_posn { continue; }
                else if char.0 == last_char_posn        { last_char = true; }
                //else case covers when apostrophe is in a place where it shouldn't be
                else                                    { return None}


                //This match below kept giving me "unreachable" errors,
                //so I just went with the if-statements.
                // match char.0 {
                //     0 => { firstChar = true; },
                //     secondLastCharPosn => { continue; },
                //     lastCharPosn => {lastChar = true;},
                //     _ => {return None}, //bad apostrophe
                // }
            }
        }

        //Not sure why I have to call the iterator again,
        //but I can't compile without creating char_indices, so...
        let char_indices: Vec<_> = word.char_indices().collect(); 
        let mut cleaned_word = word.clone();

        //Handling the apostrophe flags here...
        if first_char && last_char { 
            cleaned_word = cleaned_word.split_at(char_indices[last_char_posn].0).0;
            cleaned_word = cleaned_word.split_at(char_indices[1].0).1;
            if cleaned_word.len() == 0 {
                return None //Edge case where word is simply two apostrophes
            }
        }
        else if first_char {
            cleaned_word = cleaned_word.split_at(char_indices[1].0).1;
        }
        else if last_char {
            cleaned_word = cleaned_word.split_at(char_indices[last_char_posn].0).0;
        }

        //...and we should be clean!
        Some(cleaned_word)
    }
}

fn increment_word(mut map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}


#[cfg(test)]
//This module also tests clean_word functionality, since read_words
//depends on clean_word to handle apostrophes and single-letter words
//correctly.
mod read_words_tests {
    use super::{CountTable, read_words};
    use std::io::{Read, Result};

    #[test]
    fn one_word_per_line() {
        let input = StringReader::new("Hello\nWorld".to_owned());
        let mut under_test = CountTable::new();
        read_words(input, &mut under_test);

        let mut expected = CountTable::new();
        expected.insert("hello".to_owned(), 1);
        expected.insert("world".to_owned(), 1);

        assert_eq!(expected, under_test);
    }

    #[test]
    fn non_alphabetic() {
        let input = StringReader::new(".....&&*(*&( \n    %$#@Ok!!43424!".to_owned());
        let mut under_test = CountTable::new();
        read_words(input, &mut under_test);

        let mut expected = CountTable::new();
        //Notice that Ok counts as a word, because non-alphabetic chars
        //are simply seen as "spaces" or dividers between uncleaned words.
        expected.insert("ok".to_owned(), 1); 
        assert_eq!(expected, under_test);
    }

    #[test]
    fn apostrophes() {
        let input = StringReader::new("Jesse 'jesse' 'jesse JESSE' '' ''Jesse".to_owned());
        let mut under_test = CountTable::new();
        read_words(input, &mut under_test);

        let mut expected = CountTable::new();
        //Notice the last ''Jesse will not map to a word because
        //there is an apostrophe that is not in the first, last, or
        //second-to-last position in the word during cleaning.
        expected.insert("jesse".to_owned(), 4);

        assert_eq!(expected, under_test);
    }

    #[test]
    fn acronymns() {
        let input = StringReader::new("U.S.A.".to_owned());
        let mut under_test = CountTable::new();
        read_words(input, &mut under_test);

        let mut expected = CountTable::new();
        expected.insert("a".to_owned(), 1);

        assert_eq!(expected, under_test);
    }

    #[test]
    fn one_letter_words() {
        let input = StringReader::new("a\ne\ni\n'o\n'u'".to_owned());
        let mut under_test = CountTable::new();
        read_words(input, &mut under_test);

        let mut expected = CountTable::new();
        expected.insert("a".to_owned(), 1);
        expected.insert("i".to_owned(), 1);
        expected.insert("o".to_owned(), 1);
        expected.insert("u".to_owned(), 1);
        //Notice that the letter e will not be inserted,
        //but the words with a single-letter and an 
        //apostrophe or two WILL be written in (like 'o and 'u').

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

