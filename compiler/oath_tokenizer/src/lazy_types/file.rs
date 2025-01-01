pub struct LazyTokenFile<'src> {}

impl Iterator for LazyTokenFile {
    type Item = LazyTokenTree;
}
