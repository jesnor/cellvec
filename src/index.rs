pub trait Index {
    fn to_usize(self) -> usize;
    fn from_usize(size: usize) -> Self;
}

impl Index for usize {
    fn to_usize(self) -> usize { self }
    fn from_usize(size: usize) -> Self { size }
}

impl Index for u8 {
    fn to_usize(self) -> usize { self as usize }
    fn from_usize(size: usize) -> Self { size as Self }
}

impl Index for u16 {
    fn to_usize(self) -> usize { self as usize }
    fn from_usize(size: usize) -> Self { size as Self }
}

impl Index for u32 {
    fn to_usize(self) -> usize { self as usize }
    fn from_usize(size: usize) -> Self { size as Self }
}
