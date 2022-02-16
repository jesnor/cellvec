struct PagedCellVec<T, const PAGE_CAP: usize> {
    pages:      FixedCellSet<CellVec<T>, PAGE_CAP>,
    total_size: Cell<usize>,
}

impl<T, const PAGE_CAP: usize> PagedCellVec<T, PAGE_CAP> {
    fn new(page_size: usize) -> Self {
        let s = Self {
            pages:      Default::default(),
            total_size: Default::default(),
        };
    }
}
