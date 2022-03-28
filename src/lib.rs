//! # libmount
//!
//! [Documentation](https://docs.rs/libmount) |
//! [Github](https://github.com/tailhook/libmount) |
//! [Crate](https://crates.io/crates/libmount)
//!
//! This library has two major goals:
//!
//! 1. Add type-safe interface for mount() system call
//! 2. Add very good explanation of what's wrong when the call fails
//!
//! So we have two error types:
//!
//! 1. `OSError` holds mount info and errno
//! 2. `Error` is returned by `OSError::explain()`
//!
//! The first one is returned by `bare_mount()` the second by `mount()`, and
//! using latter is preffered for most situations. Unless performance is
//! too critical (i.e. you are doing thousands of *failing* mounts per second).
//! On the success path there is no overhead.
//!
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

extern crate libc;
extern crate nix;
#[macro_use]
extern crate quick_error;

mod bind;
mod error;
mod explain;
mod modify;
pub mod mountinfo;
mod overlay;
mod remount;
mod tmpfs;
mod util;

use std::io;

pub use bind::BindMount;
use explain::Explainable;
pub use modify::Move;
pub use overlay::Overlay;
pub use remount::Remount;
use remount::RemountError;
pub use tmpfs::Tmpfs;

quick_error! {
    #[derive(Debug)]
    enum MountError {
        Io(err: io::Error) {
            source(err)
            from()
        }
        Remount(err: RemountError) {
            source(err)
            from()
        }
    }
}

/// The raw os error
///
/// This is a wrapper around `io::Error` providing `explain()` method
///
/// Note: you need to explain as fast as possible, because during explain
/// library makes some probes for different things in filesystem, and if
/// anything changes it may give incorrect results.
///
/// You should always `explain()` the errors, unless you are trying lots of
/// mounts for bruteforcing or other similar thing and you are concerned of
/// performance. Usually library does `stat()` and similar things which are
/// much faster than mount anyway. Also explaining is zero-cost in the success
/// path.
///
#[derive(Debug)]
pub struct OSError(MountError, Box<dyn Explainable>);

impl OSError {
    fn from_remount(err: RemountError, explain: Box<dyn Explainable>) -> OSError {
        OSError(MountError::Remount(err), explain)
    }

    fn from_nix(err: nix::Error, explain: Box<dyn Explainable>) -> OSError {
        OSError(MountError::Io(io::Error::from(err)), explain)
    }
}

/// The error holder which contains as much information about why failure
/// happens as the library implementors could gain
///
/// This type only provides `Display` for now, but some programmatic interface
/// is expected in future.
#[derive(Debug)]
pub struct Error(Box<dyn Explainable>, io::Error, String);
