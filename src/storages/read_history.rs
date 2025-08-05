use std::{
    collections::{HashMap, HashSet},
    io::Result,
};

use hackernews::StoryType;
use serde::{Deserialize, Serialize};

use crate::storages::save_data::SaveData;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ReadHistoryData {
    #[serde(flatten)]
    pub data: HashMap<StoryType, HashSet<usize>>,
}

#[derive(Debug)]
pub struct ReadHistory {
    storage: SaveData<ReadHistoryData>,
}

impl ReadHistory {
    pub fn new() -> Self {
        #[cfg(test)]
        let file_name = "read_history_test".to_string();
        #[cfg(not(test))]
        let file_name = "read_history".to_string();
        let storage = SaveData::new(
            file_name,
            ReadHistoryData {
                data: HashMap::new(),
            },
        );
        ReadHistory { storage }
    }

    pub fn add_read_item(&mut self, story_type: StoryType, item_id: usize) -> Result<()> {
        self.storage.load()?;
        let history = &mut self.storage.data.data;
        history.entry(story_type).or_default().insert(item_id);
        self.storage.save()
    }

    pub fn id_is_readed(&self, story_type: StoryType, item_id: usize) -> bool {
        self.storage
            .data
            .data
            .get(&story_type)
            .is_some_and(|ids| ids.contains(&item_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hackernews::StoryType;

    fn clean_test_file() {
        let base_dirs = directories::BaseDirs::new().unwrap();
        let cache_dir = base_dirs.cache_dir();
        let file_path = cache_dir.join("./hn-rs/read_history_test.json");
        if file_path.exists() {
            std::fs::remove_file(file_path).expect("Failed to remove test file");
        }
    }

    #[test]
    fn test_add_read_item() {
        let mut history = ReadHistory::new();
        history.add_read_item(StoryType::Show, 42).unwrap();
        assert!(history.id_is_readed(StoryType::Show, 42));

        clean_test_file();
    }

    #[test]
    fn test_id_is_not_readed() {
        let history = ReadHistory::new();
        assert!(!history.id_is_readed(StoryType::Show, 100));

        clean_test_file();
    }
}
