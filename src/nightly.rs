// Copyright 2016 Amanieu d'Antras
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use core::mem;
use core::ops;
use core::num::Wrapping;
use core::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

mod fallback;

#[inline]
pub fn atomic_is_lock_free<T>() -> bool {
    match mem::size_of::<T>() {
        #[cfg(target_has_atomic = "8")]
        1 if mem::align_of::<T>() >= 1 => true,
        #[cfg(target_has_atomic = "16")]
        2 if mem::align_of::<T>() >= 2 => true,
        #[cfg(target_has_atomic = "32")]
        4 if mem::align_of::<T>() >= 4 => true,
        #[cfg(target_has_atomic = "64")]
        8 if mem::align_of::<T>() >= 8 => true,
        _ => false,
    }
}

#[inline]
pub unsafe fn atomic_load<T>(dst: *mut T, order: Ordering) -> T {
    match mem::size_of::<T>() {
        #[cfg(target_has_atomic = "8")]
        1 if mem::align_of::<T>() >= 1 => {
            mem::transmute_copy(&(*(dst as *const AtomicU8)).load(order))
        }
        #[cfg(target_has_atomic = "16")]
        2 if mem::align_of::<T>() >= 2 => {
            mem::transmute_copy(&(*(dst as *const AtomicU16)).load(order))
        }
        #[cfg(target_has_atomic = "32")]
        4 if mem::align_of::<T>() >= 4 => {
            mem::transmute_copy(&(*(dst as *const AtomicU32)).load(order))
        }
        #[cfg(target_has_atomic = "64")]
        8 if mem::align_of::<T>() >= 8 => {
            mem::transmute_copy(&(*(dst as *const AtomicU64)).load(order))
        }
        _ => fallback::atomic_load(dst),
    }
}

#[inline]
pub unsafe fn atomic_store<T>(dst: *mut T, val: T, order: Ordering) {
    match mem::size_of::<T>() {
        #[cfg(target_has_atomic = "8")]
        1 if mem::align_of::<T>() >= 1 => {
            (*(dst as *const AtomicU8)).store(mem::transmute_copy(&val), order)
        }
        #[cfg(target_has_atomic = "16")]
        2 if mem::align_of::<T>() >= 2 => {
            (*(dst as *const AtomicU16)).store(mem::transmute_copy(&val), order)
        }
        #[cfg(target_has_atomic = "32")]
        4 if mem::align_of::<T>() >= 4 => {
            (*(dst as *const AtomicU32)).store(mem::transmute_copy(&val), order)
        }
        #[cfg(target_has_atomic = "64")]
        8 if mem::align_of::<T>() >= 8 => {
            (*(dst as *const AtomicU64)).store(mem::transmute_copy(&val), order)
        }
        _ => fallback::atomic_store(dst, val),
    }
}

#[inline]
pub unsafe fn atomic_swap<T>(dst: *mut T, val: T, order: Ordering) -> T {
    match mem::size_of::<T>() {
        #[cfg(target_has_atomic = "8")]
        1 if mem::align_of::<T>() >= 1 => {
            mem::transmute_copy(&(*(dst as *const AtomicU8)).swap(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "16")]
        2 if mem::align_of::<T>() >= 2 => {
            mem::transmute_copy(&(*(dst as *const AtomicU16))
                .swap(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "32")]
        4 if mem::align_of::<T>() >= 4 => {
            mem::transmute_copy(&(*(dst as *const AtomicU32))
                .swap(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "64")]
        8 if mem::align_of::<T>() >= 8 => {
            mem::transmute_copy(&(*(dst as *const AtomicU64))
                .swap(mem::transmute_copy(&val), order))
        }
        _ => fallback::atomic_swap(dst, val),
    }
}

#[inline]
unsafe fn map_result<T, U>(r: Result<T, T>) -> Result<U, U> {
    match r {
        Ok(x) => Ok(mem::transmute_copy(&x)),
        Err(x) => Err(mem::transmute_copy(&x)),
    }
}

#[inline]
pub unsafe fn atomic_compare_exchange<T>(dst: *mut T,
                                         current: T,
                                         new: T,
                                         success: Ordering,
                                         failure: Ordering)
                                         -> Result<T, T> {
    match mem::size_of::<T>() {
        #[cfg(target_has_atomic = "8")]
        1 if mem::align_of::<T>() >= 1 => {
            map_result((*(dst as *const AtomicU8)).compare_exchange(mem::transmute_copy(&current),
                                                                    mem::transmute_copy(&new),
                                                                    success,
                                                                    failure))
        }
        #[cfg(target_has_atomic = "16")]
        2 if mem::align_of::<T>() >= 2 => {
            map_result((*(dst as *const AtomicU16)).compare_exchange(mem::transmute_copy(&current),
                                                                     mem::transmute_copy(&new),
                                                                     success,
                                                                     failure))
        }
        #[cfg(target_has_atomic = "32")]
        4 if mem::align_of::<T>() >= 4 => {
            map_result((*(dst as *const AtomicU32)).compare_exchange(mem::transmute_copy(&current),
                                                                     mem::transmute_copy(&new),
                                                                     success,
                                                                     failure))
        }
        #[cfg(target_has_atomic = "64")]
        8 if mem::align_of::<T>() >= 8 => {
            map_result((*(dst as *const AtomicU64)).compare_exchange(mem::transmute_copy(&current),
                                                                     mem::transmute_copy(&new),
                                                                     success,
                                                                     failure))
        }
        _ => fallback::atomic_compare_exchange(dst, current, new),
    }
}

#[inline]
pub unsafe fn atomic_compare_exchange_weak<T>(dst: *mut T,
                                              current: T,
                                              new: T,
                                              success: Ordering,
                                              failure: Ordering)
                                              -> Result<T, T> {
    match mem::size_of::<T>() {
        #[cfg(target_has_atomic = "8")]
        1 if mem::align_of::<T>() >= 1 => {
            map_result((*(dst as *const AtomicU8))
                .compare_exchange_weak(mem::transmute_copy(&current),
                                       mem::transmute_copy(&new),
                                       success,
                                       failure))
        }
        #[cfg(target_has_atomic = "16")]
        2 if mem::align_of::<T>() >= 2 => {
            map_result((*(dst as *const AtomicU16))
                .compare_exchange_weak(mem::transmute_copy(&current),
                                       mem::transmute_copy(&new),
                                       success,
                                       failure))
        }
        #[cfg(target_has_atomic = "32")]
        4 if mem::align_of::<T>() >= 4 => {
            map_result((*(dst as *const AtomicU32))
                .compare_exchange_weak(mem::transmute_copy(&current),
                                       mem::transmute_copy(&new),
                                       success,
                                       failure))
        }
        #[cfg(target_has_atomic = "64")]
        8 if mem::align_of::<T>() >= 8 => {
            map_result((*(dst as *const AtomicU64))
                .compare_exchange_weak(mem::transmute_copy(&current),
                                       mem::transmute_copy(&new),
                                       success,
                                       failure))
        }
        _ => fallback::atomic_compare_exchange(dst, current, new),
    }
}

#[inline]
pub unsafe fn atomic_add<T: Copy>(dst: *mut T, val: T, order: Ordering) -> T
    where Wrapping<T>: ops::Add<Output = Wrapping<T>>
{
    match mem::size_of::<T>() {
        #[cfg(target_has_atomic = "8")]
        1 if mem::align_of::<T>() >= 1 => {
            mem::transmute_copy(&(*(dst as *const AtomicU8))
                .fetch_add(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "16")]
        2 if mem::align_of::<T>() >= 2 => {
            mem::transmute_copy(&(*(dst as *const AtomicU16))
                .fetch_add(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "32")]
        4 if mem::align_of::<T>() >= 4 => {
            mem::transmute_copy(&(*(dst as *const AtomicU32))
                .fetch_add(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "64")]
        8 if mem::align_of::<T>() >= 8 => {
            mem::transmute_copy(&(*(dst as *const AtomicU64))
                .fetch_add(mem::transmute_copy(&val), order))
        }
        _ => fallback::atomic_add(dst, val),
    }
}

#[inline]
pub unsafe fn atomic_sub<T: Copy>(dst: *mut T, val: T, order: Ordering) -> T
    where Wrapping<T>: ops::Sub<Output = Wrapping<T>>
{
    match mem::size_of::<T>() {
        #[cfg(target_has_atomic = "8")]
        1 if mem::align_of::<T>() >= 1 => {
            mem::transmute_copy(&(*(dst as *const AtomicU8))
                .fetch_sub(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "16")]
        2 if mem::align_of::<T>() >= 2 => {
            mem::transmute_copy(&(*(dst as *const AtomicU16))
                .fetch_sub(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "32")]
        4 if mem::align_of::<T>() >= 4 => {
            mem::transmute_copy(&(*(dst as *const AtomicU32))
                .fetch_sub(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "64")]
        8 if mem::align_of::<T>() >= 8 => {
            mem::transmute_copy(&(*(dst as *const AtomicU64))
                .fetch_sub(mem::transmute_copy(&val), order))
        }
        _ => fallback::atomic_sub(dst, val),
    }
}

#[inline]
pub unsafe fn atomic_and<T: Copy + ops::BitAnd<Output = T>>(dst: *mut T,
                                                            val: T,
                                                            order: Ordering)
                                                            -> T {
    match mem::size_of::<T>() {
        #[cfg(target_has_atomic = "8")]
        1 if mem::align_of::<T>() >= 1 => {
            mem::transmute_copy(&(*(dst as *const AtomicU8))
                .fetch_and(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "16")]
        2 if mem::align_of::<T>() >= 2 => {
            mem::transmute_copy(&(*(dst as *const AtomicU16))
                .fetch_and(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "32")]
        4 if mem::align_of::<T>() >= 4 => {
            mem::transmute_copy(&(*(dst as *const AtomicU32))
                .fetch_and(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "64")]
        8 if mem::align_of::<T>() >= 8 => {
            mem::transmute_copy(&(*(dst as *const AtomicU64))
                .fetch_and(mem::transmute_copy(&val), order))
        }
        _ => fallback::atomic_and(dst, val),
    }
}

#[inline]
pub unsafe fn atomic_or<T: Copy + ops::BitOr<Output = T>>(dst: *mut T,
                                                          val: T,
                                                          order: Ordering)
                                                          -> T {
    match mem::size_of::<T>() {
        #[cfg(target_has_atomic = "8")]
        1 if mem::align_of::<T>() >= 1 => {
            mem::transmute_copy(&(*(dst as *const AtomicU8))
                .fetch_or(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "16")]
        2 if mem::align_of::<T>() >= 2 => {
            mem::transmute_copy(&(*(dst as *const AtomicU16))
                .fetch_or(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "32")]
        4 if mem::align_of::<T>() >= 4 => {
            mem::transmute_copy(&(*(dst as *const AtomicU32))
                .fetch_or(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "64")]
        8 if mem::align_of::<T>() >= 8 => {
            mem::transmute_copy(&(*(dst as *const AtomicU64))
                .fetch_or(mem::transmute_copy(&val), order))
        }
        _ => fallback::atomic_or(dst, val),
    }
}

#[inline]
pub unsafe fn atomic_xor<T: Copy + ops::BitXor<Output = T>>(dst: *mut T,
                                                            val: T,
                                                            order: Ordering)
                                                            -> T {
    match mem::size_of::<T>() {
        #[cfg(target_has_atomic = "8")]
        1 if mem::align_of::<T>() >= 1 => {
            mem::transmute_copy(&(*(dst as *const AtomicU8))
                .fetch_xor(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "16")]
        2 if mem::align_of::<T>() >= 2 => {
            mem::transmute_copy(&(*(dst as *const AtomicU16))
                .fetch_xor(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "32")]
        4 if mem::align_of::<T>() >= 4 => {
            mem::transmute_copy(&(*(dst as *const AtomicU32))
                .fetch_xor(mem::transmute_copy(&val), order))
        }
        #[cfg(target_has_atomic = "64")]
        8 if mem::align_of::<T>() >= 8 => {
            mem::transmute_copy(&(*(dst as *const AtomicU64))
                .fetch_xor(mem::transmute_copy(&val), order))
        }
        _ => fallback::atomic_xor(dst, val),
    }
}
