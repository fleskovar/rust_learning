use pyo3::prelude::*;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct Point {
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

pub fn point_from_row(row: &Vec<f64>) -> Option<Point> {
    let n: usize = row.len();

    if n == 3 {
        Some(Point::new(row[0], row[1], row[2]))
    } else {
        None
    }
}
