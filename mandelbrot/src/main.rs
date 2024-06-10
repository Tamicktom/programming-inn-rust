use num::Complex;

fn main() {
    let c = Complex {re: 2., im: 0.};
    match escape_time(c, 100) {
        Some(count) => println!("Escaped after {} iterations", count),
        None => println!("Didn't escape")
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

fn escape_time(c:Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex {re: 0., im: 0.};

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    return None;
}