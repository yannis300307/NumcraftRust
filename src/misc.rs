pub fn div_floor(a: isize, b: isize) -> isize {
    let (d, r) = (a / b, a % b);
    if (r != 0) && ((r < 0) != (b < 0)) {
        d - 1
    } else {
        d
    }
}