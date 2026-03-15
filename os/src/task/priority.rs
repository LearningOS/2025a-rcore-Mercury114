//! Priority definition

use core::isize;

type PriorityInner = isize;

/// Task priority
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(pub PriorityInner); 

impl Priority {
    /// default value for Priority
    pub const DEFAULT: PriorityInner = 16;

    /// new with priority
    pub fn new(value: PriorityInner) -> Self {
        Self(value)
    }
}

impl Default for Priority 
{
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

impl TryFrom<isize> for Priority 
{
    type Error = ();
    
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
           value @ 2..=isize::MAX => PriorityInner::try_from(value)
                .map(Self)
                .map_err(|_| ()),
           _ => Err(()),  
        }
    }
}