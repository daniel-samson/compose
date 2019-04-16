#[allow(unused_macros)]
#[allow(dead_code)]
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

#[allow(dead_code)]
fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let add = |x| x + 2;
        let multiply = |x| x * 2;
        let divide = |x| x / 2;
        let calc = compose!(add, multiply, divide);
        assert_eq!(calc(10), 12)
    }
}
