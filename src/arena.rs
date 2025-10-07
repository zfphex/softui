//! From https://github.com/zed-industries/zed/blob/main/crates/gpui/src/arena.rs
use std::{
    alloc::{self, handle_alloc_error},
    cell::Cell,
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    rc::Rc,
};

struct ArenaElement<T> {
    value: *mut T,
}

impl<T> Drop for ArenaElement<T> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.value);
        }
    }
}

struct Chunk {
    start: *mut u8,
    end: *mut u8,
    offset: *mut u8,
}

impl Drop for Chunk {
    fn drop(&mut self) {
        unsafe {
            let chunk_size = self.end.offset_from_unsigned(self.start);
            // SAFETY: This succeeded during allocation.
            let layout = alloc::Layout::from_size_align_unchecked(chunk_size, 1);
            alloc::dealloc(self.start, layout);
        }
    }
}

impl Chunk {
    fn new(chunk_size: usize) -> Self {
        unsafe {
            // this only fails if chunk_size is unreasonably huge
            let Ok(layout) = alloc::Layout::from_size_align(chunk_size, 1) else {
                unreachable!();
            };
            let start = alloc::alloc(layout);
            if start.is_null() {
                handle_alloc_error(layout);
            }
            let end = start.add(chunk_size);
            Self {
                start,
                end,
                offset: start,
            }
        }
    }

    fn allocate(&mut self, layout: alloc::Layout) -> Option<NonNull<u8>> {
        unsafe {
            let aligned = self.offset.add(self.offset.align_offset(layout.align()));
            let next = aligned.add(layout.size());

            if next <= self.end {
                self.offset = next;
                NonNull::new(aligned)
            } else {
                None
            }
        }
    }

    fn reset(&mut self) {
        self.offset = self.start;
    }
}

pub struct Arena<T> {
    chunks: Vec<Chunk>,
    elements: Vec<ArenaElement<T>>,
    current_chunk_index: usize,
    chunk_size: usize,
}

impl<T> Drop for Arena<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T> Arena<T> {
    pub fn new(chunk_size: usize) -> Self {
        assert_ne!(chunk_size, 0);
        Self {
            chunks: vec![Chunk::new(chunk_size)],
            elements: Vec::new(),
            current_chunk_index: 0,
            chunk_size,
        }
    }

    pub fn capacity(&self) -> usize {
        self.chunks.len() * self.chunk_size
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        for chunk_index in 0..=self.current_chunk_index {
            self.chunks[chunk_index].reset();
        }
        self.current_chunk_index = 0;
    }

    #[inline(always)]
    pub fn get(&self, idx: usize) -> Option<&T> {
        unsafe { self.elements.get(idx).and_then(|e| e.value.as_ref()) }
    }

    #[inline(always)]
    //TODO:
    pub fn iter<'a>(&'a self) -> impl Iterator + 'a {
        unsafe { self.elements.iter().map(|e| e.value.as_ref()).flatten() }
    }

    #[inline(always)]
    pub fn alloc(&mut self, f: T) -> usize {
        unsafe {
            let layout = alloc::Layout::new::<T>();
            let mut current_chunk = &mut self.chunks[self.current_chunk_index];

            let ptr = if let Some(ptr) = current_chunk.allocate(layout) {
                ptr.as_ptr()
            } else {
                self.current_chunk_index += 1;
                if self.current_chunk_index >= self.chunks.len() {
                    self.chunks.push(Chunk::new(self.chunk_size));
                    assert_eq!(self.current_chunk_index, self.chunks.len() - 1);
                }
                current_chunk = &mut self.chunks[self.current_chunk_index];
                if let Some(ptr) = current_chunk.allocate(layout) {
                    ptr.as_ptr()
                } else {
                    panic!(
                        "Arena chunk_size of {} is too small to allocate {} bytes",
                        self.chunk_size,
                        layout.size()
                    );
                }
            };

            let len = self.elements.len();
            self.elements.push(ArenaElement { value: ptr as *mut T });
            len
        }
    }
}
