#[cfg(test)]
mod specs {

    struct A {}

    fn foo(arg: A) -> i32 {
        3
    }

    #[test]
    fn move_in_maps() {
        let inputs = vec![A {}];
        let output = inputs.into_iter().map(|item| foo(item));
    }
}
