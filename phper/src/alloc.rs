// Copyright (c) 2025 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

//! Memory allocation utilities and boxed types for PHP values.

pub use phper_alloc::{RefClone, ToRefOwned};
use std::{
    borrow::{Borrow, BorrowMut}, cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut}, collections::HashMap, fmt::{self, Debug}, mem::ManuallyDrop, ops::{Deref, DerefMut}, ptr::NonNull
};

// thread_local! {
//     static REF_CELL_MAP: RefCell<HashMap<usize, RefCell<()>>> = Default::default();
// }

/// A smart pointer for PHP values allocated in the Zend Engine memory.
///
/// `EBox<T>` provides owned access to values allocated in PHP's memory
/// management system. It automatically handles deallocation when dropped,
/// ensuring proper cleanup of PHP resources.
pub struct EBox<T> {
    ptr: NonNull<T>,
    cell: RefCell<()>,
}

impl<T> EBox<T> {
    /// Constructs from a raw pointer.
    ///
    /// # Safety
    ///
    /// Make sure the pointer is from `into_raw`, or created from `emalloc`.
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        // REF_CELL_MAP.with_borrow_mut(|map| {
        //     map.entry(raw as usize).or_default();
        // });
        Self { ptr: NonNull::new(raw).unwrap(), cell: RefCell::new(()) }
    }

    /// Consumes and returning a wrapped raw pointer.
    ///
    /// Will leak memory.
    pub fn into_raw(b: EBox<T>) -> *mut T {
        // REF_CELL_MAP.with_borrow_mut(|map| {
        //     map.remove(&(b.ptr.as_ptr() as usize));
        // });
        ManuallyDrop::new(b).ptr.as_ptr()
    }

    pub fn borrow(&self) -> ERef<'_, T> {
        // let cell = REF_CELL_MAP.with_borrow(|map| {
        //     map.get(&(self.ptr.as_ptr() as usize)).unwrap().clone()
        // });
        ERef {
            ptr: self.ptr,
            borrow: self.cell.borrow(),
        }
    }

    pub fn try_borrow(&self) -> Result<ERef<'_, T>, BorrowError> {
        Ok(
            ERef {
                ptr: self.ptr,
                borrow: self.cell.try_borrow()?,
            }
        )
    }

    pub fn borrow_mut(&self) -> ERefMut<'_, T> {
        // let cell = REF_CELL_MAP.with_borrow(|map| {
        //     map.get(&(self.ptr.as_ptr() as usize)).unwrap().clone()
        // });
        ERefMut {
            ptr: self.ptr,
            borrow: self.cell.borrow_mut(),
        }
    }

    pub fn try_borrow_mut(&self) -> Result<ERefMut<'_, T>, BorrowMutError> {
        Ok(
            ERefMut {
                ptr: self.ptr,
                borrow: self.cell.try_borrow_mut()?,
            }
        )
    }
}

impl<T> Drop for EBox<T> {
    fn drop(&mut self) {
        // REF_CELL_MAP.with_borrow_mut(|map| {
        //     map.remove(&(self.ptr.as_ptr() as usize));
        // });
        unsafe {
            self.ptr.drop_in_place();
        }
    }
}

impl<T: Debug> Debug for EBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("EBox");
        match self.try_borrow() {
            Ok(borrow) => d.field("value", &borrow),
            Err(_) => d.field("value", &format_args!("<borrowed>")),
        };
        d.finish()
    }
}

pub struct ERef<'b, T> where T: 'b {
    ptr: NonNull<T>,
    borrow: Ref<'b, ()>,
}

impl<T> Deref for ERef<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: the value is accessible as long as we hold our borrow.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: Debug> Debug for ERef<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&**self, f)
    }
}

pub struct ERefMut<'b, T> where T: 'b {
    ptr: NonNull<T>,
    borrow: RefMut<'b, ()>,
}

impl<T> Deref for ERefMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: the value is accessible as long as we hold our borrow.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for ERefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: the value is accessible as long as we hold our borrow.
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: Debug> Debug for ERefMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&*(self.deref()), f)
    }
}
