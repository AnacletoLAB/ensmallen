from time import time
from humanize import naturaldelta
from ensmallen_graph import EnsmallenGraph  # pylint: disable=no-name-in-module
import numpy as np
import pandas as pd
from tqdm import tqdm, trange
import os
from collections import Counter
from notipy_me import Notipy


def single_speed_test(directory: str):
    start = time()
    try:
        graph = EnsmallenGraph(
            edge_path=f"{directory}/edges.tsv",
            sources_column="subject",
            destinations_column="object",
            directed=True,
            ignore_duplicated_edges=True
        )
    except ValueError as e:
        print("="*100, "\n", e)
        return {}

    completed_graph = time() - start
    start_walk = time()
    walks = graph.walk(10, 80)
    delta = time() - start
    total_walk_time = time() - start_walk

    walks_lengths = [
        len(walk) for walk in walks
    ]

    results = {
        "directory": directory,
        "total_required_time": delta,
        "building_graph_required_time": completed_graph,
        "random_walk_time": total_walk_time,
        "mean_walks_length": np.mean(walks_lengths),
        "median_walks_length": np.median(walks_lengths),
        **graph.report()
    }

    print(results)

    return results


def speed_test(root: str, iterations: int = 1) -> pd.DataFrame:
    directories = [
        f"{root}/{directory}"
        for directory in os.listdir(root)
        if os.path.isdir(f"{root}/{directory}")
    ]
    return pd.DataFrame([
        single_speed_test(directory)
        for directory in tqdm(directories, desc="Speed test")
        for _ in trange(iterations, desc="Iterations", leave=False)
    ])


with Notipy(task_name="Ensmallen Graph speed test"):
    speed_test("../graph").to_csv("speed_test_report.csv", index=False)
