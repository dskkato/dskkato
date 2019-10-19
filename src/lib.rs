fn x2(n: i32) -> i32{
    2*n
}

#[no_mangle]
pub extern "C" fn hello(n: i32){
    println!("World at {0}", x2(n));
}

#[cfg(test)]
mod tests {
    use crate::x2;
    #[test]
    fn x2_test() {
        assert_eq!(x2(2), 4);
    }
}
