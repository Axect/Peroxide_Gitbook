extern crate peroxide;
use peroxide::*;

fn main() {
    let a = Bernoulli(0.1);
    a.pdf(0).print();

    a.mean().print();
    a.var().print();
    a.sd().print();
}