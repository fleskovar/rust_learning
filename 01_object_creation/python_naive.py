import pandas as pd
import numpy as np
import time
from tqdm import tqdm


class Point:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z


class Cluster:
    def __init__(self, points):
        self.points = points


def time_func(func, *args, **kwargs):

    start_time = time.time()
    return_val = func(*args, **kwargs)

    end_time = time.time()
    time_taken = end_time - start_time
    print(f"Time taken {time_taken} for {func}")

    return return_val


def get_clusters(df, masks):

    _df = df[["x", "y", "z"]].to_dict("records")

    clusters = list()
    for mask in masks.values.astype(int):

        clusters.append(
            Cluster(
                points=[Point(**point_data) for point_data in [_df[i] for i in mask]]
            )
        )

    return clusters


def get_clusters_apply(df: pd.DataFrame, masks: pd.DataFrame):

    df = df.loc[:, ["x", "y", "z"]]

    def create_records(row: pd.Series):
        masked_df = df.iloc[row.to_list()]
        points_list = masked_df.apply(lambda x: Point(**x.to_dict()), axis=1).to_list()
        return Cluster(points=points_list)

    clusters = masks.apply(create_records, axis=1)

    return clusters


if __name__ == "__main__":

    print("Starting")
    df = pd.read_csv("data.csv")
    masks = pd.read_csv("masks.csv")

    clusters = time_func(get_clusters, df, masks)
