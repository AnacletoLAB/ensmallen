import re
import json
import glob
from tqdm.auto import tqdm

path = "../../graph/src/*.rs"

def read_file(path):
    with open(path, "r") as f:
        return f.read()
    
def read_line(text):
    line, _, text = text.partition("\n")
    return line, text

def read_files(path):
    return [
        read_file(file)
        for file in glob.glob(path)
    ]

def get_all_impls(path):
    return "\n".join(read_files(path))

def remove_prefix(text, prefix):
    if text.startswith(prefix):
        return text[len(prefix):]
    return text  # or whatever

class Parser:
    def __init__(self):
        self.functions = []
        self.current_function = {}

    def doc(self, text):
        doc_line, text = read_line(text)
        self.current_function.setdefault("doc", []) 
        self.current_function["doc"].append(doc_line.strip())
        return text

    def skip_to_match(self, text):
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

    def parse_args(self, arguments):
        result = []
        maybe_self, _, text = arguments.partition(",")

        if "self" in maybe_self:
            result.append(("self", maybe_self[1:].strip().rstrip(")").lstrip("(")))
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
        

    def function(self, text):
        if set(self.current_function.keys()) != {"doc"}:
            self.functions.append(self.current_function)
            self.current_function = {}

        ########################################################################

        modifiers, _, text = text.partition("fn")
        self.current_function["modifiers"] = modifiers.strip()
        text = text.lstrip()

        name = ""
        while text[0] not in "(<":
            name += text[0]
            text = text[1:]
        self.current_function["name"] = name
        print(name)

        ########################################################################
        # Parse the generics if present
        if text[0] == "<":
            generics, text = self.skip_to_match(text)
            self.current_function["generics"] = generics
        
        # Parse the arguments
        args, text = self.skip_to_match(text)
        self.current_function["args"] = self.parse_args(args)
        ########################################################################

        # Parse the return arguments
        if text.startswith("->"):
            text = text[2:].strip()
            return_type = ""
            while text[0] not in "<{":
                return_type += text[0]
                text = text[1:]

            if text[0] == "<":
                generics, text = self.skip_to_match(text)
                return_type += "<" + generics + ">"

            while text[0] not in "{":
                return_type += text[0]
                text = text[1:]

            self.current_function["return_type"] = return_type.strip()

        # Pase the body
        body, text = self.skip_to_match(text)
        self.current_function["body"] = body.strip()
        return text

    def start(self, text):
        while text:
            # Remove the white space
            text = text.lstrip()

            # Check if the current line is an impl or a function
            impl_matches = re.match(r"\s*impl\s+(\S+)\s+{", text)
            func_matches = re.match(r"\s*(pub(\(crate\))?\s+)?fn\s+", text)
            if impl_matches is not None:
                # Reset the doc if present
                # we don't care about impl documentation.
                self.doc = []
                # Get the name of the struct
                self.struct_name = impl_matches.groups()[0]
                # skip to the end of the impl definition
                _, _, text = text.partition(self.struct_name)
                # Get all the text inside the current impl
                to_parse, text = self.skip_to_match(text.strip())
                # Parse all the content 
                self.start(to_parse.strip())
                # Reset the struct name
                self.struct_name = None
            elif func_matches is not None:
                text = self.function(text)
            # if we encounter a struct we skip it.
            elif text.startswith("use"):
                _line, text = read_line(text)  
            # if we encounter an use statement we just skip the line
            elif text.startswith("use"):
                _line, text = read_line(text)  
            # If it's a doc line, add it to the current buffer
            elif text.startswith("///"):
                text = self.doc(text[3:])
            # if we encounter an attribute we just skip the line
            elif text.startswith("#["):
                _skipped_attr, text = read_line(text)
            # Otherwise just skip the line
            else:
                _skipped_line, text = read_line(text)
                print("Skipping line: '{}'".format(_skipped_line.strip()))

txt = get_all_impls(path)

with open("debug.rs", "w") as f:
    f.write(txt)

p = Parser()
p.start(txt)

with open("analysis.json", "w") as f:
    f.write(json.dumps(p.functions, indent=4))
