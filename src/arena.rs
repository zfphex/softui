use std::{
    cell::UnsafeCell,
    fmt::Debug,
    ops::{Index, IndexMut},
};

pub struct Arena<T> {
    pub items: UnsafeCell<Vec<T>>,
}

impl<T> Arena<T> {
    pub const fn new() -> Self {
        Self {
            items: UnsafeCell::new(Vec::new()),
        }
    }
    pub fn alloc(&self, item: T) -> usize {
        let items = unsafe { &mut *self.items.get() };
        let id = items.len();
        items.push(item);
        id
    }
    pub fn clear(&self) {
        let items = unsafe { &mut *self.items.get() };
        items.clear();
    }
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        let items = unsafe { &*self.items.get() };
        items.iter()
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        unsafe { (&*self.items.get()).get(index) }
    }
    pub unsafe fn get_mut(&self, index: usize) -> Option<&mut T> {
        unsafe { (&mut *self.items.get()).get_mut(index) }
    }
    pub unsafe fn as_mut_slice(&self) -> &mut [T] {
        unsafe { (&mut *self.items.get()).as_mut_slice() }
    }
}

impl<T> Index<usize> for Arena<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &(&*self.items.get())[index] }
    }
}

impl<T: Debug> Debug for Arena<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arena")
            .field("items", unsafe { &*self.items.get() })
            .finish()
    }
}

// impl<T> IndexMut<usize> for Arena<T> {
//     fn index_mut(&mut self, index: usize) -> &mut T {
//         unsafe { &mut (&mut *self.items.get())[index] }
//     }
// }

unsafe impl<T> Send for Arena<T> {}
unsafe impl<T> Sync for Arena<T> {}
