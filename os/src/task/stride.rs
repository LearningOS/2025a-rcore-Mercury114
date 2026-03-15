//! Stride definition for Stride algorithm

use super::priority::Priority;

type StrideInner = usize;

/// Stride value for stride scheduling algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stride(StrideInner);

impl Stride {
    /// magic number, / 10000 is necessary
    const BIG_STRIDE: StrideInner = StrideInner::MAX / 10000;
}

impl Stride 
{
    /// Perform a step in stride scheduling, increasing stride by pass value
    pub fn step(&mut self, priority: Priority) 
    {
        self.0 += Self::BIG_STRIDE / priority.0 as StrideInner
    }
}

impl Ord for Stride 
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    } 
}

impl PartialOrd for Stride
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}