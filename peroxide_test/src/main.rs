extern crate peroxide;
use peroxide::*;

fn main() {
    let a = c!(1,2,3,4);
    let m_col = matrix(c!(1,2,3,4), 4, 1, Col);
    assert_eq!(a.to_matrix(), m_col);

    let m_row = matrix(c!(1,2,3,4), 1, 4, Row);
    assert_eq!(a.transpose(), m_row);
}
