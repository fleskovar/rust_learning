use ndarray::parallel::prelude::*;
use ndarray::Axis;
use numpy::PyArray2;
use pyo3::prelude::*;

#[pyclass(subclass)]
#[derive(Clone)]
struct Point {
    #[pyo3(get, set)]
    x: f64,

    #[pyo3(get, set)]
    y: f64,

    #[pyo3(get, set)]
    z: f64,
}

#[pymethods]
impl Point {
    #[new]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Point(x={}, y='{}', z='{}')",
            self.x, self.y, self.z
        ))
    }
}

fn point_from_row(row: &Vec<f64>) -> Option<Point> {
    let n: usize = row.len();

    if n == 3 {
        Some(Point::new(row[0], row[1], row[2]))
    } else {
        None
    }
}

#[pyclass(subclass)]
struct Cluster {
    #[pyo3(get, set)]
    points: Vec<Point>,
}

#[pymethods]
impl Cluster {
    #[new]
    fn new(points: Vec<Point>) -> Self {
        Cluster { points }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Cluster(size: {}')", self.points.len()))
    }
}

#[pyfunction]
fn get_clusters_rayon(array: &PyArray2<f64>, mask: &PyArray2<i32>) -> PyResult<Vec<Cluster>> {
    let ro_array = array.readonly().to_owned_array();
    let ro_mask = mask.readonly().to_owned_array();

    let result: Vec<Cluster> = ro_mask
        .axis_iter(Axis(0))
        .into_par_iter()
        .map(|row| {
            Cluster::new(
                row.into_par_iter()
                    .filter_map(|&value| point_from_row(&ro_array.row(value as usize).to_vec()))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    Ok(result)
}

#[pyfunction]
fn get_clusters(array: &PyArray2<f64>, mask: &PyArray2<i32>) -> PyResult<Vec<Cluster>> {
    let ro_array = array.readonly().to_owned_array();
    let ro_mask = mask.readonly().to_owned_array();

    let result: Vec<Cluster> = ro_mask
        .outer_iter()
        .map(|row| {
            Cluster::new(
                row.iter()
                    .filter_map(|&value| point_from_row(&ro_array.row(value as usize).to_vec()))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn new_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_clusters, m)?)?;
    m.add_function(wrap_pyfunction!(get_clusters_rayon, m)?)?;
    m.add_class::<Cluster>()?;
    m.add_class::<Point>()?;

    Ok(())
}
