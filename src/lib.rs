extern crate cannyls;
#[allow(unused_imports)]
#[macro_use]
extern crate trackable;
extern crate structopt;

pub mod storage;
pub mod timer;

#[macro_export]
macro_rules! kilo {
    ($e:expr) => {
        $e * 1024
    };
}

#[macro_export]
macro_rules! mega {
    ($e:expr) => {
        $e * 1024 * 1024
    };
}

#[macro_export]
macro_rules! giga {
    ($e:expr) => {
        $e * 1024 * 1024 * 1024
    };
}

#[macro_export]
macro_rules! man {
    ($e:expr) => {
        $e * 1_0000
    };
}

#[macro_export]
macro_rules! oku {
    ($e:expr) => {
        $e * 1_0000_0000
    };
}
