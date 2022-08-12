use std::collections::HashSet;
use crate::bible_ref::BibleRef;
use crate::bible_ref_utils::num_verses_in_chapter;
use crate::book::Book;

/// Represents a particular Bible version, such as the ESV, NA28, RVR, etc.
pub struct BibleVersion {
    /// A unique identifier for this version (e.g. "esv", "kjv", etc).
    id: String,

    /// The set of books included in this version of the Bible. For example, a Greek New Testament
    /// will only contain Matthew-Revelation.
    corpus: HashSet<Book>,

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
}