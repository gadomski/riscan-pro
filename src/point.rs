#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T, N>
    where N: Copy
{
    pub crs: T,
    pub x: N,
    pub y: N,
    pub z: N,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRCS;
