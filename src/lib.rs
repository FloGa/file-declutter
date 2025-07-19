use std::path::PathBuf;

pub struct FileDeclutterIterator<I> {
    inner: I,
    base: PathBuf,
    levels: usize,
}

impl<I> FileDeclutterIterator<I>
where
    I: Iterator<Item = PathBuf>,
{
    pub fn declutter_files(self, remove_empty_directories: bool) -> anyhow::Result<()> {
        let base = self.base.clone();

        for (source, target) in self {
            std::fs::create_dir_all(&target.parent().unwrap())?;
            std::fs::rename(source, target)?;
        }

        if remove_empty_directories {
            for dir in walkdir::WalkDir::new(base)
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

impl<I> Iterator for FileDeclutterIterator<I>
where
    I: Iterator<Item = PathBuf>,
{
    type Item = (PathBuf, PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(move |entry| {
            let sub_dirs = entry.file_name().unwrap().to_string_lossy();
            let sub_dirs = sub_dirs.chars().take(self.levels).map(String::from);

            let mut target_path = self.base.clone();
            for sub_dir in sub_dirs {
                target_path.push(sub_dir);
            }
            target_path.push(entry.file_name().unwrap());

            (entry, target_path)
        })
    }
}

pub struct FileDeclutter;

impl FileDeclutter {
    pub fn new_from_iter(
        base: impl Into<PathBuf>,
        iter: impl Iterator<Item = PathBuf>,
        levels: usize,
    ) -> FileDeclutterIterator<impl Iterator<Item = PathBuf>> {
        FileDeclutterIterator {
            inner: iter,
            base: base.into(),
            levels,
        }
    }

    pub fn new_from_path(
        base: impl Into<PathBuf>,
        levels: usize,
    ) -> FileDeclutterIterator<impl Iterator<Item = PathBuf>> {
        let base = base.into();

        let iter = walkdir::WalkDir::new(&base)
            .min_depth(1)
            .into_iter()
            .flatten()
            .filter(|f| f.file_type().is_file())
            .map(|entry| entry.into_path());

        FileDeclutter::new_from_iter(base, iter, levels)
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::prelude::*;
    use assert_fs::TempDir;
    use rand::Rng;

    use super::*;

    #[test]
    fn decluttered_from_path_file_names_same() -> anyhow::Result<()> {
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

        for (source, target) in FileDeclutter::new_from_path(temp_dir.to_path_buf(), 1) {
            assert_ne!(source.parent(), target.parent());
            assert_eq!(source.file_name(), target.file_name());
        }

        Ok(())
    }

    #[test]
    fn decluttered_from_iter_file_names_same() -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();
        let files = (0..100).map(move |_| {
            let mut file_name = rng
                .gen_range(1_000_000_000u64..10_000_000_000u64)
                .to_string();

            if rng.gen_bool(0.25) {
                file_name = format!("subdir/{file_name}");
            }

            PathBuf::from(file_name)
        });

        for (source, target) in FileDeclutter::new_from_iter(PathBuf::new(), files, 1) {
            assert_ne!(source.parent(), target.parent());
            assert_eq!(source.file_name(), target.file_name());
        }

        Ok(())
    }
}
