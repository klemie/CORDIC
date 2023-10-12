/**
 * @file utils.rs
 * @brief Utility functions and types for CORDIC.
 */

 // Currently building for a 32bit system
const ITERATIONS: u8 = 32;

enum CordicType {
    Circular,
    Linear,
    Hyperbolic
}

struct Union {
    float: f32,
    binary: [u32; ITERATIONS]
}

struct CordicResult {
    cordic_type: CordicType,
    angle: Union,
    x: Union,
    y: Union,
    z: Union
}

fn shift_angle(num_iterations: u32, cordic: CordicType) -> Union {
    /*
    * Delta = 
    */
} 

fn rotation_angle(num_iterations: u32, cordic: CordicType) -> Union {
    /*
    * theta = sum (2^-i)upper: n-1 lower: i=0
    */ 
}
