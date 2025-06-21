use std::{fs::File, io::Read, mem::take, path::Path, time::SystemTime};

use oathc_ast::ParseAstExt;
use oathc_tokenizer::TokenizeExt;

use super::*;

#[derive(Debug)]
pub struct ModFile {
    path_as_file: PathBuf,
    path_as_dir: PathBuf,
    name: Ident,
    path: Try<PathBuf>,
    time: SystemTime,
    content: Mod,
    invalid_file_error: Option<DiagnosticHandle>,
    highlights: Vec<Highlight>,
}

#[derive(Debug)]
pub enum ItemMod {
    File(ModFile),
    Block(ItemModBlock),
    Failure(Option<DiagnosticHandle>),
}

#[derive(Debug)]
pub struct ItemModBlock {
    _delims: delims!("{ }"),
    _mod_: Mod,
}

#[derive(Debug, Default)]
pub struct Mod {
    _submod_dir: Option<PathBuf>,
    #[allow(dead_code)]
    items: Vec<Item>,
    #[allow(dead_code)]
    leftovers: Leftovers,
}

impl ModFile {
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

        let (path, submod_dir, file) = match (file_file, dir_file) {
            (Ok(_), Ok(_)) => (
                Try::Failure(Some(diagnostics.push_error(Error::DoubleMod(name.span(), name.str_id())))),
                None,
                Err(()),
            ),
            (Ok(file), Err(_)) => (Try::Success(path_as_file.clone()), None, Ok(file)),
            (Err(_), Ok(file)) => (
                Try::Success(path_as_dir.clone()),
                Some(path_as_dir.parent().unwrap().to_path_buf()),
                Ok(file),
            ),
            (Err(_), Err(_)) => (
                Try::Failure(Some(diagnostics.push_error(Error::NoMod(name.span(), name.str_id())))),
                None,
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

        let (content, highlights) = if let Ok(mut file) = file {
            let mut text = String::new();
            match file.read_to_string(&mut text) {
                Ok(_) => {}
                Err(_) => {
                    return Self {
                        invalid_file_error: Some(diagnostics.push_error(Error::InvalidFile(Span::from_range(
                            file_interner.intern(path.unwrap_ref()),
                            0,
                            0,
                            0,
                            0,
                        )))),
                        path_as_file,
                        path_as_dir,
                        name,
                        path,
                        time,
                        content: Mod::default(),
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
                .map(|item| {
                    Item::new(
                        item,
                        submod_dir.as_ref().map(|p| p.as_path()),
                        interner,
                        file_interner,
                        diagnostics,
                    )
                })
                .collect();

            let content = Mod {
                _submod_dir: submod_dir,
                items,
                leftovers,
            };

            (content, highlights)
        } else {
            (Mod::default(), vec![])
        };

        Self {
            path_as_file,
            path_as_dir,
            name,
            path,
            time,
            content,
            highlights,
            invalid_file_error: None,
        }
    }

    pub fn check(&mut self, interner: &Interner, file_interner: &FileInterner, diagnostics: &Diagnostics) {
        let file_file = File::open(&self.path_as_file);
        let dir_file = File::open(&self.path_as_dir);
        let check_time = SystemTime::now();

        let (path, submod_dir, file) = match (file_file, dir_file) {
            (Ok(_), Ok(_)) => (
                Try::Failure(Some(
                    diagnostics.push_error(Error::DoubleMod(self.name.span(), self.name.str_id())),
                )),
                None,
                Err(()),
            ),
            (Ok(file), Err(_)) => (Try::Success(self.path_as_file.clone()), None, Ok(file)),
            (Err(_), Ok(file)) => (
                Try::Success(self.path_as_dir.clone()),
                Some(self.path_as_dir.parent().unwrap().to_path_buf()),
                Ok(file),
            ),
            (Err(_), Err(_)) => (
                Try::Failure(Some(
                    diagnostics.push_error(Error::NoMod(self.name.span(), self.name.str_id())),
                )),
                None,
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
            for submod_file in self.content.items.iter_mut().filter_map(|item| match item {
                Item::Mod(ItemMod::File(mod_)) => Some(mod_),
                _ => None,
            }) {
                submod_file.check(interner, file_interner, diagnostics);
            }

            return;
        }

        let (content, highlights) = if let Ok(mut file) = file {
            let mut text = String::new();
            match file.read_to_string(&mut text) {
                Ok(_) => {}
                Err(_) => {
                    *self = Self {
                        invalid_file_error: Some(diagnostics.push_error(Error::InvalidFile(Span::from_range(
                            file_interner.intern(path.unwrap_ref()),
                            0,
                            0,
                            0,
                            0,
                        )))),
                        path_as_file: take(&mut self.path_as_file),
                        path_as_dir: take(&mut self.path_as_dir),
                        name: self.name,
                        path,
                        time,
                        content: Mod::default(),
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
                .map(|item| {
                    Item::new(
                        item,
                        submod_dir.as_ref().map(|p| p.as_path()),
                        interner,
                        file_interner,
                        diagnostics,
                    )
                })
                .collect();

            let content = Mod {
                _submod_dir: submod_dir,
                items,
                leftovers,
            };

            (content, highlights)
        } else {
            (Mod::default(), vec![])
        };

        self.content = content;
        self.path = path;
        self.time = time;
        self.invalid_file_error = None;
        self.highlights = highlights;
    }

    pub fn get_highlights(&self) -> &[Highlight] {
        &self.highlights
    }

    pub fn find(&self, path: impl AsRef<Path>) -> Option<&Self> {
        let path = path.as_ref();

        let is_this = self.path.success_ref().map(|p| p.as_path());
        if Some(path) == is_this { Some(self) } else { None }
    }
}

impl ItemMod {
    pub fn new(
        ast: oathc_ast::Mod,
        submod_dir: Option<&Path>,
        interner: &Interner,
        file_interner: &FileInterner,
        diagnostics: &Diagnostics,
    ) -> Self {
        match ast.body {
            Try::Success(oathc_ast::ModBody::Block(body_ast)) => Self::Block(ItemModBlock {
                _delims: body_ast.frame.delims,
                _mod_: Mod::new(
                    None,
                    SyntaxTree {
                        items: body_ast.items,
                        leftovers: body_ast.frame.leftovers,
                    },
                    interner,
                    file_interner,
                    diagnostics,
                ),
            }),

            Try::Success(oathc_ast::ModBody::Semi(_)) => {
                if let Some(submod_dir) = &submod_dir {
                    Self::File(ModFile::new(
                        submod_dir,
                        *ast.ident.unwrap_ref(),
                        interner,
                        file_interner,
                        diagnostics,
                    ))
                } else {
                    Self::Failure(Some(
                        diagnostics.push_error(Error::FileMod(ast.ident.success_ref().unwrap().span())),
                    ))
                }
            }

            Try::Failure(error) => Self::Failure(error),
        }
    }
}

impl Mod {
    pub fn new(
        submod_dir: Option<PathBuf>,
        ast: SyntaxTree,
        interner: &Interner,
        file_interner: &FileInterner,
        diagnostics: &Diagnostics,
    ) -> Self {
        let items = ast
            .items
            .values
            .into_iter()
            .map(|item| {
                Item::new(
                    item,
                    submod_dir.as_ref().map(|p| p.as_path()),
                    interner,
                    file_interner,
                    diagnostics,
                )
            })
            .collect();

        let leftovers = ast.leftovers;

        Self {
            _submod_dir: submod_dir,
            items,
            leftovers,
        }
    }
}
