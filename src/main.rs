#![feature(type_alias_impl_trait, generic_const_exprs)]
use std::iter::Filter;
use std::ops::Range;

pub struct BigBoolean<const N: usize>([u8; N / 8 + 1])
where
    [u8; N / 8 + 1]: Sized;

impl<const N: usize> BigBoolean<N>
where
    [u8; N / 8 + 1]: Sized,
{
    pub fn fill(b: bool) -> Self {
        BigBoolean(
            [match b {
                true => 0xFF,
                false => 0x00,
            }; N / 8 + 1],
        )
    }
}

impl<'a, const N: usize> IntoIterator for &'a BigBoolean<N>
where
    [u8; N / 8 + 1]: Sized,
{
    type Item = usize;
    type IntoIter = Filter<Range<usize>, impl FnMut(&usize) -> bool>;

    fn into_iter(self) -> Self::IntoIter {
        (0..N).filter(|&x| 1 << (x % 8) & self.0[x / 8] != 0)
    }
}

fn main() {
    let b = BigBoolean::<7>::fill(true);
    println!("{:?}", b.into_iter().collect::<Vec<usize>>());
}
