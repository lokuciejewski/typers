pub mod wiki_source;
pub mod file_source;

pub trait Sourceable {
    fn get_new_sentence(&self) -> Result<String, String>;
}
