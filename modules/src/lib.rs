pub fn my_add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn my_sub(left: i32, right: i32) -> i32 {
    left - right
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::my_add;
    use crate::my_sub;

    #[test]
    fn it_works() {
        let result = my_add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works2() {
       let result = my_sub(3, 2);
       assert_eq!(result, 1);
    }
}
