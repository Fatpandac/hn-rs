use std::{
    collections::{BTreeSet, HashMap},
    io::Result,
};

use hackernews::StoryType;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use uuid::Uuid;

use crate::storages::save_data::SaveData;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ReadHistoryData {
    #[serde(flatten)]
    pub data: HashMap<StoryType, BTreeSet<usize>>,
}

#[derive(Debug)]
pub struct ReadHistory {
    storage: SaveData<ReadHistoryData>,
    max: usize,
}

impl ReadHistory {
    pub fn new(max: usize) -> Self {
        #[cfg(test)]
        let file_name = format!("read_history_test_{}", Uuid::new_v4());
        #[cfg(not(test))]
        let file_name = "read_history".to_string();
        let storage = SaveData::new(
            file_name,
            ReadHistoryData {
                data: HashMap::new(),
            },
        );
        ReadHistory { storage, max }
    }

    pub fn add_read_item(&mut self, story_type: StoryType, item_id: usize) -> Result<()> {
        self.storage.load()?;
        let history = &mut self.storage.data.data;
        let items = history.entry(story_type).or_default();
        println!("{:?}", items);
        if items.len() >= self.max {
            if let Some(oldest) = items.iter().next().cloned() {
                println!("Removing oldest item: {}", oldest);
                items.remove(&oldest);
            }
        }
        items.insert(item_id);
        self.storage.save()
    }

    pub fn id_is_readed(&self, story_type: StoryType, item_id: usize) -> bool {
        self.storage
            .data
            .data
            .get(&story_type)
            .is_some_and(|ids| ids.contains(&item_id))
    }

    #[cfg(test)]
    pub fn remove(&mut self) -> Result<()> {
        self.storage.remove()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hackernews::StoryType;

    #[test]
    fn test_add_read_item() {
        let mut history = ReadHistory::new(100);
        history.add_read_item(StoryType::Show, 42).unwrap();
        assert!(history.id_is_readed(StoryType::Show, 42));

        history.remove().unwrap();
    }

    #[test]
    fn test_id_is_not_readed() {
        let mut history = ReadHistory::new(100);
        assert!(!history.id_is_readed(StoryType::Show, 100));

        history.remove().unwrap();
    }

    #[test]
    fn test_add_read_item_exceed_max() {
        let mut history = ReadHistory::new(2);
        history.add_read_item(StoryType::Show, 1).unwrap();
        history.add_read_item(StoryType::Show, 2).unwrap();
        history.add_read_item(StoryType::Show, 3).unwrap();

        println!("{:?}", history.storage.data.data);
        assert!(!history.id_is_readed(StoryType::Show, 1));
        assert!(history.id_is_readed(StoryType::Show, 2));
        assert!(history.id_is_readed(StoryType::Show, 3));

        history.remove().unwrap();
    }
}
