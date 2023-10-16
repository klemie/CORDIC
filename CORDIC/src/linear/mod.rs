/**
 * @file linear.rs
 * @brief Linear CORDIC functions. Generates 32bit precision of division and multiplication. 
 */
// import steps from utils.rs

pub mod LinearCordic {
    use crate::utils::{Union, STEPS};

    fn generate_delta(step: i32) -> f32 {
        /*
         * delta_i = arctan(2^-i) 
         *   right shift by step
         */

        let i: i32 = -1*step;
        return (2u32 as f32).powi(i);
    }

    fn generate_theta(step: i32) -> f32 {
        /*
         * theta_i = delta_i
         */
        return generate_delta(step);
    }

    fn generate_mu(y: f32) -> i8 {
        /*
         * mu_i = 
         *  -1 when y_i > 0  
         *  +1 when y_i < 0
         */
        let mu: i8 = match y {
            y if y > 0.0 => 1,
            y if y < 0.0 => -1,
            _ => 0
        };
        return mu;
    }

    pub fn linear_cordic(num_steps: u32) {
        let (mut x, mut y, mut z) = (
            Union {float: 1.0, binary: [0; STEPS]},
            Union {float: 1.0, binary: [0; STEPS]},
            Union {float: 0.0, binary: [0; STEPS]}
        );

        let (mut delta, mut theta, mut mu) = (generate_delta(0), generate_delta(0), generate_delta(0));

        for i in 0..num_steps {
            delta = generate_delta(i);
            theta = generate_theta(i);
            mu = generate_mu(y);
            (x, y, z) = (
                x - mu*y*delta, 
                y + mu*x*delta, 
                z - mu*theta
            );
        }
        println!("(x: {}, y: {}, z: {})", x, y, z);
    }
}