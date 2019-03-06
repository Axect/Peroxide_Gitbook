extern crate peroxide;
use peroxide::*;

fn main() {
    let a = c!(1,2,3,4);
    let b = c!(5,6,7,8);

    // Original rust
    a.clone()
        .into_iter()
        .zip(&b)
        .map(|(x, y)| x + *y)
        .collect::<Vec<f64>>().print();

    a.zip_with(|x, y| x + y, &b).print();
}