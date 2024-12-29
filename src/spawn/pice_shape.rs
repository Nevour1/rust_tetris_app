//! This module declares the shapes of each pice.
//! Defines the shape of the tetris pice by using
//! a 4x5 "Matrix". The '1' is a filed filled with
//! a square the '0' is an empty files.

pub const PURPLE_SHAPE: [(u8,u8,u8,u8); 5] = [
  (1,1,1,1),
  (1,0,1,0),
  (0,0,0,0),
  (0,0,0,0),
  (0,0,0,0)
];

pub const BLUE_SHAPE: [(u8,u8,u8,u8); 5] = [
    (1,0,0,0),
    (1,0,0,0),
    (1,0,0,0),
    (1,0,0,0),
    (1,0,0,0)
];

pub const YELLOW_SHAPE: [(u8,u8,u8,u8); 5] = [
    (0,0,0,0),
    (0,0,1,0),
    (0,1,1,0),
    (0,1,0,0),
    (0,0,0,0)
];

pub const GREEN_SHAPE: [(u8,u8,u8,u8); 5] = [
    (1,1,1,0),
    (1,0,0,0),
    (0,0,0,0),
    (0,0,0,0),
    (0,0,0,0)
];

pub const RED_SHAPE: [(u8,u8,u8,u8); 5] = [
    (0,1,1,1),
    (0,0,0,1),
    (0,0,0,0),
    (0,0,0,0),
    (0,0,0,0)
];

pub const LILAC_SHAPE: [(u8,u8,u8,u8); 5] = [
    (0,0,0,0),
    (0,1,0,0),
    (0,1,1,0),
    (0,1,0,0),
    (0,0,0,0)
];

pub const ORANGE_SHAPE: [(u8,u8,u8,u8); 5] = [
    (0,0,0,0),
    (0,1,1,0),
    (0,1,1,0),
    (0,0,0,0),
    (0,0,0,0)
];

pub const PINK_SHAPE: [(u8,u8,u8,u8); 5] = [
    (0,0,0,0),
    (0,0,0,0),
    (0,0,1,0),
    (0,0,0,0),
    (0,0,0,0)
];