use core::sync::atomic::{AtomicUsize, Ordering};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

const FREE: usize = 0;
const BORROW_MUT: usize = 1;
const FROZEN: usize = 2;

pub struct FreezableRefCell<T> {
    value: UnsafeCell<T>,
    state: AtomicUsize,
}

unsafe impl<T: Send> Send for FreezableRefCell<T> { }
unsafe impl<T: Send> Sync for FreezableRefCell<T> { }

impl<T> FreezableRefCell<T> {
    pub fn new(value: T) -> FreezableRefCell<T> {
        FreezableRefCell {
            value: UnsafeCell::new(value),
            state: AtomicUsize::new(FREE),
        }
    }

    pub fn default() -> FreezableRefCell<T> where T: Default {
        FreezableRefCell::new(Default::default())
    }

    // Disable inlining to work around LLVM bug
    #[inline(never)]
    pub fn borrow_mut(&self) -> RefMut<T> {
        if self.state.compare_and_swap(
            FREE,
            BORROW_MUT,
            Ordering::SeqCst) != FREE
        {
            panic!("cell not mutably borrowable");
        }

        RefMut {
            value: unsafe { &mut *self.value.get() },
            state: &self.state,
        }
    }

    // Disable inlining to work around LLVM bug
    #[inline(never)]
    pub fn freeze(&self) {
        if self.state.compare_and_swap(
            FREE,
            FROZEN,
            Ordering::SeqCst) != FREE
        {
            panic!("cell not freezable");
        }
    }

    pub fn borrow(&self) -> &T {
        if self.state.load(Ordering::SeqCst) != FROZEN {
            panic!("cell not frozen")
        }

        unsafe { &*self.value.get() }
    }
}

pub struct RefMut<'a, T: 'a> {
    value: &'a mut T,
    state: &'a AtomicUsize,
}

impl<'a, T: 'a> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value
    }
}

impl<'a, T: 'a> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<'a, T: 'a> Drop for RefMut<'a, T> {
    fn drop(&mut self) {
        self.state.store(FREE, Ordering::SeqCst);
    }
}
