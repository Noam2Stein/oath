use std::{fs::File, io::Read, path::Path, time::SystemTime};

use oathc_ast::ParseAstExt;
use oathc_tokenizer::TokenizeExt;

use super::*;

#[derive(Debug)]
pub struct DiskMod {
    path_as_file: PathBuf,
    path_as_dir: PathBuf,
    ident: Ident,
    file: Try<ModFile>,
}

#[derive(Debug)]
pub struct ModFile {
    path: PathBuf,
    content: Try<Mod>,
    highlights: Vec<Highlight>,
    time: SystemTime,
}

#[derive(Debug)]
pub enum ItemMod {
    File(DiskMod),
    Block(ItemModBlock),
    Failure(Option<DiagnosticHandle>),
}

#[derive(Debug)]
pub struct ItemModBlock {
    _delims: delims!("{ }"),
    _mod_: Mod,
}

#[derive(Debug)]
pub struct Mod {
    _submod_dir: Option<PathBuf>,
    #[allow(dead_code)]
    items: Vec<Item>,
    #[allow(dead_code)]
    leftovers: Leftovers,
}

#[derive(Debug)]
pub struct ModRawFile {
    path: PathBuf,
    file: File,
    submod_dir: Option<PathBuf>,
}

impl DiskMod {
    pub fn new(
        dir_path: impl Into<PathBuf>,
        ident: Ident,
        interner: &Interner,
        file_interner: &FileInterner,
        diagnostics: &Diagnostics,
    ) -> Self {
        let dir_path = dir_path.into();
        let ident_str = interner.unintern(ident.str_id());

        let path_as_file = dir_path.join(&ident_str).with_extension("oh");
        let path_as_dir = dir_path.join(&ident_str).join("mod.oh");

        let file = ModRawFile::try_new(&path_as_file, &path_as_dir, ident, diagnostics)
            .map(|raw_file| raw_file.resolve(interner, file_interner, diagnostics));

        Self {
            path_as_file,
            path_as_dir,
            ident,
            file,
        }
    }

    pub fn check(&mut self, interner: &Interner, file_interner: &FileInterner, diagnostics: &Diagnostics) {
        let raw_file = ModRawFile::try_new(&self.path_as_file, &self.path_as_dir, self.ident, diagnostics);

        if let (Some(raw_file), Some(previous_file)) = (raw_file.success_ref(), self.file.success_mut()) {
            if raw_file.is_unchanged_from(previous_file) {
                if let Try::Success(content) = &mut previous_file.content {
                    content.check_submods(interner, file_interner, diagnostics);
                }

                return;
            }
        }

        self.file = raw_file.map(|raw_file| raw_file.resolve(interner, file_interner, diagnostics));
    }

    pub fn get_highlights(&self) -> &[Highlight] {
        match self.file {
            Try::Success(ref file) => &file.highlights,
            Try::Failure(_) => &[],
        }
    }

    pub fn find(&self, path: impl AsRef<Path>) -> Option<&Self> {
        let path = path.as_ref();

        let is_this = self.file.success_ref().map_or(false, |file| file.path == path);

        is_this.then_some(self)
    }
}

impl ModRawFile {
    fn try_new(path_as_file: &Path, path_as_dir: &Path, ident: Ident, diagnostics: &Diagnostics) -> Try<Self> {
        let file_file = File::open(&path_as_file);
        let dir_file = File::open(&path_as_dir);

        match (file_file, dir_file) {
            (Ok(_), Ok(_)) => Try::Failure(Some(diagnostics.push_error(Error::DoubleMod(ident.span(), ident.str_id())))),
            (Ok(file), Err(_)) => Try::Success(Self {
                path: path_as_file.to_path_buf(),
                file,
                submod_dir: None,
            }),
            (Err(_), Ok(file)) => Try::Success(Self {
                path: path_as_dir.to_path_buf(),
                file,
                submod_dir: Some(path_as_dir.parent().unwrap().to_path_buf()),
            }),
            (Err(_), Err(_)) => Try::Failure(Some(diagnostics.push_error(Error::NoMod(ident.span(), ident.str_id())))),
        }
    }

    fn is_unchanged_from(&self, previous: &ModFile) -> bool {
        let modified_time = self
            .file
            .metadata()
            .and_then(|metadata| metadata.modified())
            .unwrap_or_else(|_| {
                let time = SystemTime::now();
                self.file.set_modified(time).unwrap();

                time
            });

        self.path == previous.path && modified_time <= previous.time
    }

    fn resolve(mut self, interner: &Interner, file_interner: &FileInterner, diagnostics: &Diagnostics) -> ModFile {
        let interned_path = file_interner.intern(&self.path);

        let time = self
            .file
            .metadata()
            .and_then(|metadata| metadata.modified())
            .unwrap_or_else(|_| {
                let time = SystemTime::now();
                self.file.set_modified(time).unwrap();

                time
            });

        let mut text = String::new();
        match self.file.read_to_string(&mut text) {
            Ok(_) => {}
            Err(_) => {
                return ModFile {
                    content: Try::Failure(Some(diagnostics.push_error(Error::InvalidFile(Span::from_range(
                        interned_path,
                        0,
                        0,
                        0,
                        0,
                    ))))),
                    path: self.path,
                    time,
                    highlights: vec![],
                };
            }
        };

        let mut highlights = vec![];
        let ast = text
            .tokenize(interned_path, interner, diagnostics, &mut highlights)
            .parse_ast();

        let leftovers = ast.leftovers;

        let items = ast
            .items
            .into_iter()
            .map(|item| {
                Item::new(
                    item,
                    self.submod_dir.as_ref().map(|p| p.as_path()),
                    interner,
                    file_interner,
                    diagnostics,
                )
            })
            .collect();

        let content = Mod {
            _submod_dir: self.submod_dir,
            items,
            leftovers,
        };

        ModFile {
            path: self.path,
            content: Try::Success(content),
            highlights,
            time,
        }
    }
}

impl ItemMod {
    pub(super) fn new(
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
                    Self::File(DiskMod::new(
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
    pub(super) fn new(
        submod_dir: Option<PathBuf>,
        ast: SyntaxTree,
        interner: &Interner,
        file_interner: &FileInterner,
        diagnostics: &Diagnostics,
    ) -> Self {
        let items = ast
            .items
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

    fn check_submods(&mut self, interner: &Interner, file_interner: &FileInterner, diagnostics: &Diagnostics) {
        for item in &mut self.items {
            if let Item::Mod(ItemMod::File(mod_)) = item {
                mod_.check(interner, file_interner, diagnostics);
            }
        }
    }
}
