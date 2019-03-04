// Copyright 2016 Amanieu d'Antras
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! This library provides implementations of `Mutex`, `RwLock`, `Condvar` and
//! `Once` that are smaller, faster and more flexible than those in the Rust
//! standard library. It also provides a `ReentrantMutex` type.

#![warn(missing_docs)]
#![cfg_attr(feature = "nightly", feature(const_fn))]
#![cfg_attr(feature = "nightly", feature(integer_atomics))]
#![cfg_attr(feature = "nightly", feature(asm))]
#![cfg_attr(feature = "nightly", feature(time_checked_add))]

mod condvar;
mod elision;
mod mutex;
mod once;
mod raw_mutex;
mod raw_rwlock;
mod remutex;
mod rwlock;
mod util;

#[cfg(feature = "deadlock_detection")]
pub mod deadlock;
#[cfg(not(feature = "deadlock_detection"))]
mod deadlock;

pub use self::condvar::{Condvar, WaitTimeoutResult};
pub use self::mutex::{MappedMutexGuard, Mutex, MutexGuard};
pub use self::once::{Once, OnceState, ONCE_INIT};
pub use self::raw_mutex::RawMutex;
pub use self::raw_rwlock::RawRwLock;
pub use self::remutex::{
    MappedReentrantMutexGuard, RawThreadId, ReentrantMutex, ReentrantMutexGuard,
};
pub use self::rwlock::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard,
    RwLockUpgradableReadGuard, RwLockWriteGuard,
};
