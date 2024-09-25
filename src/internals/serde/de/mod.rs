use std::str::Split;

use bon::bon;

#[derive(Debug, Clone)]
pub struct IndexedDeserializer<'de> {
    sequence: bool,
    splitter: Split<'de, &'de str>,
    source: &'de str,
    delimiter: &'de str,
}

#[bon]
impl<'de> IndexedDeserializer<'de> {
    #[builder]
    pub fn new(source: &'de str, delimiter: &'de str, sequence: bool) -> Self {
        let splitter = source.split(delimiter);

        Self {
            sequence,
            splitter,
            source,
            delimiter,
        }
    }
}
