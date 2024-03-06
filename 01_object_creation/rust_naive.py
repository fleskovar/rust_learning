import pandas as pd
import numpy as np
import time
from new_test import Point, Cluster, get_clusters, get_clusters_rayon


def time_func(func, *args, **kwargs):

    start_time = time.time()
    return_val = func(*args, **kwargs)

    end_time = time.time()
    time_taken = end_time - start_time
    print(f"Time taken {time_taken} for {func}")

    return return_val


if __name__ == "__main__":

    print("Starting")
    df = pd.read_csv("data.csv")
    masks = pd.read_csv("masks.csv")

    f = [get_clusters, get_clusters_rayon]

    # clusters = time_func(
    #     get_clusters, df[["x", "y", "z"]].values, masks.values.astype(int)
    # )

    clusters = time_func(f[1], df[["x", "y", "z"]].values, masks.values.astype(int))

    print(clusters[0].points[0])
