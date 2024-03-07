use std::str::FromStr;

use super::points::Point;
use pyo3::prelude::*;

#[pyclass(subclass)]
pub struct Cluster {
    #[pyo3(get, set)]
    points: Vec<Point>,
    #[pyo3(get, set)]
    tag: String,
}

#[pymethods]
impl Cluster {
    #[new]
    pub fn new(points: Vec<Point>, tag: String) -> Self {
        Cluster { points, tag }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Cluster(tag: {} size: {}')",
            self.tag,
            self.points.len()
        ))
    }
}

trait ClusterGenerator {
    fn generate_cluster(&self, points: Vec<Point>) -> Cluster;
}

pub struct ClusterGeneratorA;
pub struct ClusterGeneratorB;
pub struct ClusterGeneratorC;

impl ClusterGenerator for ClusterGeneratorA {
    fn generate_cluster(&self, points: Vec<Point>) -> Cluster {
        Cluster::new(points, String::from_str("A").unwrap())
    }
}

impl ClusterGenerator for ClusterGeneratorB {
    fn generate_cluster(&self, points: Vec<Point>) -> Cluster {
        Cluster::new(points, String::from_str("B").unwrap())
    }
}

impl ClusterGenerator for ClusterGeneratorC {
    fn generate_cluster(&self, points: Vec<Point>) -> Cluster {
        Cluster::new(points, String::from_str("C").unwrap())
    }
}

fn get_generator(tag: String) -> Box<dyn ClusterGenerator> {
    match tag.as_str() {
        "A" => Box::new(ClusterGeneratorA {}),
        "B" => Box::new(ClusterGeneratorB {}),
        "C" => Box::new(ClusterGeneratorC {}),
        _ => Box::new(ClusterGeneratorA {}),
    }
}

pub fn make_cluster(points: Vec<Point>, tag: String) -> Cluster {
    let gen = get_generator(tag);
    gen.generate_cluster(points)
}
