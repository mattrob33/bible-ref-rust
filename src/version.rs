use std::collections::HashSet;
use crate::bible_ref::BibleRef;
use crate::bible_ref_utils::num_verses_in_chapter;
use crate::book::Book::{self, *};
use crate::corpus::Corpus;

/// Represents a particular Bible version, such as the ESV, NA28, RVR, etc.
pub struct BibleVersion {
    /// A unique identifier for this version (e.g. "esv", "kjv", etc).
    id: String,

    /// The set of books included in this version of the Bible. For example, a Greek New Testament
    /// will only contain Matthew-Revelation.
    corpus: Vec<Book>,

    /// Verses that are excluded from this version of the Bible.
    excluded_verses: HashSet<BibleRef>
}

impl BibleVersion {

    /// Returns true if this version contains the specified book, or false otherwise. This
    /// would return false for example for Genesis in a Greek New Testament.
    pub fn contains_book(&self, book: &Book) -> bool {
        self.corpus.contains(&book)
    }

    /// Returns true if this
    pub fn excludes_verse(&self, verse: &BibleRef) -> bool {
        self.excluded_verses.contains(&verse)
    }

    /// Returns the number of verses in the specified chapter for this version, accounting for
    /// the specific verses that some Bible versions omit. Returns `None` if the version does
    /// not even include this book, such as calling this for Genesis in a Greek New Testament.
    pub fn number_of_verses_in_chapter(&self, book: &Book, chapter: i32) -> Option<i32> {
        if !self.contains_book(book) { return None }
        let mut num_verses = num_verses_in_chapter(book, chapter);

        for verse in self.excluded_verses.iter() {
            if verse.book == *book && verse.chapter == chapter {
                num_verses -= 1
            }
        }

        Some(num_verses)
    }

    /// Create a new `BibleVersion` based on the Textus Receptus textual base.
    pub fn textus_receptus_bible(id: &str) -> BibleVersion {
        BibleVersion {
            id: String::from(id),
            corpus: Corpus::bible(),
            excluded_verses: HashSet::new()
        }
    }

    /// Create a new `BibleVersion` based on the more modern textual base which omits
    /// several verses included in the kjv / Textus Receptus.
    pub fn modern_bible(id: &str) -> BibleVersion {
        BibleVersion {
            id: String::from(id),
            corpus: Corpus::bible(),
            excluded_verses: HashSet::from([
                BibleRef { book: Matthew, chapter: 12, verse: 47 },
                BibleRef { book: Matthew, chapter: 17, verse: 21 },
                BibleRef { book: Matthew, chapter: 18, verse: 11 },
                BibleRef { book: Matthew, chapter: 23, verse: 14 },
                BibleRef { book: Mark, chapter: 7, verse: 16 },
                BibleRef { book: Mark, chapter: 9, verse: 44 },
                BibleRef { book: Mark, chapter: 9, verse: 46 },
                BibleRef { book: Mark, chapter: 11, verse: 26 },
                BibleRef { book: Mark, chapter: 15, verse: 28 },
                BibleRef { book: Luke, chapter: 17, verse: 36 },
                BibleRef { book: Luke, chapter: 22, verse: 44 },
                BibleRef { book: Luke, chapter: 23, verse: 17 },
                BibleRef { book: John, chapter: 5, verse: 4 },
                BibleRef { book: Acts, chapter: 8, verse: 37 },
                BibleRef { book: Acts, chapter: 15, verse: 34 },
                BibleRef { book: Acts, chapter: 24, verse: 7 },
                BibleRef { book: Acts, chapter: 28, verse: 29 },
                BibleRef { book: Romans, chapter: 16, verse: 24 },
                BibleRef { book: FirstJohn, chapter: 5, verse: 7 },
            ])
        }
    }
}

impl BibleVersion {
    pub fn kjv() -> BibleVersion { BibleVersion::textus_receptus_bible("KJV") }
    pub fn nkjv() -> BibleVersion { BibleVersion::textus_receptus_bible("NKJV") }

    pub fn esv() -> BibleVersion { BibleVersion::modern_bible("ESV") }
    pub fn niv() -> BibleVersion { BibleVersion::modern_bible("NIV") }
    pub fn nlt() -> BibleVersion { BibleVersion::modern_bible("NLT") }
    pub fn csb() -> BibleVersion { BibleVersion::modern_bible("CSB") }
    pub fn nasb() -> BibleVersion { BibleVersion::modern_bible("NASB") }

    pub fn na28() -> BibleVersion {
        BibleVersion {
            id: "NA28".to_string(),
            corpus: Corpus::new_testament(),
            excluded_verses: HashSet::new()
        }
    }

    pub fn thgnt() -> BibleVersion {
        BibleVersion {
            id: "THGNT".to_string(),
            corpus: {
                vec![
                    Matthew,
                    Mark,
                    Luke,
                    John,
                    Acts,
                    James,
                    FirstPeter,
                    SecondPeter,
                    FirstJohn,
                    SecondJohn,
                    ThirdJohn,
                    Jude,
                    Romans,
                    FirstCorinthians,
                    SecondCorinthians,
                    Galatians,
                    Ephesians,
                    Philippians,
                    Colossians,
                    FirstThessalonians,
                    SecondThessalonians,
                    FirstTimothy,
                    SecondTimothy,
                    Titus,
                    Philemon,
                    Hebrews,
                    Revelation
                ]
            },
            excluded_verses: HashSet::new()
        }
    }
}

