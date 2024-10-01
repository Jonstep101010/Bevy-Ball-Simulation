use bevy::prelude::*;

pub const SCREENSIZE: Vec3 = Vec3::new(1280.0, 720.0, 0.0);

pub const HALF_DIM: Vec3 = Vec3::new(640.0, 360.0, 0.0);

pub const CHUNK_SIZE: i32 = 4000;

pub const GRAVITY: Vec3 = Vec3::new(0.0, 98.0, 0.0); // increased it by 10x

pub const ITERATION_COUNT : i32 = 10;
pub const ITERATION_DELTA : f32 = 0.1; 