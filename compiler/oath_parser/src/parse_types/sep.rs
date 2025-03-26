use std::marker::PhantomData;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sep<T, S>(Vec<T>, PhantomData<S>);

impl<T: ParseDesc, S: ParseDesc> ParseDesc for Sep<T, S> {
    fn desc() -> &'static str {
        "seperated items"
    }
}
impl<T: OptionParse, S: OptionParse> OptionParse for Sep<T, S> {
    fn option_parse(parser: &mut Parser<impl ParserIterator>) -> Option<Self> {
        let first = T::option_parse(parser)?;

        let mut vec = vec![first];

        while S::option_parse(parser).is_some() {
            match T::try_parse(parser) {
                Try::Success(value) => vec.push(value),
                Try::Failure => break,
            }
        }

        Some(Self::from_vec(vec).unwrap())
    }
}
impl<T: Detect, S: ParseDesc> Detect for Sep<T, S> {
    fn detect(parser: &Parser<impl ParserIterator>) -> bool {
        T::detect(parser)
    }
}

impl<T, S> IntoIterator for Sep<T, S> {
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a, T, S> IntoIterator for &'a Sep<T, S> {
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<'a, T, S> IntoIterator for &'a mut Sep<T, S> {
    type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<T, S> Sep<T, S> {
    pub fn from_vec(vec: Vec<T>) -> Option<Self> {
        if vec.len() > 0 {
            Some(Self(vec, PhantomData))
        } else {
            None
        }
    }

    pub fn vec(&self) -> &Vec<T> {
        &self.0
    }
    pub fn vec_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.0.iter_mut()
    }

    pub fn first(&self) -> &T {
        self.0.first().unwrap()
    }
    pub fn first_mut(&mut self) -> &mut T {
        self.0.first_mut().unwrap()
    }

    pub fn last(&self) -> &T {
        self.0.last().unwrap()
    }
    pub fn last_mut(&mut self) -> &mut T {
        self.0.last_mut().unwrap()
    }
}
