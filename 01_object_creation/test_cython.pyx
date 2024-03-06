import numpy as np
import pandas as pd

cimport numpy as np
cimport pandas as pd

cimport cython

from libc.stdlib cimport malloc, free

cdef class Point:
    cdef public double x, y, z
    
    def __cinit__(self, double x, double y, double z):
        self.x = x
        self.y = y
        self.z = z

cdef class Cluster:
    cdef public list points
    
    def __cinit__(self, list points):
        self.points = points


@cython.boundscheck(False)
@cython.wraparound(False)
cpdef get_clusters(object df, object masks):
    cdef list clusters = []
    cdef np.ndarray[np.int_t, ndim=2] mask_vectors = np.array(masks).astype(np.intc)
    cdef list point_dicts = [{"x": row[0], "y": row[1], "z": row[2]} for row in df[["x", "y", "z"]].values]
    cdef int num_clusters = mask_vectors.shape[0]

    cdef int i, j
    cdef np.int_t *mask_ptr
    cdef int mask_length
    cdef list point_data_list
    cdef Point point
    cdef Cluster cluster
    cdef dict point_data

    for i in range(num_clusters):
        mask_ptr = &mask_vectors[i, 0]
        mask_length = mask_vectors.shape[1]
        point_data_list = []
        
        for j in range(mask_length):
            point_data = point_dicts[mask_ptr[j]]
            point = Point(point_data["x"], point_data["y"], point_data["z"])
            point_data_list.append(point)
        
        cluster = Cluster(point_data_list)
        clusters.append(cluster)

    return clusters