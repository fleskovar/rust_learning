mod clusters;
mod points;

use clusters::{make_cluster, Cluster};
use points::{point_from_row, Point};

use ndarray::parallel::prelude::*;
use ndarray::Axis;
use numpy::PyArray2;
use pyo3::prelude::*;

#[pyfunction]
fn get_clusters_rayon(
    array: &PyArray2<f64>,
    mask: &PyArray2<i32>,
    tags: Vec<String>,
) -> PyResult<Vec<Cluster>> {
    let ro_array = array.readonly().to_owned_array();
    let ro_mask = mask.readonly().to_owned_array();

    let result: Vec<Cluster> = ro_mask
        .axis_iter(Axis(0))
        .into_par_iter()
        .zip(tags.par_iter())
        .map(|args| {
            make_cluster(
                args.0
                    .into_par_iter()
                    .filter_map(|&value| point_from_row(&ro_array.row(value as usize).to_vec()))
                    .collect::<Vec<_>>(),
                args.1.to_owned(),
            )
        })
        .collect();

    Ok(result)
}

#[pyfunction]
fn get_clusters(
    array: &PyArray2<f64>,
    mask: &PyArray2<i32>,
    tags: Vec<String>,
) -> PyResult<Vec<Cluster>> {
    let ro_array = array.readonly().to_owned_array();
    let ro_mask = mask.readonly().to_owned_array();

    let result: Vec<Cluster> = ro_mask
        .outer_iter()
        .zip(tags.iter())
        .map(|args| {
            make_cluster(
                args.0
                    .into_par_iter()
                    .filter_map(|&value| point_from_row(&ro_array.row(value as usize).to_vec()))
                    .collect::<Vec<_>>(),
                args.1.to_owned(),
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
