/**
 * @file linear.rs
 * @brief Linear CORDIC functions. Generates 32bit precision of division and multiplication. 
 */

use crate::utils::{CordicResult, CordicType, Union};

mod linear {
    pub fn multiply(x: f32, y: f32) -> f32 {
        x * y;
    }

    pub fn divide(x: f32, y: f32) -> f32 {
        x / y;
    }
}

mod linearCordic {
    
}