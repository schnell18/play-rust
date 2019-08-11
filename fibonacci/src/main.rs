use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: fibonacci NUMBER ... ").unwrap();
        std::process::exit(1);
    }
    for m in &numbers {
        println!("fib({}) = {}",  m, fib(*m));
    }
}

fn fib(n: u64) -> u64 {

    // if n <=  1 {
    //     return 1;
    // }

    let mut a = 1;
    let mut b = 1;
    for _i in 1..n {
        let t = a + b;
        a = b;
        b = t;
    }
    b
}

#[test]
fn test_fib() {
    assert_eq!(fib(0), 1);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 2);
    assert_eq!(fib(3), 3);
    assert_eq!(fib(4), 5);
    assert_eq!(fib(5), 8);
    assert_eq!(fib(6), 13);
    assert_eq!(fib(7), 21);
    assert_eq!(fib(8), 34);
    assert_eq!(fib(9), 55);
    assert_eq!(fib(10), 89);

}
