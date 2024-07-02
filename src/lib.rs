const ALIGNMENT: usize = 1;

#[derive(Debug, Clone, Copy)]
pub struct SegKey {
    start: usize,
    end: usize,
}
impl SegKey {
    pub fn empty_slice() -> Self {
        Self { start: 0, end: 0 }
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Extension only arena for slices of same items
#[derive(Debug, Clone)]
pub struct VecSeg<T> {
    arena: Vec<T>,
}
impl<T> VecSeg<T>
where
    T: Default,
{
    pub fn extend(&mut self, iter: impl Iterator<Item = T>) -> SegKey {
        let element_size = core::mem::size_of::<T>();
        let bytes = self.arena.len() * element_size;
        let padding_bytes = bytes % ALIGNMENT;
        let padding_elements = padding_bytes / element_size;
        for _ in 0..padding_elements {
            self.arena.push(T::default());
        }

        let start = self.arena.len();
        self.arena.extend(iter);
        let end = self.arena.len();
        SegKey { start, end }
    }
}
impl<T> VecSeg<T> {
    pub fn new() -> Self {
        Self { arena: vec![] }
    }

    pub fn slice(&self, key: SegKey) -> &[T] {
        &self.arena[key.start..key.end]
    }
    pub fn slice_mut(&mut self, key: SegKey) -> &mut [T] {
        &mut self.arena[key.start..key.end]
    }
}
impl<T> Default for VecSeg<T> {
    fn default() -> Self {
        Self::new()
    }
}
