"""Utility to iterate over the holdouts."""
from typing import Dict, Callable, Generator
from tqdm.auto import trange


def holdouts_generator(
    holdout_callback: Callable,
    train_size: float = 0.8,
    holdouts_number: int = 10,
    random_state: int = 42,
    random_state_factor: int = 1000,
    desc: str = "Computing holdouts",
    disable: bool = False,
    leave: bool = False,
    enable_speedup: bool = True,
    **kwargs: Dict
) -> Generator:
    """Return generator of the holdouts.

    The generator returned yields a tuple with the current holdouts number
    and the training and test graphs.

    Parameters
    ----------------------------
    holdout_callback: Callable,
        The callback that generates the training and test holdout.
    train_size: float = 0.8,
        The portion of the data to reserve for the training data.
        Note that this value is a maximal, if there is an odd number
        of values the value will be assigned to the test set in order to
        avoid a potentially small positive evaluation bias.
    holdouts_number: int = 10,
        The number of holdouts to yield.
    random_state: int = 42,
        The random state to use to start generating the holdouts.
    random_state_factor: int = 1000,
        The factor to use to multiply the increase of the random state.
        This is needed to make the randomly generated holdouts more different.
    desc: str = "Computing holdouts",
        The description for the TQDM bar.
    disable: bool = False,
        Whether to show the loading bars,
    leave: bool = False,
        Whether to leave the loading bar after execution.
    enable_speedup: bool = True,
        Whether to enable all speedups in the holdouts graphs.
    **kwargs: Dict,
        The kwargs to pass to the given callback.
    """
    for holdout_number in trange(
        holdouts_number,
        disable=disable,
        desc=desc,
        leave=leave
    ):
        # Generate the training and test graphs.
        train_graph, test_graph = holdout_callback(
            **kwargs,
            train_size=train_size,
            # The multiplication is a simple way to make the
            # randomly sampled holdouts a bit farther one to the other.
            random_state=random_state+holdout_number*random_state_factor,
        )
        # Enable the speedups
        if enable_speedup:
            train_graph.enable(
                vector_destinations=True,
                vector_outbounds=True
            )
            test_graph.enable(
                vector_destinations=True,
                vector_outbounds=True
            )
        yield holdout_number, (train_graph, test_graph)
