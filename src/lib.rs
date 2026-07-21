extern crate self as alife;

pub mod bootstrap;
pub mod core;
pub mod runner;

pub mod cell;
pub mod organism;
pub mod physics;
pub mod renderer;
pub mod simulation;
pub mod world;

pub mod process {
    pub use crate::core::process::*;
}

pub mod bin {
    pub mod sweep_analyzer;
}

pub mod observer;
pub mod storage;
pub mod viewer_server;
