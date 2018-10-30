//! Task notification.

mod spawn;
pub use self::spawn::{Spawn, LocalSpawn, SpawnError};

pub use core::task::{Poll, Waker, LocalWaker, UnsafeWake};
#[cfg(feature = "std")]
pub use std::task::{Wake, local_waker, local_waker_from_nonlocal};
