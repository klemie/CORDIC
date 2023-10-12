/**
 * @file linear.rs
 * @brief Hyperbolic CORDIC functions. Generates 32bit precision of division and multiplication. 
 */

use crate::utils::{CordicResult, CordicType, Union};

mod hyperbolic {
    pub fn sinh(x: f32) -> f32 {
        x.sinh();
    }

    pub fn cosh(x: f32) -> f32 {
        x.cosh();
    }

    pub fn tanh(x: f32) -> f32 {
        x.tanh();
    }
}

mod hyperbolicCordic {
    
}