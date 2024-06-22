use num::Complex;
use std::str::FromStr;

fn main() {
    let c = Complex { re: 2., im: 0. };
    match escape_time(c, 100) {
        Some(count) => println!("Escaped after {} iterations", count),
        None => println!("Didn't escape"),
    }
}

// fn square_loop(mut x: f64){
//     loop{
//         x = x * x;
//         println!("{}", x);
//     }
// }

// fn square_add_loop(c: f64){
//     let mut x = 0.;
//     loop {
//         x = x* x + c;
//         println!("{}", x);
//     }
// }

// fn complex_square_add_loop(c:Complex<f64>){
//     let mut z = Complex {re: 0., im: 0.};
//     loop {
//         z = z * z + c;
//         println!("{}, {}", z.re, z.im);
//     }
// }

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0., im: 0. };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    return None;
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
}
