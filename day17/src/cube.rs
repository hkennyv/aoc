#[derive(Debug)]
pub struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}
#[derive(Debug)]
struct Cube {
    active: bool,
    coordinate: Coordinate,
}

#[derive(Debug)]
pub struct Space {
    bounds: Vec<Coordinate>,
    grid: Vec<Vec<Vec<i32>>>,
}
