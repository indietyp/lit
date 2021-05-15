use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use either::Either;
use indoc::indoc;
use std::collections::hash_map::Iter;

pub type FileContents = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Directory(HashMap<String, Box<Path>>);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path(Either<FileContents, Box<Directory>>);

type FlatDirectory = Vec<(Vec<String>, FileContents)>;

pub struct DirectoryIterator {
    dir: FlatDirectory,
    cur: usize,
}

impl DirectoryIterator {
    fn flat_map(key: Vec<String>, value: &Path) -> FlatDirectory {
        let path = value.clone();
        match path {
            Path(Either::Left(value)) => {
                vec![(key, value)]
            }
            Path(Either::Right(dir)) => (*dir)
                .0
                .iter()
                .flat_map(|(module, value)| {
                    let mut path = key.clone();
                    path.push(module.clone());

                    Self::flat_map(path, value)
                })
                .collect(),
        }
    }

    fn new(dir: Directory) -> Self {
        let flat: FlatDirectory = dir
            .0
            .iter()
            .flat_map(|(key, value)| Self::flat_map(vec![key.clone()], value))
            .collect();
        DirectoryIterator { dir: flat, cur: 0 }
    }
}

impl Iterator for DirectoryIterator {
    type Item = (Vec<String>, FileContents);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.dir.len() {
            None
        } else {
            let item = self.dir.get(self.cur)?.clone();
            self.cur += 1;

            Some(item)
        }
    }
}

impl Directory {
    pub fn walk(&self) -> DirectoryIterator {
        DirectoryIterator::new(self.clone())
    }

    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: Path) -> Option<Box<Path>> {
        self.0.insert(key, Box::new(value))
    }

    pub fn remove(&mut self, key: &String) -> Option<Box<Path>> {
        self.0.remove(key)
    }

    pub fn get(&mut self, key: &String) -> Option<&Box<Path>> {
        self.0.get(key)
    }

    pub fn iter(&self) -> Iter<String, Box<Path>> {
        self.0.iter()
    }

    fn format(&self, indent: usize, level: Option<usize>) -> String {
        let level = level.unwrap_or(0);
        let spacing = " ".repeat(indent * level);

        let mut output = String::new();

        for (key, boxed) in self.0.clone() {
            let path = *boxed.clone();
            let value = match path {
                Path(Either::Left(_)) => {
                    format!("{s}{name}: File\n", s = spacing, name = key)
                }
                Path(Either::Right(dir)) => {
                    format!(
                        indoc! {"
                        {s}{name}: Directory
                        {contents}"},
                        s = spacing,
                        name = key,
                        contents = dir.format(indent, Some(level + 1))
                    )
                }
            };

            output.push_str(value.as_str());
        }

        output
    }
}

impl Into<Path> for String {
    fn into(self) -> Path {
        Path(Either::Left(self))
    }
}

impl Into<Path> for Directory {
    fn into(self) -> Path {
        Path(Either::Right(Box::new(self)))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::hir::func::fs::{Directory, FileContents};
    use std::collections::HashMap;

    fn create() -> Directory {
        let mut root = Directory::new();
        let mut child = Directory::new();
        let mut child2 = Directory::new();
        let mut child3 = Directory::new();
        let child4 = Directory::new();

        for idx in 0..4 {
            child3.insert(format!("_{}", idx), "<contents>".to_string().into());
        }

        child2.insert("D".to_string(), child3.into());
        child2.insert("E".to_string(), child4.into());
        child.insert("C".to_string(), child2.into());

        for idx in 0..10 {
            child.insert(format!("_{}", idx), "<contents>".to_string().into());
        }

        root.insert("X".to_string(), child.into());

        root
    }

    fn assert_in(dir: &[(Vec<String>, FileContents)], key: Vec<String>) {
        let keys: HashMap<_, _> = dir
            .iter()
            .enumerate()
            .map(|(idx, (a, b))| (a, idx))
            .collect();

        assert!(keys.contains_key(&key));
        let idx = keys.get(&key);
        assert!(idx.is_some());
    }

    #[test]
    fn test_directory_flattening() {
        let root = create();
        let flat: Vec<_> = root.walk().collect();

        assert_in(&flat, vec!["X".to_string(), "_7".to_string()]);
        assert_in(&flat, vec!["X".to_string(), "_0".to_string()]);
        assert_in(
            &flat,
            vec![
                "X".to_string(),
                "C".to_string(),
                "D".to_string(),
                "_1".to_string(),
            ],
        );
    }
}
