/// type alias defining a Tile to be a 2D vector of chars. all tiles _should_
/// have the same dimensions
pub type Tile = Vec<Vec<char>>;

fn rotate_tile(tile: &Tile) -> Tile {
    let new_length = tile[0].len();
    let new_width = tile.len();

    let mut rotated = vec![vec!['.'; new_width]; new_length];

    for i in 0..new_length {
        for j in 0..new_width {
            
        }
    }

    rotated
}
