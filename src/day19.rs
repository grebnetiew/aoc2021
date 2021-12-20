use euclid::Angle;
type Point3D = euclid::default::Point3D<i32>;
type Vector3D = euclid::default::Vector3D<i32>;
type Rotation3D = euclid::default::Rotation3D<f64>;

#[aoc_generator(day19)]
pub fn parse_scanners(input: &str) -> Vec<Vec<Point3D>> {
    let mut scanners = Vec::new();
    let input = input.lines().filter(|line| !line.is_empty());
    for line in input {
        if line.starts_with("---") {
            scanners.push(Vec::new());
        } else {
            let mut p = line.split(',');
            scanners.last_mut().unwrap().push(Point3D::from([
                p.next().map(str::parse).unwrap().unwrap(),
                p.next().map(str::parse).unwrap().unwrap(),
                p.next().map(str::parse).unwrap().unwrap(),
            ]));
        }
    }
    scanners
}

#[aoc(day19, part1)]
pub fn part1(input: &[Vec<Point3D>]) -> usize {
    let mut input = input.to_owned();
    let mut reference = input.remove(0);
    while !input.is_empty() {
        let mut ok = false;
        for i in 0..input.len() {
            if let Some((rot, vec)) = has_overlap_rot(&reference, &input[i]) {
                let transformed: Vec<_> = input
                    .swap_remove(i)
                    .iter()
                    .map(|p| rot.transform_point3d(p.cast()).round().cast() + vec)
                    .collect();
                reference.extend_from_slice(&transformed);
                reference
                    .sort_unstable_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)).then(a.z.cmp(&b.z)));
                reference.dedup();
                ok = true;
                break;
            }
        }
        if !ok {
            panic!("will not converge");
        }
    }
    reference.len()
}

#[aoc(day19, part2)]
pub fn part2(input: &[Vec<Point3D>]) -> i32 {
    // Maximum manhattan distance between sensors
    let mut input = input.to_owned();
    let mut scanner_pos = Vec::new();
    let mut reference = input.remove(0);
    while !input.is_empty() {
        for i in 0..input.len() {
            if let Some((rot, vec)) = has_overlap_rot(&reference, &input[i]) {
                scanner_pos.push(vec);
                let transformed: Vec<_> = input
                    .swap_remove(i)
                    .iter()
                    .map(|p| rot.transform_point3d(p.cast()).round().cast() + vec)
                    .collect();
                reference.extend_from_slice(&transformed);
                reference
                    .sort_unstable_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)).then(a.z.cmp(&b.z)));
                reference.dedup();
                break;
            }
        }
    }

    (0..scanner_pos.len())
        .flat_map(|i| (0..scanner_pos.len()).map(move |j| (i, j)))
        .map(|(i, j)| {
            (scanner_pos[i] - scanner_pos[j])
                .abs()
                .to_array()
                .iter()
                .sum()
        })
        .max()
        .unwrap()
}

/// Brute forces all differences between points to see if at least 12 points
/// overlap. If so, returns the position of `candidate`'s origin in `reference`'s
/// coordinate system. Does not take rotations into account.
fn has_overlap(reference: &[Point3D], candidate: &[Point3D]) -> Option<Vector3D> {
    for can in candidate {
        for i in 0..reference.len() {
            let v = *can - reference[i];
            let overlaps = reference
                .iter()
                .filter(|&p| candidate.contains(&(*p + v)))
                .count();
            if overlaps >= 12 {
                return Some(-v);
            }
        }
    }
    None
}

/// Brute forces `has_overlap` between the two points, for all 24 possible
/// rotations and orientations of candidate. If there is an overlap, the return
/// values are how the candidate should be rotated and where it should be placed
/// in the reference coordinate system for overlap to occur.
fn has_overlap_rot(reference: &[Point3D], candidate: &[Point3D]) -> Option<(Rotation3D, Vector3D)> {
    let (r000, r090, r180, r270) = (
        Angle::zero(),
        Angle::frac_pi_2(),
        Angle::pi(),
        Angle::frac_pi_2() * 3.0,
    );
    let angles = vec![
        Rotation3D::euler(r000, r000, r000),
        Rotation3D::euler(r090, r000, r000),
        Rotation3D::euler(r180, r000, r000),
        Rotation3D::euler(r270, r000, r000),
        Rotation3D::euler(r000, r000, r090),
        Rotation3D::euler(r090, r000, r090),
        Rotation3D::euler(r180, r000, r090),
        Rotation3D::euler(r270, r000, r090),
        Rotation3D::euler(r000, r000, r180),
        Rotation3D::euler(r090, r000, r180),
        Rotation3D::euler(r180, r000, r180),
        Rotation3D::euler(r270, r000, r180),
        Rotation3D::euler(r000, r000, r270),
        Rotation3D::euler(r090, r000, r270),
        Rotation3D::euler(r180, r000, r270),
        Rotation3D::euler(r270, r000, r270),
        Rotation3D::euler(r000, r090, r000),
        Rotation3D::euler(r090, r090, r000),
        Rotation3D::euler(r180, r090, r000),
        Rotation3D::euler(r270, r090, r000),
        Rotation3D::euler(r000, r270, r000),
        Rotation3D::euler(r090, r270, r000),
        Rotation3D::euler(r180, r270, r000),
        Rotation3D::euler(r270, r270, r000),
    ];

    for a in angles {
        let rotated: Vec<_> = candidate
            .iter()
            .map(|p| a.transform_point3d(p.cast()).round().cast())
            .collect();
        if let Some(position) = has_overlap(reference, &rotated) {
            return Some((a, position));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    const SHORTER_TEST: &str = "--- scanner 0 ---
-618,-824,-621
-537,-823,-458
-447,-329,318
404,-588,-901
544,-627,-890
528,-643,409
-661,-816,-575
390,-675,-793
423,-701,434
-345,-311,381
459,-707,401
-485,-357,347

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
-476,619,847
-460,603,-452
729,430,532
-322,571,750
-355,545,-477
413,935,-424
-391,539,-444
553,889,-390";

    #[test]
    fn rotations() {
        let (r000, _r090, r180, _r270) = (
            Angle::<f64>::zero(),
            Angle::<f64>::frac_pi_2(),
            Angle::<f64>::pi(),
            Angle::<f64>::frac_pi_2() * 3.0,
        );
        let pts = vec![
            Point3D::from([0; 3]),
            Point3D::from([1; 3]),
            Point3D::from([2; 3]),
            Point3D::from([3; 3]),
            Point3D::from([4; 3]),
            Point3D::from([5; 3]),
            Point3D::from([6; 3]),
            Point3D::from([7; 3]),
            Point3D::from([8; 3]),
            Point3D::from([9; 3]),
            Point3D::from([10; 3]),
            Point3D::from([11; 3]),
            Point3D::from([12; 3]),
        ];
        let pts1: Vec<_> = pts
            .iter()
            .map(|p| Point3D::from((p.x, -p.y, -p.z)) + Vector3D::from((1, -1, -1)))
            .collect();
        assert_eq!(
            has_overlap_rot(&pts, &pts1),
            Some((
                Rotation3D::euler(r180, r000, r000),
                Vector3D::from((-1, -1, -1))
            ))
        );
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_scanners(TEST_INPUT)), 79);
    }
    #[test]
    fn sample1a() {
        let input = parse_scanners(SHORTER_TEST);
        assert!(has_overlap_rot(&input[0], &input[1]).is_some());
    }
    #[test]
    fn sample1b() {
        assert_eq!(part1(&parse_scanners(SHORTER_TEST)), 12);
    }
    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_scanners(TEST_INPUT)), 3621);
    }
}
