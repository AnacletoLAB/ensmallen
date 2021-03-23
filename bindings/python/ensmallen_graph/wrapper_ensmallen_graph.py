"""Wrapper for adding iterators over holdouts."""
from typing import Dict, Generator, List
from .ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error
from .utils import holdouts_wrapper


class WrapperEnsmallenGraph(EnsmallenGraph):

    def connected_holdouts(
        self,
        train_size: float,
        holdouts_number: int,
        random_state: int = 42,
        random_state_factor: int = 1000,
        desc: str = "Computing connected holdouts for link prediction",
        edge_types: List[str] = None,
        include_all_edge_types: bool = False,
        verbose: bool = True
    ) -> Generator:
        """Return generator of the connected holdouts.

        The generator returned yields a tuple with the current holdouts number
        and the training and test graphs.

        Parameters
        ----------------------------
        train_size: float,
            The rate of edges to reserve for the training.
        holdouts_number: int,
            The number of holdouts to yield.
        random_state: int = 42,
            The random state to use to start generating the holdouts.
        random_state_factor: int = 1000,
            The factor to use to multiply the increase of the random state.
            This is needed to make the randomly generated holdouts more different.
        desc: str = "Computing holdouts",
            The description for the TQDM bar.
        random_state: int = 42,
            The random_state to use to generate the holdout.
        edge_types: List[str] = None,
            List of names of the edge types to put into the validation.
        include_all_edge_types: bool = False,
            whether to include all the edges between two nodes.
            This is only relevant in multi-graphs.
        verbose: bool = True,
            whether to show the loading bar.
        """
        return holdouts_wrapper(
            self.connected_holdout,
            holdouts_number,
            random_state=random_state,
            random_state_factor=random_state_factor,
            desc=desc,
            disable=not verbose,
            train_size=train_size,
            edge_types=edge_types,
            include_all_edge_types=include_all_edge_types,
            verbose=verbose
        )

    def node_label_holdouts(
        self,
        train_size: float,
        holdouts_number: int,
        random_state: int = 42,
        random_state_factor: int = 1000,
        desc: str = "Computing holdouts for node-label prediction",
        use_stratification: bool = True,
        verbose: bool = True
    ) -> Generator:
        """Return generator of the node-label holdouts.

        The generator returned yields a tuple with the current holdouts number
        and the training and test graphs.

        Parameters
        ----------------------------
        train_size: float,
            The rate of edges to reserve for the training.
        holdouts_number: int,
            The number of holdouts to yield.
        random_state: int = 42,
            The random state to use to start generating the holdouts.
        random_state_factor: int = 1000,
            The factor to use to multiply the increase of the random state.
            This is needed to make the randomly generated holdouts more different.
        desc: str = "Computing holdouts for node-label prediction",
            The description for the TQDM bar.
        random_state: int = 42,
            The random_state to use to generate the holdout.
        use_stratification: bool = True,
            Whether to use node-label stratification,
        verbose: bool = True,
            whether to show the loading bar.

        Raises
        -----------------------------
        ValueError,
            If the graph does not have node types.
        ValueError,
            If the stratification is required but the graph has multi-label node types.
        ValueError,
            If the stratification is required but the graph has some node types with insufficient cardinality.
        """
        return holdouts_wrapper(
            self.node_label_holdout,
            holdouts_number,
            random_state=random_state,
            random_state_factor=random_state_factor,
            desc=desc,
            disable=not verbose,
            use_stratification=use_stratification,
            verbose=verbose
        )

    def edge_label_holdouts(
        self,
        train_size: float,
        holdouts_number: int,
        random_state: int = 42,
        random_state_factor: int = 1000,
        desc: str = "Computing holdouts for edge-label prediction",
        use_stratification: bool = True,
        verbose: bool = True
    ) -> Generator:
        """Return generator of the edge-label holdouts.

        The generator returned yields a tuple with the current holdouts number
        and the training and test graphs.

        Parameters
        ----------------------------
        train_size: float,
            The rate of edges to reserve for the training.
        holdouts_number: int,
            The number of holdouts to yield.
        random_state: int = 42,
            The random state to use to start generating the holdouts.
        random_state_factor: int = 1000,
            The factor to use to multiply the increase of the random state.
            This is needed to make the randomly generated holdouts more different.
        desc: str = "Computing holdouts for edge-label prediction",
            The description for the TQDM bar.
        random_state: int = 42,
            The random_state to use to generate the holdout.
        use_stratification: bool = True,
            Whether to use edge-label stratification,
        verbose: bool = True,
            whether to show the loading bar.

        Raises
        -----------------------------
        ValueError,
            If the graph does not have edge types.
        ValueError,
            If the stratification is required but the graph has some edge types with insufficient cardinality.
        """
        return holdouts_wrapper(
            self.edge_label_holdout,
            holdouts_number,
            random_state=random_state,
            random_state_factor=random_state_factor,
            desc=desc,
            disable=not verbose,
            use_stratification=use_stratification,
            verbose=verbose
        )
