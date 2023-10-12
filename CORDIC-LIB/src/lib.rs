mod utils;
mod circular;
pub mod circularCordic;
mod hyperbolic;
pub mod hyperbolicCordic;
mod linear;
pub mod linearCordic;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
