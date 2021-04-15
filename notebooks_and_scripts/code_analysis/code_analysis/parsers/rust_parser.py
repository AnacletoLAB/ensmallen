import os, re
import json

from .utils import *

class RustParser:
    def __init__(self):
        self.functions = []
        self.doc = []

    def skip_to_match(self, text:str):
        """Find the next matching parenthesis and return all the text whitin."""
        par_wanted = text[0]
        closing_wanted = {
            "{":"}",
            "<":">",
            "[":"]",
            "(":")",
        }[par_wanted]

        wanted = 0
        skipped = ""

        while True:
            if text[0] == par_wanted:
                wanted += 1                
            elif text[0] == closing_wanted:
                wanted -= 1
            skipped += text[0]
            text = text[1:]

            if wanted == 0:
                break

        text = text.lstrip()
        skipped = skipped[1:-1]
        return skipped, text

    def parse_struct(self, text):
        _, text = partition(text, "{")
        _, text = self.skip_to_match("{" + text)
        return text

    def parse_use(self, text):
        _line, text = read_line(text) 
        return text

    def parse_attr(self, text):
        _skipped_attr, text = read_line(text)
        return text

    def parse_extern(self, text):
        _, text = partition(text, "{")
        _, text = self.skip_to_match("{" + text)
        return text


    def parse_doc(self, text:str) -> str:
        """Parse a documentation line"""
        doc_line, text = read_line(text[3:])
        self.doc.append(doc_line.strip())
        return text

    def parse_args(self, arguments:str) -> str:
        """Parse the arguments of a function"""
        result = []
        maybe_self, text = partition(arguments, ",")

        if "self" in maybe_self:
            result.append(("self", maybe_self.strip().rstrip(")").lstrip("(")))
            arguments = text

        flat = [
            remove_prefix(x.strip(), "mut").strip()
            for comb in arguments.split(":")
            for x in comb.rsplit(",", 1)
        ]
        
        result += [
                (arg_name, arg_type)
                for (arg_name, arg_type) in zip(flat[::2], flat[1::2])
        ]
        return result            
        
    def parse_identifier(self, text:str):
        """Parse a function, struct, or variable name"""
        identifier = ""
        while text[0] not in "<{[()]}>? \t\r\n":
            identifier += text[0]
            text = text[1:]
        return identifier, text

    def parse_type(self, text:str):
        """Parse a type"""
        parsed_type, text = self.parse_identifier(text)

        if text[0] == "<":
            generics, text = self.skip_to_match(text)
            parsed_type += "<" + generics + ">"

        return parsed_type, text

    def parse_function(self, text:str) -> str:
        """Parse a function declaration"""
        function = {
            "file":self.file,
        }
        if self.struct_name is not None:
            function["struct"] = self.struct_name
        # If we parsed some documentation we add it to the current function
        # and reset it.
        function["doc"] = self.doc
        self.doc = []
        ########################################################################
        modifiers, text = partition(text, "fn")
        function["modifiers"] = modifiers.strip()

        name, text = self.parse_identifier(text)
        function["name"] = name
        print(name)

        ########################################################################
        # Parse the generics if present
        if text[0] == "<":
            generics, text = self.skip_to_match(text)
            function["generics"] = generics
        
        # Parse the arguments
        args, text = self.skip_to_match(text)
        function["args"] = self.parse_args(args)
        ########################################################################

        # Parse the return arguments
        if text.startswith("->"):
            text = text[2:].strip()
            return_type, text = self.parse_type(text)

            while text[0] not in "{":
                return_type += text[0]
                text = text[1:]

            function["return_type"] = return_type.strip()

        # Pase the body
        body, text = self.skip_to_match(text)
        function["body"] = body.strip()

        # add the function
        self.functions.append(function)
        return text

    def parse_impl(self, text):
        # Reset the doc if present
        # we don't care about impl documentation.
        self.doc = []
        # Get the name of the struct
        self.struct_name = re.match(r"\s*impl\s+(\S+)\s+{", text).groups()[0]
        # skip to the end of the impl definition
        _, _, text = text.partition(self.struct_name)
        # Get all the text inside the current impl
        to_parse, text = self.skip_to_match(text.strip())
        # Parse all the content 
        self.start(to_parse.strip())
        # Reset the struct name
        self.struct_name = None
        return text

    def start(self, text):
        """Main entrypoint of the parser."""
        while text:
            # Remove the white space
            text = text.lstrip()

            # Check if it's an impl
            if re.match(r"\s*impl\s+(\S+)\s+{", text):
                text = self.parse_impl(text)
            # Check if it's a function
            elif re.match(r"\s*(pub(\(crate\))?\s+)?fn\s+", text):
                text = self.parse_function(text)
            # iCheck if it's a struct
            elif re.match(r"\s*(pub(\(crate\))?\s+)?struct\s+", text):
                text = self.parse_struct(text)
            elif text.startswith("use"):
                text = self.parse_use(text) 
            elif text.startswith("///"):
                text = self.parse_doc(text)
            elif text.startswith("#["):
                text = self.parse_attr(text)
            elif text.startswith("extern"):
                text = self.parse_extern(text)
            else:
                _skipped_line, text = read_line(text)
                print("Skipping line: '{}'".format(_skipped_line.strip()))

    def parse_files(self, files):
        for file, txt in files:
            self.file = os.path.abspath(file)
            self.start(txt)
