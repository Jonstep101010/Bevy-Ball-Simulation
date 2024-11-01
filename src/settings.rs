use bevy::prelude::*;

pub const SCREENSIZE: Vec3 = Vec3::new(1280.0, 680.0, 0.0);

pub const HALF_DIM: Vec3 = Vec3::new(SCREENSIZE.x / 2.0, SCREENSIZE.y / 2.0, 0.0);

pub const BALL_SIZE: f32 = 10.0;
pub const CHUNK_SIZE: i32 = 10;


pub const GRAVITY: Vec3 = Vec3::new(0.0, 98.0, 0.0); // increased it by 10x

pub const ITERATION_COUNT : i32 = 10;
pub const ITERATION_DELTA : f32 = 0.1; 

pub const MOUSE_STRENGTH : f32 = 400.0;
pub const REMOVE_RADIUS_SQUARED : f32 = 2500.0;