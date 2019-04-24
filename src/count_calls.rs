#[allow(dead_code)]
fn count_calls<S, F>(s: S, mut f: F) -> u32
where
    S: Clone,
    // P: Sized + Fn() -> S,
    F: FnMut(&mut dyn FnMut() -> S) -> (),
{
    let mut counter: u32 = 0;

    f(&mut || {
        counter += 1;
        s.clone()
    });
    counter
}

#[cfg(test)]
mod stackoverflow {
    use super::*;

    fn f(p: &mut FnMut() -> i32) {
        p();
        p();
        p();
    }

    #[test]
    fn test() {
        let counts = count_calls(3, f);
        assert_eq!(counts, 3);
    }
}
