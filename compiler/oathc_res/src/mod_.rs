use std::{fs::File, io::Read, mem::take, path::Path, time::SystemTime};

use oathc_ast::ParseAstExt;
use oathc_tokenizer::TokenizeExt;

use super::*;

#[derive(Debug)]
pub struct Mod {
    path_as_file: PathBuf,
    path_as_dir: PathBuf,
    name: Ident,
    path: Try<PathBuf>,
    time: SystemTime,
    #[allow(dead_code)]
    items: Vec<Item>,
    #[allow(dead_code)]
    leftovers: Leftovers,
    highlights: Vec<Highlight>,
}

impl Mod {
    pub fn new(
        dir_path: impl Into<PathBuf>,
        name: Ident,
        interner: &Interner,
        file_interner: &FileInterner,
        diagnostics: &Diagnostics,
    ) -> Self {
        let dir_path = dir_path.into();
        let name_str = interner.unintern(name.str_id());

        let path_as_file = dir_path.join(&name_str).with_extension("oh");
        let path_as_dir = dir_path.join(&name_str).join("mod.oh");

        let file_file = File::open(&path_as_file);
        let dir_file = File::open(&path_as_dir);
        let check_time = SystemTime::now();

        let (path, file) = match (file_file, dir_file) {
            (Ok(_), Ok(_)) => (
                Try::Failure(Some(diagnostics.push_error(Error::DoubleMod(name.span(), name.str_id())))),
                Err(()),
            ),
            (Ok(file), Err(_)) => (Try::Success(path_as_file.clone()), Ok(file)),
            (Err(_), Ok(file)) => (Try::Success(path_as_dir.clone()), Ok(file)),
            (Err(_), Err(_)) => (
                Try::Failure(Some(diagnostics.push_error(Error::NoMod(name.span(), name.str_id())))),
                Err(()),
            ),
        };

        let time = match &file {
            Ok(file) => file.metadata().and_then(|metadata| metadata.modified()).unwrap_or_else(|_| {
                file.set_modified(check_time).unwrap();

                check_time
            }),
            Err(_) => check_time,
        };

        let (items, leftovers, highlights) = if let Ok(mut file) = file {
            let mut text = String::new();
            match file.read_to_string(&mut text) {
                Ok(_) => {}
                Err(_) => {
                    return Self {
                        leftovers: Leftovers {
                            error: Some(diagnostics.push_error(Error::InvalidFile(Span::from_range(
                                file_interner.intern(path.unwrap_ref()),
                                0,
                                0,
                                0,
                                0,
                            )))),
                        },
                        path_as_file,
                        path_as_dir,
                        name,
                        path,
                        time,
                        items: vec![],
                        highlights: vec![],
                    };
                }
            };

            let path = path.unwrap_ref();

            let mut highlights = vec![];
            let ast = text
                .tokenize(file_interner.intern(path), interner, diagnostics, &mut highlights)
                .parse_ast();

            let leftovers = ast.leftovers;

            let items = ast
                .items
                .values
                .into_iter()
                .map(|item| Item::new(item, diagnostics))
                .collect();

            (items, leftovers, highlights)
        } else {
            (vec![], Leftovers::default(), vec![])
        };

        Self {
            path_as_file,
            path_as_dir,
            name,
            path,
            time,
            items,
            leftovers,
            highlights,
        }
    }

    pub fn check(&mut self, interner: &Interner, file_interner: &FileInterner, diagnostics: &Diagnostics) {
        let file_file = File::open(&self.path_as_file);
        let dir_file = File::open(&self.path_as_dir);
        let check_time = SystemTime::now();

        let (path, file) = match (file_file, dir_file) {
            (Ok(_), Ok(_)) => (
                Try::Failure(Some(
                    diagnostics.push_error(Error::DoubleMod(self.name.span(), self.name.str_id())),
                )),
                Err(()),
            ),
            (Ok(file), Err(_)) => (Try::Success(self.path_as_file.clone()), Ok(file)),
            (Err(_), Ok(file)) => (Try::Success(self.path_as_dir.clone()), Ok(file)),
            (Err(_), Err(_)) => (
                Try::Failure(Some(
                    diagnostics.push_error(Error::NoMod(self.name.span(), self.name.str_id())),
                )),
                Err(()),
            ),
        };

        let time = match &file {
            Ok(file) => file.metadata().and_then(|metadata| metadata.modified()).unwrap_or_else(|_| {
                file.set_modified(check_time).unwrap();

                check_time
            }),
            Err(_) => check_time,
        };

        if path.success_ref() == self.path.success_ref() && time <= self.time {
            return;
        }

        let (items, leftovers, highlights) = if let Ok(mut file) = file {
            let mut text = String::new();
            match file.read_to_string(&mut text) {
                Ok(_) => {}
                Err(_) => {
                    *self = Self {
                        leftovers: Leftovers {
                            error: Some(diagnostics.push_error(Error::InvalidFile(Span::from_range(
                                file_interner.intern(path.unwrap_ref()),
                                0,
                                0,
                                0,
                                0,
                            )))),
                        },
                        path_as_file: take(&mut self.path_as_file),
                        path_as_dir: take(&mut self.path_as_dir),
                        name: self.name,
                        path,
                        time,
                        items: vec![],
                        highlights: vec![],
                    };

                    return;
                }
            };

            let path = path.unwrap_ref();

            let mut highlights = vec![];
            let ast = text
                .tokenize(file_interner.intern(path), interner, diagnostics, &mut highlights)
                .parse_ast();

            let leftovers = ast.leftovers;

            let items = ast
                .items
                .values
                .into_iter()
                .map(|item| Item::new(item, diagnostics))
                .collect();

            (items, leftovers, highlights)
        } else {
            (vec![], Leftovers::default(), vec![])
        };

        *self = Self {
            path_as_file: take(&mut self.path_as_file),
            path_as_dir: take(&mut self.path_as_dir),
            name: self.name,
            path,
            time,
            items,
            leftovers,
            highlights,
        };
    }

    pub fn get_highlights(&self) -> &[Highlight] {
        &self.highlights
    }

    pub fn find(&self, path: impl AsRef<Path>) -> Option<&Mod> {
        let path = path.as_ref();

        let is_this = self.path.success_ref().map(|p| p.as_path());
        if Some(path) == is_this { Some(self) } else { None }
    }
}
