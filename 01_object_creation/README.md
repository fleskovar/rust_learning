## Intro

This examples explores instantiating objects that were serialized in csv. 

There are two main classes:

- Point: a simple 3D vector with x, y and z coordinates.
- Cluster: a collection of 10 points

Data is stored in two files:

- data.csv: 700k different points
- masks.csv: 700k clusters. Each column in the dataset contains an index referencing a point in data.csv

The goal is to compare the performance of naive solutions in python, cython and rust. In other words, the idea
is to compare low-effort implementations.

## Results

- Python - 15s

- Python (using pandas apply) - TBD

- Cython - 3s

- Rust (single thread) - 1.5s*

- Rust (using rayon for parallelization) - 0.2s*

*(remember to use "maturin develop --release" to get optimized build)
