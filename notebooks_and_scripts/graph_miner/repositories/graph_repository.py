"""Class providing an abstract graph repository."""
import json
import os
from glob import glob
from typing import Dict, List, Set

import compress_json
from tqdm.auto import tqdm


class GraphRepository:
    """Class representing an abstract graph repository."""

    def __init__(self):
        """Create new Graph Repository object."""
        with open(
            "{}/models/{}.py".format(
                os.path.dirname(os.path.abspath(__file__)),
                self.get_graph_retrieval_file()
            ),
            "r"
        ) as f:
            self._graph_method_model = f.read()

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

    def get_graph_data_path(self, graph_name: str) -> str:
        """Return path where to store the graph data."""
        return os.path.join(
            "graph_repositories",
            self.get_formatted_repository_name(),
            "{}.json.gz".format(graph_name),
        )

    def load_graph_data(self, graph_name: str) -> Dict:
        """Return the data stored for the provided graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrieve data for.

        Returns
        -----------------------
        The stored data for this graph.
        """
        return compress_json.local_load(
            self.get_graph_data_path(graph_name)
        )

    def store_graph_data(self, data: Dict, graph_name: str) -> Dict:
        """Return the data stored for the provided graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to store data for.

        Returns
        -----------------------
        The stored data for this graph.
        """
        return compress_json.local_dump(
            data,
            self.get_graph_data_path(graph_name)
        )

    def is_graph_cached(self, graph_name: str, version: str) -> bool:
        """Return boolean representing if graph is cached.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to check if it is cached.
        version: str,
            The version of the graph to check for.

        Returns
        -----------------------
        Wether the cache if cached.
        """
        return os.path.exists(self.get_graph_data_path(graph_name, version))

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
            graph_name
            for graph_name in self.get_graph_list()
            if not (
                self.is_graph_corrupted(graph_name) or
                self.is_graph_unsupported(graph_name)
            )
        ]

    def get_graph_versions(self, graph_name: str) -> List[str]:
        """Return list of versions of the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrieve versions for.

        Returns
        -----------------------
        List of versions for the given graph.
        """
        raise NotImplementedError(
            "The method get_graph_versions must be implemented in child classes."
        )

    def get_graph_urls(self, graph_name: str, version: str) -> List[str]:
        """Return urls for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The urls list from where to download the graph data.
        """
        raise NotImplementedError(
            "The method get_graph_urls must be implemented in child classes."
        )

    def get_graph_paths(self, graph_name: str, version: str) -> List[str]:
        """Return paths for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The paths where to store the downloaded graphs.
        """
        raise NotImplementedError(
            "The method get_graph_paths must be implemented in child classes."
        )

    def get_graph_references(self, graph_name: str, version: str) -> List[str]:
        """Return references for a given graph and version.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        The bibliographic references relative to this graph.
        """
        raise NotImplementedError(
            "The method get_graph_references must be implemented in child classes."
        )

    def get_graph_arguments(self, graph_name: str, version: str) -> Dict:
        """Return arguments for a given graph and version.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        version: str,
            Version to retrieve this information for.

        Returns
        -----------------------
        Dictionary with the arguments required to load this graph.
        """
        raise NotImplementedError(
            "The method get_graph_arguments must be implemented in child classes."
        )

    def get_imports(self, graph_name: str, version: str) -> str:
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

    def get_description(self, graph_name: str, version: str) -> str:
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

    def get_callbacks(self, graph_name: str, version: str) -> str:
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

    def get_callbacks_arguments(self, graph_name: str, version: str) -> List[Dict]:
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

    def retrieve_all(self):
        """Retrives data for the graphs from the considered repository."""
        for graph_name in tqdm(
            self.get_uncached_graph_list(),
            desc="Retrieving graphs for {}".format(self.name),
            leave=False,
            dynamic_ncols=True
        ):
            self.store_graph_data(
                {
                    version: dict(
                        graph_name=self.get_graph_arguments(
                            graph_name,
                            version
                        )["name"],
                        version=version,
                        graph_method_name=self.build_stored_graph_name(
                            graph_name),
                        urls=self.get_graph_urls(graph_name, version),
                        paths=self.get_graph_paths(graph_name, version),
                        references=self.get_graph_references(
                            graph_name,
                            version
                        ),
                        arguments=self.get_graph_arguments(
                            graph_name,
                            version
                        )
                    )
                    for version in self.get_graph_versions(graph_name)
                },
                graph_name=graph_name,
            )

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

    def get_graph_retrieval_file(self) -> str:
        """Return graph retrieval file."""
        return "graph_retrieval_file"

    def format_versions(self, versions: List[str]) -> str:
        """Return versions available."""
        if versions == ["latest"]:
            return ""
        return "\n\tThe available versions are:\n{}".format("\n".join([
            "\t\t- {}".format(version)
            for version in versions
        ]))

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
        return '\t' + "\n\t".join(text.split("\n"))

    def format_callbacks_data(self, graph_name: str, version: str) -> str:
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
            callbacks = self.get_callbacks(graph_name, version)
            if callbacks is None:
                return ""
            callbacks_data = self.get_callbacks_arguments(graph_name, version)
            json_formatted = json.dumps(
                callbacks_data,
                indent=4
            )
            return ",\n" + self.add_tabs(self.add_tabs("\n".join((
                "callbacks=[\n{}\n],".format(self.add_tabs("\n".join(
                    callbacks
                ))),
                f"callbacks_arguments={json_formatted}",
            ))))
        except NotImplementedError:
            return ""

    def format_graph_retrieval_file(
        self,
        graph_name: str,
        graph_method_name: str,
        references: List[str],
        versions: List[str],
        has_unique_references: bool
    ) -> str:
        """Return formatted model.

        Parameters
        ---------------------
        graph_name: str,
            Name of the graph to retrieve.
        references: List[str],
            List of the references of the graph.

        Returns
        ---------------------
        Formatted model of the report.
        """
        return self._graph_method_model.format(
            graph_method_name=graph_method_name,
            repository_package_name=self.repository_package_name,
            graph_name=graph_name,
            repository_name=self.get_formatted_repository_name(),
            callbacks_data=self.format_callbacks_data(
                graph_name,
                versions[-1]
            ),
            description=self.format_lines(
                self.get_description(graph_name, versions[-1])
            ),
            tabbed_description=self.add_tabs(
                self.format_lines(self.get_description(
                    graph_name, versions[-1]))
            ),
            references=self.format_references(references),
            tabbed_references="" if has_unique_references else self.add_tabs(
                "\n\n{}".format(self.format_references(references))
            ),
            default_version="latest" if "latest" in versions else versions[-1],
            available_graph_versions=self.add_tabs(
                self.format_versions(versions)
            ),
        )

    def get_graph_retrieval_import(self) -> str:
        """Return what should be imported as automatic graph retrieval class."""
        return "from .graph_retrieval import RetrievedGraph"

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
        target_directory_path = os.path.join(
            "../bindings/python/ensmallen/datasets",
            self.repository_package_name,
        )
        file_path = f"{target_directory_path}.py"

        imports = []

        for graph_data_path in tqdm(
            glob(os.path.join(
                os.path.dirname(os.path.abspath(__file__)),
                "graph_repositories",
                self.get_formatted_repository_name(),
                "*.json.gz"
            )),
            desc=f"Building graph retrieval methods for {self.name}",
            leave=False
        ):
            graph_data = compress_json.load(graph_data_path)
            if len(list(graph_data.values())) == 0:
                raise ValueError(
                    f"The graph {graph_data_path} has no versions."
                )
            first_graph_version_data = list(graph_data.values())[0]
            graph_name = first_graph_version_data["graph_name"]
            packages_to_import = self.get_imports(
                graph_name, list(graph_data.keys())[-1])
            if packages_to_import:
                imports.append(packages_to_import)

        imports = list(set(imports))

        first_references = list(compress_json.load(glob(os.path.join(
            os.path.dirname(os.path.abspath(__file__)),
            "graph_repositories",
            self.get_formatted_repository_name(),
            "*.json.gz"
        ))[0]).values())[0]["references"]

        has_unique_references = all(
            list(compress_json.load(path).values())[
                0]["references"] == first_references
            for path in glob(os.path.join(
                os.path.dirname(os.path.abspath(__file__)),
                "graph_repositories",
                self.get_formatted_repository_name(),
                "*.json.gz"
            ))
        ) and first_references

        file = open(file_path, "w")
        file.write("\n".join([
            "\"\"\"Module providing graphs available from {repository_name}.{references}\"\"\"".format(
                repository_name=self.get_formatted_repository_name(),
                references="\n\n{}\n".format(self.format_references(
                    first_references)) if has_unique_references else ""
            ),
            "from ensmallen import Graph  # pylint: disable=import-error",
            self.get_graph_retrieval_import(),
            *imports,
            "",
            ""
        ]))
        graph_repository_metadata = {}
        for graph_data_path in tqdm(
            glob(os.path.join(
                os.path.dirname(os.path.abspath(__file__)),
                "graph_repositories",
                self.get_formatted_repository_name(),
                "*.json.gz"
            )),
            desc="Building graph retrieval methods for {}".format(self.name),
            leave=False,
            dynamic_ncols=True
        ):
            graph_data = compress_json.load(graph_data_path)
            first_graph_version_data = list(graph_data.values())[0]
            graph_name = first_graph_version_data["graph_name"]
            graph_method_name = first_graph_version_data["graph_method_name"]
            graph_retrieval_file = self.format_graph_retrieval_file(
                graph_name=graph_name,
                graph_method_name=graph_method_name,
                references=first_graph_version_data["references"],
                versions=list(graph_data.keys()),
                has_unique_references=has_unique_references
            )
            for value in graph_data.values():
                value.pop("references")
            graph_repository_metadata[graph_method_name] = graph_data
            file.write(graph_retrieval_file)

        file.close()
        compress_json.dump(
            graph_repository_metadata,
            "{}.json.gz".format(target_directory_path)
        )
