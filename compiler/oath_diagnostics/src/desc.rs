pub trait Desc {
    fn desc() -> &'static str;
}

pub use oath_diagnostics_proc_macros::Desc;
