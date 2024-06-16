use std::path::PathBuf;

#[derive(Default)]
pub struct FileDeclutter {
    source_path: PathBuf,
    levels: usize,
    remove_empty_directories: bool,
}

impl FileDeclutter {
    pub fn new(source_path: impl Into<PathBuf>) -> Self {
        let source_path = source_path.into();
        Self {
            source_path,
            ..Default::default()
        }
    }

    pub fn levels(mut self, levels: usize) -> Self {
        self.levels = levels;
        self
    }

    pub fn remove_empty_directories(mut self, remove_empty_directories: bool) -> Self {
        self.remove_empty_directories = remove_empty_directories;
        self
    }

    pub fn create_iter(&self) -> impl Iterator<Item = (PathBuf, PathBuf)> + '_ {
        walkdir::WalkDir::new(&self.source_path)
            .min_depth(1)
            .into_iter()
            .flatten()
            .filter(|f| f.file_type().is_file())
            .map(|entry| {
                let sub_dirs = entry.file_name().to_string_lossy();
                let sub_dirs = sub_dirs.chars().take(self.levels).map(String::from);

                let mut target_path = self.source_path.clone();
                for sub_dir in sub_dirs {
                    target_path.push(sub_dir);
                }
                target_path.push(entry.file_name());

                (entry.into_path(), target_path)
            })
    }

    pub fn declutter_files(&self) -> anyhow::Result<()> {
        for (source, target) in self.create_iter() {
            std::fs::create_dir_all(&target.parent().unwrap())?;
            std::fs::rename(source, target)?;
        }

        if self.remove_empty_directories {
            for dir in walkdir::WalkDir::new(&self.source_path)
                .min_depth(1)
                .contents_first(true)
                .into_iter()
                .filter_entry(|f| f.file_type().is_dir())
                .flatten()
            {
                let dir = dir.into_path();

                if dir.read_dir()?.count() == 0 {
                    // Ignore result, we don't care if we actually deleted something here.
                    let _ = std::fs::remove_dir(dir);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use rand::Rng;

    use super::*;

    #[test]
    fn decluttered_file_names_same() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;

        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let mut file_name = rng
                .gen_range(1_000_000_000u64..10_000_000_000u64)
                .to_string();

            if rng.gen_bool(0.25) {
                file_name = format!("subdir/{file_name}");
            }

            let child = temp_dir.child(file_name);
            child.touch()?;
        }

        for (source, target) in FileDeclutter::new(temp_dir.to_path_buf())
            .levels(1)
            .create_iter()
        {
            assert_eq!(source.file_name(), target.file_name());
        }

        Ok(())
    }
}
