pub(crate) struct MapConfig {
    pub width: usize,
    pub height: usize,
}

pub type Coord = (i64, i64);

#[allow(dead_code)]
pub(crate) fn get_next_coord(coord: &(i64, i64), map: &MapConfig) -> Option<(i64, i64)> {
    let (x, y) = coord;
    if x + 1 < map.width as i64 {
        Some((x + 1, *y))
    } else if y + 1 < map.height as i64 {
        Some((0, y + 1))
    } else {
        None
    }
}

pub(crate) fn get_at<T: Clone>(coord: &(i64, i64), map: &[T], map_config: &MapConfig) -> Option<T> {
    let (x, y) = coord;
    if *x >= map_config.width as i64 || *x < 0 || *y < 0 || *y >= map_config.height as i64 {
        None
    } else {
        let pos: Result<usize, _> = ((map_config.height as i64 * y) + x).try_into();

        pos.ok().map(|p| map[p].clone())
    }
}

pub(crate) fn index_to_coord(index: usize, map_config: &MapConfig) -> Coord {
    let y = index / map_config.width;
    let x = index % map_config.width;
    (x as i64, y as i64)
}

pub(crate) fn coord_to_index(coord: Coord, map_config: &MapConfig) -> usize {
    let (x, y) = coord;
    (y * map_config.width as i64 + x).try_into().unwrap()
}
