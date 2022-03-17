#![feature(const_mut_refs)]
#![feature(const_replace)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_ptr_read)]

mod interval;
pub use interval::Interval;

pub mod note;

pub mod pitch;
use pitch::Pitch;

pub mod scale;

pub fn transpose(key: Pitch, note: Pitch, to: Pitch) -> Pitch {
    let f = key - note;
    to + f
}
