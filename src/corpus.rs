use std::collections::HashSet;
use crate::book::Book::{self, *};

pub struct Corpus;
impl Corpus {
    pub fn bible() -> HashSet<Book> {
        let mut books = HashSet::new();
        books.insert(Genesis);
        books.insert(Exodus);
        books.insert(Leviticus);
        books.insert(Numbers);
        books.insert(Deuteronomy);
        books.insert(Joshua);
        books.insert(Judges);
        books.insert(Ruth);
        books.insert(FirstSamuel);
        books.insert(SecondSamuel);
        books.insert(FirstKings);
        books.insert(SecondKings);
        books.insert(FirstChronicles);
        books.insert(SecondChronicles);
        books.insert(Ezra);
        books.insert(Nehemiah);
        books.insert(Esther);
        books.insert(Job);
        books.insert(Psalms);
        books.insert(Proverbs);
        books.insert(Ecclesiastes);
        books.insert(SongOfSolomon);
        books.insert(Isaiah);
        books.insert(Jeremiah);
        books.insert(Lamentations);
        books.insert(Ezekiel);
        books.insert(Daniel);
        books.insert(Hosea);
        books.insert(Joel);
        books.insert(Amos);
        books.insert(Obadiah);
        books.insert(Jonah);
        books.insert(Micah);
        books.insert(Nahum);
        books.insert(Habakkuk);
        books.insert(Zephaniah);
        books.insert(Haggai);
        books.insert(Zechariah);
        books.insert(Malachi);
        books.insert(Matthew);
        books.insert(Mark);
        books.insert(Luke);
        books.insert(John);
        books.insert(Acts);
        books.insert(Romans);
        books.insert(FirstCorinthians);
        books.insert(SecondCorinthians);
        books.insert(Galatians);
        books.insert(Ephesians);
        books.insert(Philippians);
        books.insert(Colossians);
        books.insert(FirstThessalonians);
        books.insert(SecondThessalonians);
        books.insert(FirstTimothy);
        books.insert(SecondTimothy);
        books.insert(Titus);
        books.insert(Philemon);
        books.insert(Hebrews);
        books.insert(James);
        books.insert(FirstPeter);
        books.insert(SecondPeter);
        books.insert(FirstJohn);
        books.insert(SecondJohn);
        books.insert(ThirdJohn);
        books.insert(Jude);
        books.insert(Revelation);

        books
    }

    pub fn old_testament() -> HashSet<Book> {
        let mut books = HashSet::new();
        books.insert(Genesis);
        books.insert(Exodus);
        books.insert(Leviticus);
        books.insert(Numbers);
        books.insert(Deuteronomy);
        books.insert(Joshua);
        books.insert(Judges);
        books.insert(Ruth);
        books.insert(FirstSamuel);
        books.insert(SecondSamuel);
        books.insert(FirstKings);
        books.insert(SecondKings);
        books.insert(FirstChronicles);
        books.insert(SecondChronicles);
        books.insert(Ezra);
        books.insert(Nehemiah);
        books.insert(Esther);
        books.insert(Job);
        books.insert(Psalms);
        books.insert(Proverbs);
        books.insert(Ecclesiastes);
        books.insert(SongOfSolomon);
        books.insert(Isaiah);
        books.insert(Jeremiah);
        books.insert(Lamentations);
        books.insert(Ezekiel);
        books.insert(Daniel);
        books.insert(Hosea);
        books.insert(Joel);
        books.insert(Amos);
        books.insert(Obadiah);
        books.insert(Jonah);
        books.insert(Micah);
        books.insert(Nahum);
        books.insert(Habakkuk);
        books.insert(Zephaniah);
        books.insert(Haggai);
        books.insert(Zechariah);
        books.insert(Malachi);

        books
    }

    pub fn new_testament() -> HashSet<Book> {
        let mut books = HashSet::new();
        books.insert(Matthew);
        books.insert(Mark);
        books.insert(Luke);
        books.insert(John);
        books.insert(Acts);
        books.insert(Romans);
        books.insert(FirstCorinthians);
        books.insert(SecondCorinthians);
        books.insert(Galatians);
        books.insert(Ephesians);
        books.insert(Philippians);
        books.insert(Colossians);
        books.insert(FirstThessalonians);
        books.insert(SecondThessalonians);
        books.insert(FirstTimothy);
        books.insert(SecondTimothy);
        books.insert(Titus);
        books.insert(Philemon);
        books.insert(Hebrews);
        books.insert(James);
        books.insert(FirstPeter);
        books.insert(SecondPeter);
        books.insert(FirstJohn);
        books.insert(SecondJohn);
        books.insert(ThirdJohn);
        books.insert(Jude);
        books.insert(Revelation);

        books
    }
}