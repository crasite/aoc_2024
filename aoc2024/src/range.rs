#![allow(dead_code)]
type Range<T> = (T, T);

fn intersect<T: PartialOrd>(r1: Range<T>, r2: Range<T>) -> Option<Range<T>> {
    let (x1, y1) = r1;
    let (x2, y2) = r2;
    if y1 < x2 || x1 > y2 {
        return None;
    }
    let x = if x1 < x2 { x2 } else { x1 };
    let y = if y1 > y2 { y2 } else { y1 };
    Some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_intersect() {
        let cases = vec![
            ((-10, 10), (-12, 3), Some((-10, 3))),
            ((-10, 1), (1, 3), Some((1, 1))),
            ((1, 3), (-10, 1), Some((1, 1))),
            ((-10, 1), (2, 3), None),
            ((2, 3), (-10, 1), None),
        ];
        for case in cases {
            assert_eq!(intersect(case.0, case.1), case.2);
        }
    }
}
