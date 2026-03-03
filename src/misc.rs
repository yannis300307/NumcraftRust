#[inline(always)]
pub fn div_floor(a: isize, b: isize) -> isize {
    if a >= 0 
    {
        a / b
    }
    else
    {
        (a - b + 1) / b
    }
}

#[inline(always)]
pub fn mod_floor(a: isize, b: isize) -> isize
{
    if a >= 0 
    {
        a % b
    }
    else
    {
        (a - b + 1) % b
    }
}
