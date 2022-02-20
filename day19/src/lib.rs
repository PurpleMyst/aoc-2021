use std::collections::HashMap;
use std::fmt::Display;
use std::ops::ControlFlow;

use ahash::RandomState;
use indexmap::IndexSet;

type Scalar = i16;
type Point = nalgebra::Vector3<Scalar>;
type Transformation = nalgebra::Matrix3<Scalar>;
type Transformations = [Transformation; 24];

type Report = IndexSet<Point, RandomState>;

type Fingerprint = (Scalar, Scalar, Scalar);
type Fingerprints = HashMap<Fingerprint, Vec<(Point, Point)>, RandomState>;

fn fingerprints(report: &Report) -> Fingerprints {
    let mut result = Fingerprints::default();
    for (i, &p1) in report.iter().enumerate() {
        for &p2 in report.iter().skip(i + 1) {
            let abs_diff = (p2 - p1).abs();
            let fp = (abs_diff.sum(), abs_diff.max(), abs_diff.min());
            result.entry(fp).or_default().push((p1, p2));
        }
    }

    result
}

struct Solver {
    transformations: Transformations,
    known_beacons: Report,
    known_fingerprints: Fingerprints,

    scanner_positions: Vec<Point>,
}

impl Solver {
    fn new(known_beacons: Report) -> Self {
        let mut transformations = [Transformation::zeros(); 24];
        let mut it = transformations.iter_mut();

        for i in 0..3 {
            for i_sgn in [1, -1] {
                for j in 0..3 {
                    for j_sgn in [1, -1] {
                        if i == j {
                            continue;
                        }
                        let mut iv = Point::zeros();
                        iv[i] = i_sgn;
                        let mut jv = Point::zeros();
                        jv[j] = j_sgn;
                        let kv = iv.cross(&jv);
                        *it.next().unwrap() = Transformation::from_columns(&[iv, jv, kv]);
                    }
                }
            }
        }

        let scanner_positions = vec![];
        let known_fingerprints = fingerprints(&known_beacons);
        Self {
            transformations,
            known_beacons,
            known_fingerprints,
            scanner_positions,
        }
    }

    fn best_fit(&mut self, candidate: &Report) -> ControlFlow<()> {
        let candidate_fps = fingerprints(candidate);
        let common_fps: Vec<_> = candidate_fps
            .keys()
            .filter(|k| self.known_fingerprints.contains_key(k))
            .collect();

        if common_fps.len() < 66 {
            return ControlFlow::Continue(());
        }

        for k in common_fps {
            let known_pairs = &self.known_fingerprints[k];
            let candidate_pairs = &candidate_fps[k];

            for known_pair in known_pairs {
                for candidate_pair in candidate_pairs {
                    if let Some((m, translation)) = self.transformations.iter().find_map(|m| {
                        let translation0 = known_pair.0 - m * candidate_pair.0;
                        let translation1 = known_pair.1 - m * candidate_pair.1;
                        (translation0 == translation1).then(|| (m, translation0))
                    }) {
                        let translated_report = candidate.iter().map(|b| m * b + translation);

                        self.scanner_positions.push(translation);
                        self.known_beacons.extend(translated_report);
                        self.known_fingerprints = fingerprints(&self.known_beacons); // TODO: only recalculate as needed

                        return ControlFlow::Break(());
                    }
                }
            }
        }

        ControlFlow::Continue(())
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display) {
    let mut unknown_scanners: Vec<Report> = include_str!("input.txt")
        .split("\n\n")
        .map(|report| {
            report
                .lines()
                .skip(1)
                .map(|point| Point::from_iterator(point.split(',').map(|c| c.parse().unwrap())))
                .collect()
        })
        .collect();

    let known_beacons = unknown_scanners.pop().unwrap();
    let mut solver = Solver::new(known_beacons);

    while !unknown_scanners.is_empty() {
        for (idx, scanner) in unknown_scanners.iter().enumerate() {
            if matches!(solver.best_fit(scanner), ControlFlow::Break(())) {
                unknown_scanners.swap_remove(idx);
                break;
            }
        }
    }

    let mut part2 = 0;
    for (i, p1) in solver.scanner_positions.iter().enumerate() {
        for p2 in solver.scanner_positions.iter().skip(i + 1) {
            part2 = part2.max((p2 - p1).abs().sum());
        }
    }

    (solver.known_beacons.len(), part2)
}
