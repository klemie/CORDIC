/**
 * @file utils.rs
 * @brief Utility functions and types for CORDIC.
 */

// Currently building for a 32bit system
pub const STEPS: usize = 32;

static mut DELTA: [Union; STEPS] = [Union {float: 0.0, binary: [0; STEPS]}; STEPS];
static mut THETA: [Union; STEPS] = [Union {float: 0.0, binary: [0; STEPS]}; STEPS];
static mut K: [Union; STEPS] = [Union {float: 0.0, binary: [0; STEPS]}; STEPS];

#[derive(Copy, Clone)]
pub struct Union {
    pub float: f32,
    pub binary: [u32; STEPS]
}

enum CordicType {
    Circular,
    Linear,
    Hyperbolic
}

pub struct CordicResult {
    cordic_type: CordicType,
    rotation_angle: Union,
    shift_angle: Union,
    scale_factor: Union,
    x: Union,
    y: Union,
    z: Union
}


enum M {
    Circular = 1,
    Linear = 0,
    Hyperbolic = -1
}

/// 
/// returns the (x, y, z) for the given number of STEPS.
/// 
/// # Examples
/// ```
/// let (x, y, z) = circular_equations(32, 1.0, 0.0, 0.6072529350088812561694);
/// 
/// ```
/// 
pub fn general_equations(n: usize, cordic_type: CordicType, delta: Union, mu: Union) -> (usize, usize) {
    const K: f32 = 0.6072529350088812561694;
    let m: i32 = match cordic_type {
        Circular => 1,
        Linear => 0,
        Hyperbolic => -1
    };


    let (mut x: Union, mut y: Union, mut z: Union) = (
        Union {float: 1.0, binary: [0; STEPS]},
        Union {float: 0.0, binary: [0; STEPS]},
        Union {float: 0.0, binary: [0; STEPS]}
    );

    for i in 0..n {
        (x, y, z) = ()
    }
}


#[inline(always)]
fn theta_angle() -> &'static [Union] {
   unsafe {
       &THETA
   }
}

#[inline(always)]
fn scale_factor() -> &'static [Union] {
   unsafe {
       &K
   }
}

#[inline(always)]
fn delta_angle() -> &'static [Union] {
   unsafe {
       &DELTA
   }
}

pub fn scale_factor() ->  &'static Union {
    /*
    * 
    */
    let k = 
    
}

pub fn shift_angle(cordic: CordicType) -> &'static Union {
    /*
    * Delta = 
    */
    let delta = 
} 

/**
 * @brief Calculates the rotation angle for the given number of STEPS.
* 
* @param cordic The type of CORDIC to calculate the rotation angle for.
*/
pub fn rotation_angle(cordic: CordicType) -> &'static Union {
    /*
    * theta = sum (2^-i)upper: n-1 lower: i=0
    */ 

}