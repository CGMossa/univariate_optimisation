use std::fmt::Debug;
use std::ops::Range;

fn main() {
    let objective = |x: f64| x.ln() / (1f64 + x);

    let diff_objective = |x: f64| (1f64 + 1f64 / x - x.ln()) / ((1f64 + x).powi(2));

    let start_value: f64;
}

fn bisection_method<T, F1, F2>(objective: F1, diff_objective: F2, domain: Range<T>)
where
    T: Debug + Clone,
    F1: Fn(T) -> T,
    F2: Fn(T) -> T,
{
    dbg!(&domain);

    let a = domain.start;
    let b = domain.end;

    //    let new_x;
    //    let new_a;
    //    let new_b;
    //
    //    //    (new_a, new_b) = match (diff_objective(a) * diff_objective(x)) {};
    //

    //    new_a + new_b);

    assert!(false);
}

#[test]
fn cold_call_bisection_method() {
    bisection_method(|_| 0.1, |_| 0.1, 1.2..4.3);
}
