use ordered_float::NotNan;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A point in a 2D co-ordinite plane
#[derive(Debug, Eq, PartialEq)]
pub struct Point(pub i64, pub i64);

impl Point {
    /// Finds the distance from another point.
    fn distance_from(&self, other: &Point) -> NotNan<f64> {
        return NotNan::new(f64::sqrt(
            ((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f64,
        ))
        .unwrap();
    }
}

/// Util struct that associates a Point with it's distannce to the origin
#[derive(Debug, Eq, PartialEq)]
struct PointDistance<'a> {
    point: &'a Point,
    distance: NotNan<f64>,
}

impl Ord for PointDistance<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        return other.distance.cmp(&self.distance).reverse();
    }
}

impl PartialOrd for PointDistance<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Returns the K nearest points to the given origin in order
/// # Example
/// ```
/// use rust_exercises::k_nearest_points::{Point, get_k_nearest_points};
///
/// // Given
/// const ORIGIN_POINT: Point = Point(0, 0);
/// let input_points = vec![
///     &Point(0, 0),
///     &Point(0, 1),
///     &Point(-1, 0),
///     &Point(1,3),
///     &Point(0, 10)
/// ];
///
/// // When
/// let restult = get_k_nearest_points(&ORIGIN_POINT, input_points, 3);
///
/// // Then
/// assert_eq!(restult, vec![&Point(0, 0), &Point(0, 1), &Point(-1, 0)]);
/// ```
pub fn get_k_nearest_points<'a>(
    origin_point: &Point,
    points: Vec<&'a Point>,
    k: usize,
) -> Vec<&'a Point> {
    if k > points.len() {
        // Validate that there are enough points to satisfy the query
        panic!("Not enough points, k = {k}, lenght = {}", points.len());
    }

    // Gether the point distances into a Binary Heap
    let point_distances: BinaryHeap<PointDistance> = points
        .iter()
        .map(|p| PointDistance {
            point: p,
            distance: origin_point.distance_from(p),
        })
        .collect();

    // Collects the K nearest points
    let k_nearest_points: Vec<&Point> = point_distances
        .into_sorted_vec()
        .into_iter()
        .take(k)
        .map(|point_distance| point_distance.point)
        .collect();

    return k_nearest_points;
}
