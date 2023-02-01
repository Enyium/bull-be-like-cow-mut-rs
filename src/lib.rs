#![warn(clippy::pedantic)]

use core::ops::{Deref, DerefMut};

pub enum Bull<'a, B>
where
    B: 'a + ToOwned + ?Sized,
    <B as ToOwned>::Owned: AsRef<B> + AsMut<B>,
{
    Borrowed(&'a mut B),
    Owned(<B as ToOwned>::Owned),
}

impl<B> Deref for Bull<'_, B>
where
    B: ToOwned + ?Sized,
    <B as ToOwned>::Owned: AsRef<B> + AsMut<B>,
{
    type Target = B;

    fn deref(&self) -> &Self::Target {
        match self {
            Bull::Borrowed(borrowed) => borrowed,
            Bull::Owned(owned) => owned.as_ref(),
        }
    }
}

impl<B> DerefMut for Bull<'_, B>
where
    B: ToOwned + ?Sized,
    <B as ToOwned>::Owned: AsRef<B> + AsMut<B>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Bull::Borrowed(borrowed) => borrowed,
            Bull::Owned(owned) => owned.as_mut(),
        }
    }
}
