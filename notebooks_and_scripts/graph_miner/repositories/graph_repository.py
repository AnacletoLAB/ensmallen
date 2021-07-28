"""Class providing an abstract graph repository."""
from typing import List, Dict, Set
import pandas as pd
import os
import json
import compress_json
import datetime
import shutil
from glob import glob
from collections import Counter
from downloaders import BaseDownloader
from ensmallen_graph import EnsmallenGraph
from tqdm.auto import tqdm
from environments_utils import is_notebook
from IPython.display import display
from .custom_exceptions import UnsupportedGraphException


class GraphRepository:
    def __init__(self):
        """Create new Graph Repository object."""
        self._downloader = BaseDownloader(
            process_number=1,
            verbose=2,
            target_directory=self.repository_package_name
        )

    def build_stored_graph_name(self, partial_graph_name: str) -> str:
        """Return built graph name.

        Parameters
        -----------------------
        partial_graph_name: str,
            Partial graph name to be built.

        Returns
        -----------------------
        Complete name of the graph.
        """
        raise NotImplementedError(
            "The method build_stored_graph_name must be implemented in child classes."
        )

    def get_graph_name(self, graph_data) -> str:
        """Return built graph name.

        Parameters
        -----------------------
        graph_data: str,
            Graph data to be used.

        Returns
        -----------------------
        Complete name of the graph.
        """
        raise NotImplementedError(
            "The method get_graph_name must be implemented in child classes."
        )

    def get_formatted_repository_name(self) -> str:
        """Return formatted reporitory name."""
        raise NotImplementedError(
            "The method get_formatted_repository_name must be implemented in child classes."
        )

    def get_graph_citations(self, graph_data) -> List[str]:
        """Return citations relative to the graphs.

        Parameters
        -----------------------
        graph_data: str,
            Graph data to be used.

        Returns
        -----------------------
        Citations relative to the graph.
        """
        raise NotImplementedError(
            "The method get_graph_citations must be implemented in child classes."
        )

    def from_integer_sorted_reports_directory(self) -> str:
        """Return directory path where graph reports are stored.

        Returns
        -----------------------
        String with directtory.
        """
        return os.path.join(
            os.path.dirname(os.path.abspath(__file__)),
            self.repository_package_name,
            "reports",
        )

    def from_integer_sorted_report_path(self, graph_name: str) -> str:
        """Return path where graph report is stored.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to build path for.

        Returns
        -----------------------
        String with path.
        """
        return os.path.join(
            self.from_integer_sorted_reports_directory(),
            "{}.json.gz".format(
                self.build_stored_graph_name(graph_name)
            )
        )

    def load_graph_report(self, graph_name: str) -> Dict:
        """Return dictionary with metadata.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to build path for.

        Returns
        -----------------------
        Metadata relative to the given graph.
        """
        return compress_json.load(
            self.from_integer_sorted_report_path(graph_name)
        )

    def dump_graph_report(
        self,
        graph_name: str,
        graph_textual_report: str,
        graph_json_report: Dict,
        citations: List[str],
        urls: List[str],
        paths: List[str],
        arguments: Dict
    ):
        """Save given graph data into database.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to build path for.
        graph_textual_report: str,
            Textual report of the graph.
        graph_json_report: Dict,
            Report of the graph in JSON form.
        citations: List[str],
            List of citations.
        urls: List[str],
            Urls from where to download the files from.
        paths: List[str],
            Paths where to store the files.
        arguments: Dict,
            Arguments to use to load the graph object.
        """
        if not paths:
            paths = None
        compress_json.dump(
            {
                "graph_name": graph_name,
                "graph_method_name": self.build_stored_graph_name(graph_name),
                "graph_textual_report": graph_textual_report,
                "graph_json_report": graph_json_report,
                "citations": citations,
                "urls": urls,
                "paths": paths,
                "datetime": str(datetime.datetime.now()),
                "arguments": arguments,
            },
            self.from_integer_sorted_report_path(graph_name)
        )

    def is_graph_cached(self, graph_name: str) -> bool:
        """Return boolean representing if graph is cached.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to check if it is cached.

        Returns
        -----------------------
        Wether the cache if cached.
        """
        return os.path.exists(self.from_integer_sorted_report_path(graph_name))

    @property
    def name(self) -> str:
        """Return name of the repository."""
        return self.__class__.__name__

    @property
    def repository_package_name(self) -> str:
        """Return repository_package_name of the repository."""
        return self.name[:-len("GraphRepository")].lower()

    @property
    def corrupted_graphs_path(self):
        """Return path to corrupted graphs json."""
        return "corrupted_graphs/{}.json.gz".format(self.name)

    @property
    def unsupported_graphs_path(self):
        """Return path to unsupported graphs json."""
        return "unsupported_graphs/{}.json.gz".format(self.name)

    def _load_corrupted_graphs(self) -> Set[str]:
        """Return set of known corrupted graphs."""
        try:
            return compress_json.local_load(self.corrupted_graphs_path)
        except Exception:
            return list()

    def _load_unsupported_graphs(self) -> Set[str]:
        """Return set of known unsupported graphs."""
        try:
            return compress_json.local_load(self.unsupported_graphs_path)
        except Exception:
            return list()

    def _dump_corrupted_graphs(self, corrupted_graphs: Set[str]):
        """Return set of known corrupted graphs."""
        compress_json.local_dump(corrupted_graphs, self.corrupted_graphs_path)

    def _dump_unsupported_graphs(self, unsupported_graphs: Set[str]):
        """Return set of known unsupported graphs."""
        compress_json.local_dump(
            unsupported_graphs, self.unsupported_graphs_path)

    def add_corrupted_graph(self, graph_name: str):
        """Add given graph to corrupted graphs set.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to add to corrupted set.
        """
        corrupted_graphs = self._load_corrupted_graphs()
        corrupted_graphs.append(graph_name)
        self._dump_corrupted_graphs(corrupted_graphs)

    def add_unsupported_graph(self, graph_name: str):
        """Add given graph to unsupported graphs set.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to add to unsupported set.
        """
        unsupported_graphs = self._load_unsupported_graphs()
        unsupported_graphs.append(graph_name)
        self._dump_unsupported_graphs(unsupported_graphs)

    def is_graph_corrupted(self, graph_name: str) -> bool:
        """Return boolean representing if graph is known to be corrupted.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to check if it is corrupted.

        Returns
        -----------------------
        Wether the graph is known to be corrupted.
        """
        return graph_name in self._load_corrupted_graphs()

    def is_graph_unsupported(self, graph_name: str) -> bool:
        """Return boolean representing if graph is known to be unsupported.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to check if it is unsupported.

        Returns
        -----------------------
        Wether the graph is known to be unsupported.
        """
        return graph_name in self._load_unsupported_graphs()

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        raise NotImplementedError(
            "The method get_graph_list must be implemented in child classes."
        )

    def get_uncached_graph_list(self) -> List[str]:
        """Return graphs to be parsed."""
        return [
            (self.get_graph_name(graph_data), graph_data)
            for graph_data in self.get_graph_list()
            if not (
                self.is_graph_cached(self.get_graph_name(graph_data)) or
                self.is_graph_corrupted(self.get_graph_name(graph_data)) or
                self.is_graph_unsupported(self.get_graph_name(graph_data))
            )
        ]

    def get_graph_urls(self, graph_name: str) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.

        Returns
        -----------------------
        The urls list from where to download the graph data.
        """
        raise NotImplementedError(
            "The method get_graph_urls must be implemented in child classes."
        )

    def get_graph_paths(self, graph_name: str, urls: List[str]) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        urls: List[str],
            Urls from where to download the graphs.

        Returns
        -----------------------
        The paths where to store the downloaded graphs.
        """
        raise NotImplementedError(
            "The method get_graph_paths must be implemented in child classes."
        )

    def from_integer_sorted_parameters(
        self,
        graph_name: str,
        edge_path: str,
        node_path: str = None,
    ) -> Dict:
        """Return dictionary with kwargs to load graph.

        Parameters
        ---------------------
        graph_name: str,
            Name of the graph to load.
        edge_path: str,
            Path from where to load the edge list.
        node_path: str = None,
            Optionally, path from where to load the nodes.

        Returns
        -----------------------
        Dictionary to build the graph object.
        """
        return dict(
            name=graph_name,
            edge_path=edge_path,
            **(
                dict(node_path=node_path)
                if node_path is not None
                else {}
            ),
            edge_separator=self.get_file_separator(edge_path),
            node_separator=self.get_file_separator(node_path),
            edge_rows_to_skip=self.get_lines_to_skip(edge_path),
            edge_file_comment_symbol=self.get_file_comment_symbol(edge_path),
            node_file_comment_symbol=self.get_file_comment_symbol(node_path),
            directed=False
        )

    def download(self, graph_data, graph_name: str) -> pd.DataFrame:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_data,
            Data of the graph to retrieve.
        graph_name: str,
            Name of the graph to retrieve.

        Returns
        -----------------------
        Dataframe with download metadata.
        """
        urls = self.get_graph_urls(graph_data)
        return self._downloader.download(
            urls=urls,
            paths=self.get_graph_paths(graph_name, urls)
        )

    def get_file_separator(
        self,
        path: str
    ) -> str:
        """Return the candidate file separator.

        Parameters
        -----------------------
        path: str,
            Path for which to identify the separator.

        Returns
        -----------------------
        Character likely used as separator in the file.
        """
        if path is None:
            return None

        counter = Counter()
        with open(path, "r") as f:
            for _ in range(2000):
                counter.update([
                    c
                    for c in f.readline()
                    if c in (";", ",", " ", "\t")
                ])

        return counter.most_common(n=1)[0][0]

    def get_file_comment_symbol(
        self,
        path: str
    ) -> str:
        """Return the candidate file comment.

        Parameters
        -----------------------
        path: str,
            Path for which to identify the comment.

        Returns
        -----------------------
        Character likely used as comment identifier in the file.
        """
        if path is None:
            return None

        with open(path, "r") as f:
            first_line = f.readline()

        comment_character = None
        for symbol in ("%", "#"):
            if first_line.startswith(symbol):
                comment_character = symbol

        return comment_character

    def get_starting_commented_lines_number(self, path: str) -> int:
        """Return number of commented lines since beginning.

        Parameters
        -----------------------
        path: str,
            Path from which to count commented lines.

        Returns
        -----------------------
        Number of commented lines.
        """
        comment_symbol = self.get_file_comment_symbol(path)
        if comment_symbol is None:
            return 0
        commented_lines_number = 0
        with open(path, "r") as f:
            while True:
                if f.readline().startswith(comment_symbol):
                    commented_lines_number += 1
                else:
                    break
        return commented_lines_number

    def get_lines_to_skip(self, path: str) -> int:
        """Return number of lines to skip.

        Parameters
        -----------------------
        path: str,
            Path from which to identify lines to skip.

        Returns
        -----------------------
        Number of lines to skip.
        """
        if path.endswith(".mtx"):
            return 1
        return 0

    def load_dataframe(self, path: str) -> pd.DataFrame:
        """Return data loaded as DataFrame."""
        return pd.read_csv(
            path,
            sep=self.get_file_separator(path),
            skiprows=self.get_starting_commented_lines_number(
                path) + self.get_lines_to_skip(path),
            header=None,
            nrows=1000000,
            low_memory=False
        )

    def display_dataframe_preview(self, data: pd.DataFrame):
        """Displays in the best way possible the file."""
        if is_notebook():
            display(data[:10])
        else:
            print(data[:10])

    def get_node_path(
        self,
        graph_name: str,
        download_report: pd.DataFrame
    ) -> str:
        """Return path from where to load the node files.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.
        download_report: pd.DataFrame,
            Report from downloader.

        Returns
        -----------------------
        The path from where to load the node files.
        """
        raise NotImplementedError(
            "The method get_node_path must be implemented in child classes."
        )

    def get_edge_path(
        self,
        graph_name: str,
        download_report: pd.DataFrame
    ) -> str:
        """Return path from where to load the edge files.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.
        download_report: pd.DataFrame,
            Report from downloader.

        Returns
        -----------------------
        The path from where to load the edge files.
        """
        raise NotImplementedError(
            "The method get_edge_path must be implemented in child classes."
        )

    def get_imports(self, graph_name: str) -> str:
        """Return imports to be added to model file.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.

        Returns
        -----------------------
        Imports.
        """
        return ""

    def get_description(self, graph_name: str) -> str:
        """Return description to be added to model file.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.

        Returns
        -----------------------
        description.
        """
        return ""

    def get_callbacks(self, graph_name: str) -> str:
        """Return callbacks to be added to model file.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.

        Returns
        -----------------------
        callbacks.
        """
        raise NotImplementedError(
            "The method get_callbacks must be implemented in child classes."
        )

    def get_callbacks_arguments(self, graph_name: str) -> List[Dict]:
        """Return arguments for callbacks to be added to model file.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.

        Returns
        -----------------------
        arguments for callbacks.
        """
        raise NotImplementedError(
            "The method get_callbacks_arguments must be implemented in child classes."
        )

    def check_nominal_download(
        self,
        download_report: pd.DataFrame
    ) -> bool:
        """Return boolean representing if everything went ok.

        Parameters
        -----------------------
        download_report: pd.DataFrame,
            Report from downloader.

        Returns
        -----------------------
        Boolean representing if everything went ok.
        """
        return True

    def clear_downloaded_data(self):
        """Removes all downloaded graph files."""
        if os.path.exists(self.repository_package_name):
            shutil.rmtree(self.repository_package_name)

    def retrieve_all(self):
        """Retrives data for the graphs from the considered repository."""
        for graph_name, graph_data in tqdm(
            self.get_uncached_graph_list(),
            desc="Retrieving graphs for {}".format(self.name),
            leave=False
        ):
            self.clear_downloaded_data()
            download_report = self.download(graph_data, graph_name)
            if not self.check_nominal_download(download_report):
                self.add_corrupted_graph(graph_name)
                continue
            try:
                node_path = self.get_node_path(
                    graph_name, download_report)
                edge_path = self.get_edge_path(
                    graph_name, download_report)
                arguments = self.from_integer_sorted_parameters(
                    graph_name,
                    edge_path=edge_path,
                    node_path=node_path,
                )
            except UnsupportedGraphException:
                self.add_unsupported_graph(graph_name)
                continue
            graph: EnsmallenGraph = EnsmallenGraph.from_unsorted_csv(
                **arguments
            )
            self.dump_graph_report(
                graph_name,
                graph_textual_report=str(graph),
                graph_json_report=graph.report(),
                arguments=arguments,
                citations=self.get_graph_citations(graph_data),
                urls=download_report.url.tolist(),
                paths=download_report.destination.tolist(),
            )
            self.clear_downloaded_data()

    def format_references(self, references: List[str]) -> str:
        """Return formatted references model.

        Parameters
        ---------------------
        references: List[str],
            List of the references of the graph.

        Returns
        ---------------------
        Formatted model of the references.
        """
        if not references:
            return ""
        with open(
            "{}/models/references.rst".format(
                os.path.dirname(os.path.abspath(__file__))),
            "r"
        ) as f:
            return f.read().format("\n\n".join(references))

    def format_lines(self, text: str, line_length: int = 70) -> str:
        """Return formatted lines.

        Parameters
        --------------------------
        text: str,
            The text to be formatted.
        line_length: int = 70,
            Maximum length of the text lines.

        Returns
        --------------------------
        Formatted text.
        """
        line_length = 70
        lines = []
        line = None
        for word in text.split(" "):
            if line is None:
                line = word
            else:
                line += " " + word
            if len(line) >= line_length:
                lines.append(line)
                line = None
        if line is not None:
            lines.append(line)
        return "\n".join(lines)

    def format_report(self, report: str, datetime: str) -> str:
        """Return formatted report model.

        Parameters
        ---------------------
        report: str,
            Report of the graph.
        datetime: str,
            Datetime of when the report whas created.

        Returns
        ---------------------
        Formatted model of the report.
        """
        with open(
            "{}/models/report.rst".format(
                os.path.dirname(os.path.abspath(__file__))),
            "r"
        ) as f:
            return f.read().format(
                report=self.format_lines(report),
                datetime=datetime
            )

    def format_usage_example(self, graph_name: str) -> str:
        """Return formatted report model.

        Parameters
        ---------------------
        graph_name: str,
            Name of the graph to retrieve.

        Returns
        ---------------------
        Formatted model of the report.
        """
        with open(
            "{}/models/usage_example.rst".format(
                os.path.dirname(os.path.abspath(__file__))),
            "r"
        ) as f:
            return f.read().format(
                repository_package_name=self.repository_package_name,
                graph_method_name=self.build_stored_graph_name(graph_name)
            )

    def add_tabs(self, text: str) -> str:
        """Add tabs for formatting porposes to given text.

        Parameters
        --------------------
        text: str,
            The text to format.

        Returns
        --------------------
        Formatted text.
        """
        return "\t" + "\n\t".join(text.split("\n"))

    def format_callbacks_data(self, graph_name: str) -> str:
        """Return formatted callbacks data.

        Parameters
        ---------------------
        graph_name: str,
            Name of the graph to retrieve.

        Returns
        ---------------------
        Formatted callbacks data.
        """
        try:
            callbacks = self.get_callbacks(graph_name)
            callbacks_data = self.get_callbacks_arguments(graph_name)
            return ",\n" + self.add_tabs(self.add_tabs("\n".join((
                "callbacks=[\n{}\n],".format(self.add_tabs("\n".join(
                    callbacks
                ))),
                "callbacks_arguments={}".format(json.dumps(
                    callbacks_data,
                    indent=4
                )),
            ))))
        except NotImplementedError:
            return ""

    def format_graph_retrieval_file(
        self,
        graph_name: str,
        report: str,
        references: List[str]
    ) -> str:
        """Return formatted report model.

        Parameters
        ---------------------
        graph_name: str,
            Name of the graph to retrieve.
        report: str,
            Report of the graph.
        references: List[str],
            List of the references of the graph.

        Returns
        ---------------------
        Formatted model of the report.
        """
        with open(
            "{}/models/graph_retrieval_file.py".format(
                os.path.dirname(os.path.abspath(__file__))),
            "r"
        ) as f:
            return f.read().format(
                graph_method_name=self.build_stored_graph_name(graph_name),
                repository_package_name=self.repository_package_name,
                graph_name=graph_name,
                repository_name=self.get_formatted_repository_name(),
                report=report,
                imports=self.get_imports(graph_name),
                callbacks_data=self.format_callbacks_data(
                    graph_name,
                ),
                description=self.format_lines(
                    self.get_description(graph_name)
                ),
                tabbed_description=self.add_tabs(
                    self.format_lines(self.get_description(graph_name))
                ),
                references=self.format_references(references),
                usage_example=self.format_usage_example(graph_name),
                tabbed_report=self.add_tabs(report),
                tabbed_references=self.add_tabs(
                    self.format_references(references)),
                tabbed_usage_example=self.add_tabs(
                    self.format_usage_example(graph_name))
            )

    def format_init_file(
        self,
        graph_method_names: List[str],
        graph_file_names: List[str]
    ) -> str:
        """Return formatted init model.

        Parameters
        ---------------------
        graph_method_names: List[str],
            Names of the methods to import.
        graph_file_names: List[str],
            Names of the files to import the methods from.

        Returns
        ---------------------
        Formatted model of init file.
        """
        import_pattern = "from .{graph_file_name} import {graph_method_name}"
        imports = "\n".join([
            import_pattern.format(
                graph_file_name=graph_file_name,
                graph_method_name=graph_method_name
            )
            for graph_method_name, graph_file_name in zip(
                graph_method_names,
                graph_file_names
            )
        ])
        method_names = self.add_tabs(self.format_lines(" ".join([
            '"{}",'.format(graph_method_name)
            for graph_method_name in graph_method_names
        ])))

        with open(
            "{}/models/init_file_model.py".format(
                os.path.dirname(os.path.abspath(__file__))),
            "r"
        ) as f:
            return f.read().format(
                imports=imports,
                method_names=method_names,
                repository_name=self.get_formatted_repository_name()
            )

    def build_all(self):
        """Build graph retrieval methods."""
        graph_method_names = []
        graph_file_names = []
        target_directory_path = os.path.join(
            "bindings/python/ensmallen_graph/datasets",
            self.repository_package_name,
        )
        for graph_report_path in tqdm(
            glob("{}/*.json.gz".format(self.from_integer_sorted_reports_directory())),
            desc="Building graph retrieval methods for {}".format(self.name),
            leave=False
        ):
            graph_data = compress_json.load(graph_report_path)
            graph_retrieval_file = self.format_graph_retrieval_file(
                graph_name=graph_data["graph_name"],
                report=self.format_report(
                    graph_data["graph_textual_report"],
                    graph_data["datetime"]
                ),
                references=graph_data["citations"],
            )
            target_path = os.path.join(
                target_directory_path,
                "{}.py".format(
                    self.build_stored_graph_name(
                        graph_data["graph_name"]).lower()
                )
            )
            graph_method_names.append(
                self.build_stored_graph_name(graph_data["graph_name"])
            )
            graph_file_names.append(
                self.build_stored_graph_name(graph_data["graph_name"]).lower()
            )
            target_json_path = os.path.join(
                target_directory_path,
                "{}.json.gz".format(
                    self.build_stored_graph_name(graph_data["graph_name"])
                )
            )
            os.makedirs(target_directory_path, exist_ok=True)
            with open(target_path, "w") as f:
                f.write(graph_retrieval_file)
            compress_json.dump(graph_data, target_json_path)
        init_path = os.path.join(
            target_directory_path,
            "__init__.py"
        )
        with open(init_path, "w") as f:
            f.write(self.format_init_file(
                graph_method_names,
                graph_file_names
            ))
