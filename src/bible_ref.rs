use std::cmp::Ordering::{self, Equal, Greater, Less};
use crate::book::Book;

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct BibleRef {
    pub book: Book,
    pub chapter: i32,
    pub verse: i32
}

impl Ord for BibleRef {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.book == other.book {
            if self.chapter == other.chapter {
                if self.verse == other.verse {
                    Equal
                }
                else {
                    if self.verse > other.verse { Greater } else { Less }
                }
            }
            else {
                if self.chapter > other.chapter { Greater } else { Less }
            }
        }
        else {
            if self.book > other.book { Greater } else { Less }
        }
    }
}

impl PartialOrd for BibleRef {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

pub struct BibleRefRange {
    start: BibleRef,
    end: BibleRef
}