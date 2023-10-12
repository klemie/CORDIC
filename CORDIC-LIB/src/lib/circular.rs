/**
 * @file circular.rs
 * @brief Circular CORDIC functions. Generates 32bit precision of sine and cosine. 
 */

use crate::utils::{CordicResult, CordicType, Union};

mod circular {
    pub fn sine(x: f32) -> f32 {
        x.sin();
    }

    pub fn cosine(x: f32) -> f32 {
        x.cos();
    }
}

mod circularCordic {
    
}