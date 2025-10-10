use directories::BaseDirs;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub struct SaveData<T> {
    path: String,
    pub data: T,
}

impl<T> SaveData<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn new(file_name: String, init_data: T) -> Self {
        let base_dirs = BaseDirs::new().unwrap();
        let cache_dir = base_dirs.cache_dir();
        let file_path = cache_dir.join(format!("./hn-rs/{}.json", file_name));
        let path = file_path.display().to_string();
        if !file_path.exists() {
            std::fs::create_dir_all(file_path.parent().unwrap())
                .expect("Failed to create directory");
            std::fs::File::create(&file_path).expect("Failed to create file");
            SaveData {
                path,
                data: init_data,
            }
        } else {
            let mut save_data = SaveData {
                path,
                data: init_data,
            };
            save_data.load().expect("Failed to load data");
            save_data
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let file = std::fs::File::create(&self.path)?;
        serde_json::to_writer(file, &self.data)?;
        Ok(())
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        let file = std::fs::File::open(&self.path)?;
        self.data = serde_json::from_reader(file).unwrap_or_else(|_| self.data.clone());
        Ok(())
    }

    #[cfg(test)]
    pub fn remove(&mut self) -> std::io::Result<()> {
        if std::fs::remove_file(&self.path).is_err() {
            eprintln!("Failed to remove file: {}", self.path);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};
    use std::error::Error;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct TestData {
        value: String,
    }

    #[test]
    fn test_save_and_load() -> Result<(), Box<dyn Error>> {
        let data = TestData {
            value: "Hello, World!".to_string(),
        };
        let mut save_data = SaveData::new("test_data".to_string(), data.clone());

        save_data.save()?;

        let mut loaded_data = SaveData::new(
            "test_data".to_string(),
            TestData {
                value: "".to_string(),
            },
        );
        loaded_data.load()?;

        assert_eq!(loaded_data.data, data);

        save_data.remove()?;
        Ok(())
    }
}
