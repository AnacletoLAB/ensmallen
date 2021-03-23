"""Utility to iterate over the holdouts."""
from typing import Dict, Callable, Generator
from tqdm.auto import trange


def holdouts_generator(
    holdout_callback: Callable,
    holdouts_number: int,
    random_state: int = 42,
    random_state_factor: int = 1000,
    desc: str = "Computing holdouts",
    disable: bool = False,
    **kwargs: Dict
) -> Generator:
    """Return generator of the holdouts.

    The generator returned yields a tuple with the current holdouts number
    and the training and test graphs.

    Parameters
    ----------------------------
    holdout_callback: Callable,
        The callback that generates the training and test holdout.
    holdouts_number: int,
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
    **kwargs: Dict,
        The kwargs to pass to the given callback.
    """
    return (
        (i, holdout_callback(
            **kwargs,
            # The multiplication is a simple way to make the
            # randomly sampled holdouts a bit farther one to the other.
            random_state=random_state+i*random_state_factor,
        ))
        for i in trange(
            holdouts_number,
            disable=disable,
            desc=desc,
        )
    )
