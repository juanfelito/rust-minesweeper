use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Tile {
    pub value: TileValue,
    pub status: TileStatus,
    pub coords: (usize, usize)
}

#[derive(Debug, PartialEq)]
pub enum TileStatus {
    CLOSED,
    OPENED,
    FLAGGED,
    QUESTION,
}

#[derive(Debug, PartialEq)]
pub enum TileValue {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    MINE
}

impl TileValue {
    pub fn to_png(&self) -> String {
        match self {
            Self::ZERO => "zero.png".to_string(),
            Self::ONE => "one.png".to_string(),
            Self::TWO => "two.png".to_string(),
            Self::THREE => "three.png".to_string(),
            Self::FOUR => "four.png".to_string(),
            Self::FIVE => "five.png".to_string(),
            Self::SIX => "six.png".to_string(),
            Self::SEVEN => "seven.png".to_string(),
            Self::EIGHT => "eight.png".to_string(),
            Self::MINE => "bomb.png".to_string(),
        }
    }
}
