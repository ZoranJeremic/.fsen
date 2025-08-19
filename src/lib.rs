use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub format: NoteFormat,          // txt, md, fsen
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NoteFormat {
    Txt,
    Md,
    Fsen,
}

impl Note {
    pub fn new(title: &str, content: &str, format: NoteFormat) -> Self {
        let now = Utc::now();
        Note {
            id: Uuid::new_v4(),
            title: title.to_string(),
            content: content.to_string(),
            format,
            created_at: now,
            updated_at: now,
        }
    }
}


pub fn save_note_json(note: &Note) -> String {
    serde_json::to_string_pretty(note).unwrap()
}

pub fn load_note_json(data: &str) -> Note {
    serde_json::from_str(data).unwrap()
}


pub fn save_note_bin(note: &Note) -> Vec<u8> {
    bincode::serialize(note).unwrap()
}

pub fn load_note_bin(bytes: &[u8]) -> Note {
    bincode::deserialize(bytes).unwrap()
  }
