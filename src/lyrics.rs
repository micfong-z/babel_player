use chrono::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct TranslationEntry {
    pub language: String,
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Agent {
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LyricsMetadata {
    pub agents: Vec<Agent>,
    pub translations: Vec<TranslationEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Lyrics {
    pub lines: Vec<LyricsLine>,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct LyricsSegment {
    #[serde_as(as = "serde_with::DurationMilliSeconds<i64>")]
    pub begin: Duration,
    #[serde_as(as = "serde_with::DurationMilliSeconds<i64>")]
    pub end: Duration,
    pub text: String,

    /// A list of translations for this segment.
    /// The translation of each language is a pair `(language_id, word_index_list)`.
    ///
    /// The word index list contains indices of the translated words in, which will be associated with the segment.
    pub translations: Vec<(Uuid, Vec<usize>)>,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct LyricsLine {
    #[serde_as(as = "serde_with::DurationMilliSeconds<i64>")]
    pub begin: Duration,
    #[serde_as(as = "serde_with::DurationMilliSeconds<i64>")]
    pub end: Duration,
    pub agent_id: String,
    pub original: Vec<LyricsSegment>,
    pub uuid: Uuid,

    /// A list of translations for this line of each language.
    /// The translation of each language is a pair `(language_id, word_list)`.
    pub translations: Vec<(Uuid, Vec<String>)>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BabelLyrics {
    pub metadata: LyricsMetadata,
    pub lyrics: Lyrics,
}
