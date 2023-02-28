use typers::{WikipediaSource, Sentence};

fn main() {
    let wikisource = WikipediaSource::default();
    let mut sentence = Sentence::new(wikisource);
    sentence.type_sentence();
}
