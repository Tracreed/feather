use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

/// The velocity of an entity.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Pod, Zeroable)]
#[repr(C)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Velocity {
    fn default() -> Velocity {
        Velocity {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
