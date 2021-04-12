import re
import json
import glob
from tqdm.auto import tqdm

path = "../../graph/src/*.rs"

def read_file(path):
    with open(path, "r") as f:
        return f.read()
    
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

    def comment(self, text):
        comment_line, _, text = text.partition("\n")
        self.current_function.setdefault("doc", []) 
        self.current_function["doc"].append(comment_line.strip())
        return text

    def attr(self, text):
        attr, _, text = text.partition("\n")
        self.current_function.setdefault("attrs", []) 
        self.current_function["attrs"].append(attr[:-1].strip())
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
        
    def parse_values(self, text):
        while text:
            # Remove the white space
            text = text.lstrip()
            
            if text.startswith("///"):
                text = self.comment(text[3:])
            elif text.startswith("//"):
                _, _, text = text.partition("\n")
            elif text.startswith("#["):
                text = self.attr(text[2:])
            elif re.match(r"\s*(pub(\(crate\))?\s+)?fn\s+", text) is not None:
                text = self.function(text)
            else:
                skipped, _, text = text.partition("\n")
                print("skipping the current line: [%s]"%skipped)

        if self.current_function != {}:
            self.functions.append(self.current_function)


    def start(self, text):
        while text:
            # Remove the white space
            text = text.lstrip()

            if re.match(r"\s*impl\s+Graph\s+", text):
                _, _, text = text.partition("Graph")
                to_parse, text = self.skip_to_match(text.strip())
                self.parse_values(to_parse.strip())
            else:
                _, _, text = text.partition("\n")

txt = get_all_impls(path)

p = Parser()
p.start(txt)

with open("analysis.json", "w") as f:
    f.write(json.dumps(p.functions, indent=4))
