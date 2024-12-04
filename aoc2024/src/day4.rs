pub fn part1(input: &str) -> u64 {
    let map_height = input.lines().count();
    let map_width = input.lines().next().unwrap().trim().len();
    let map: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let map_config = MapConfig {
        width: map_width,
        height: map_height,
    };

    let mut rs = 0;
    let mut current_coord = Some((0, 0));
    while let Some(coord) = current_coord {
        rs += check_xmas(&coord, &map, &map_config);
        current_coord = get_next_coord(&coord, &map_config);
    }
    rs
}

pub fn part2(input: &str) -> u64 {
    let map_height = input.lines().count();
    let map_width = input.lines().next().unwrap().trim().len();
    let map: Vec<char> = input.lines().flat_map(|line| line.chars()).collect();
    let map_config = MapConfig {
        width: map_width,
        height: map_height,
    };

    let mut rs = 0;
    let mut current_coord = Some((0, 0));
    while let Some(coord) = current_coord {
        rs += check_mas(&coord, &map, &map_config);
        current_coord = get_next_coord(&coord, &map_config);
    }
    rs
}

struct MapConfig {
    width: usize,
    height: usize,
}

fn check_mas(coord: &(i64, i64), map: &[char], map_config: &MapConfig) -> u64 {
    let (x, y) = coord;
    let initial_char = get_at(coord, map, map_config).unwrap();

    if initial_char != 'A' {
        return 0;
    }

    let diag1 = &[
        get_at(&(x - 1, y - 1), map, map_config),
        get_at(coord, map, map_config),
        get_at(&(x + 1, y + 1), map, map_config),
    ];
    let diag2 = &[
        get_at(&(x - 1, y + 1), map, map_config),
        get_at(coord, map, map_config),
        get_at(&(x + 1, y - 1), map, map_config),
    ];
    if is_mas_or_sam(diag1) && is_mas_or_sam(diag2) {
        1
    } else {
        0
    }
}
fn check_xmas(coord: &(i64, i64), map: &[char], map_config: &MapConfig) -> u64 {
    let mut result = 0;
    let (x, y) = coord;
    let initial_char = get_at(coord, map, map_config).unwrap();

    if initial_char != 'X' && initial_char != 'S' {
        return 0;
    }

    let diag1 = &[
        get_at(coord, map, map_config),
        get_at(&(x + 1, y - 1), map, map_config),
        get_at(&(x + 2, y - 2), map, map_config),
        get_at(&(x + 3, y - 3), map, map_config),
    ];
    let horizontal = &[
        get_at(coord, map, map_config),
        get_at(&(x + 1, *y), map, map_config),
        get_at(&(x + 2, *y), map, map_config),
        get_at(&(x + 3, *y), map, map_config),
    ];
    let diag2 = &[
        get_at(coord, map, map_config),
        get_at(&(x + 1, y + 1), map, map_config),
        get_at(&(x + 2, y + 2), map, map_config),
        get_at(&(x + 3, y + 3), map, map_config),
    ];
    let vertical = &[
        get_at(coord, map, map_config),
        get_at(&(*x, y - 1), map, map_config),
        get_at(&(*x, y - 2), map, map_config),
        get_at(&(*x, y - 3), map, map_config),
    ];
    if is_xmas_or_samx(diag1) {
        result += 1;
    }
    if is_xmas_or_samx(horizontal) {
        result += 1;
    }
    if is_xmas_or_samx(diag2) {
        result += 1;
    }
    if is_xmas_or_samx(vertical) {
        result += 1;
    }
    result
}

fn is_xmas_or_samx(input: &[Option<char>]) -> bool {
    let Some(message) = input.iter().try_fold(String::new(), |mut a, b| {
        let Some(b) = b else { return None };
        a.push(*b);
        Some(a)
    }) else {
        return false;
    };
    message == "XMAS" || message == "SAMX"
}

fn is_mas_or_sam(input: &[Option<char>]) -> bool {
    let Some(message) = input.iter().try_fold(String::new(), |mut a, b| {
        let Some(b) = b else { return None };
        a.push(*b);
        Some(a)
    }) else {
        return false;
    };
    message == "MAS" || message == "SAM"
}

fn get_next_coord(coord: &(i64, i64), map: &MapConfig) -> Option<(i64, i64)> {
    let (x, y) = coord;
    if x + 1 < map.width as i64 {
        Some((x + 1, *y))
    } else if y + 1 < map.height as i64 {
        Some((0, y + 1))
    } else {
        None
    }
}

fn get_at(coord: &(i64, i64), map: &[char], map_config: &MapConfig) -> Option<char> {
    let (x, y) = coord;
    if *x >= map_config.width as i64 || *x < 0 || *y < 0 || *y >= map_config.height as i64 {
        None
    } else {
        let pos: Result<usize, _> = ((map_config.height as i64 * y) + x).try_into();

        pos.ok().map(|p| map[p])
    }
}
