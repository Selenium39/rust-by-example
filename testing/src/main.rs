pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

pub fn panic_fnc() {
    panic!("gg")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    #[should_panic]
    fn test_panic_fnc() {
        panic_fnc()
    }
}

fn main() {}
