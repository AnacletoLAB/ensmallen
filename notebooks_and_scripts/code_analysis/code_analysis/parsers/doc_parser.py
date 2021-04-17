import re

DOC_EXAMPLE = """
This is my great method

And the description can be on multiple lines

# Arguments
arguments desc
 * `arg`: type - desc
 * `arg`: type - desc
 * `arg`: type - desc
arguments footer

# Example
example desc
```
CODICE CODICE CODICE
```
example footer

# Extra header1
extraheader desc

# Extra header2
extraheader desc

# Extra header3
extraheader desc
"""


class DocParser:
    def __init__(self):
        self.description = []
        self.arguments = {
            "args":[]
        }
        self.example = {}
        self.errors = []
        self.extra_headers = []

    def log_error(self, error_line, msg):
        self.errors.append({
            "doc_line":error_line,
            "msg":msg,
            }
        )

    def get_args(self):
        return [
            x[0]
            for x in self.function.get("args", [])
            if x[0] != "self"
        ]

    def get_arg(self, wanted_arg_name):
        for arg_name, arg_type in self.function.get("args", []):
            if arg_name == wanted_arg_name:
                return arg_type
        return None

    def parse_description(self, text):
        # Just take the first values until
        if text and text[0][1].startswith("#"):
            self.log_error(text[0], "Missing description!!!!")
        # Capture all the lines until the first arg
        while text and not text[0][1].startswith("#"):
            line, *text = text
            self.description.append(line[1])

        return text

    def parse_arguments(self, text):
        # if there are no arguments, we don't want or need
        # the arguments section
        if len(self.get_args()) == 0:
            return text

        if not text:
            self.log_error(0, "Missing Arguments")
            return

        # Parse the arguments header
        if text[0][1] != "# Arguments":
            self.log_error(text[0][0], "The arguments section is missing or in the wrong order.")
            return text
        _header, *text = text

        # Parse the arguments description if present
        args_desc = ""
        while text and not text[0][1].startswith("*"):
            line, *text = text
            if line[1].strip() != "":
                args_desc += line[1] + "\n"
        
        if args_desc != "":
            self.arguments["desc"] = args_desc

        documented_args = set()
        # Parse the arguments
        while text and text[0][1].startswith("*"):
            line, *text = text
            match = re.match("\* `(\S+?)`: ([^-]+?) - (.+)", line[1])
            # Check if the format is the standard one.
            if match is None:
                self.log_error(
                    line[0], 
                    "The argument line '{}' (doc_line: {}) is not in the standard format.".format(
                        line[1], line[0]
                    )
                )
                continue
            # If the format is standard, get the arguments
            arg_name, arg_type, arg_desc = match.groups()

            real_arg_type = self.get_arg(arg_name)
            # Ensure that the argument exists
            if real_arg_type is None:
                self.log_error(line[0], 
                    "The argument '{}' does not exists in the current method. The available args are {}.".format(
                        arg_name, self.get_args()
                    )
                )
                continue

            # Ensure that the argument type matches
            if real_arg_type != arg_type:
                self.log_error(line[0], 
                    "The argument type '{}' of '{}' does not match the function declaration which is '{}'.".format(
                        arg_type, arg_name, real_arg_type
                    )
                )
                continue

            self.arguments["args"].append({
                "name":arg_name,
                "type":arg_type,
                "desc":arg_desc.strip(".").strip(","),
            })

            documented_args |= set([arg_name])
                
        for missed_arg in set(self.get_args()) - documented_args:
            self.log_error("~", 
                "Missing the documentation for the argument '{}'.".format(missed_arg)
            )
        # Capture the footer of arguments, if present
        args_footer = ""
        while text and not text[0][1].startswith("#"):
            line, *text = text
            if line[1].strip() != "":
                args_footer += line[1] + "\n"

        if args_footer != "":
            self.arguments["footer"] = args_footer
            
        return text
            

    def parse_example(self, text):
        # Parse the arguments header
        if text[0][1] != "# Example":
            self.log_error(text[0][0], "The example section is missing or in the wrong order.")
            return text

        line, *text = text
        # Parse the arguments description if present
        example_desc = ""
        while not text[0][1].startswith("```") and not text[0][1].startswith("#"):
            line, *text = text
            if line[1].strip() != "":
                example_desc += line[1] + "\n"
        
        if example_desc != "":
            self.example["desc"] = example_desc

        # Parse the example
        if not text[0][1].startswith("```"):
            self.log_error(text[0][0], "Missing the code example!")
            return text

        example = text[0][1]
        text = text[1:]

        while not text[0][1].startswith("```"):
            line, *text = text
            if line[1].strip() != "":
                example += line[1] + "\n"

        if self.function.get("name", "") not in example:
            self.log_error(line[0], "In the example there isn't the current method!!!!")
        
        self.example["content"] = example

        if text:
            line, *text = text
        # Capture the footer of arguments, if present
        example_footer = ""
        while text and not text[0][1].startswith("#"):
            line, *text = text
            if line[1].strip() != "":
                example_footer += line[1] + "\n"

        if example_footer != "":
            self.example["footer"] = example_footer
            
        return text

    def parse_extra_headers(self, text):
        while text:
            line, *text = text
            header_name = line[1][1:].strip()

            content = ""
            while text and not text[0][1].startswith("#"):
                line, *text = text
                if line[1].strip() != "":
                    content += line[1] + "\n"

            self.extra_headers.append({
                "name":header_name,
                "content":content
            })

    def start(self, function, text:str):
        self.function = function
        text = list(enumerate([x.strip() for x in text.split("\n")]))

        if text:
            text = self.parse_description(text)
        else:
            self.log_error(0, "Missing description")

        text = self.parse_arguments(text)

        if text:
            text = self.parse_example(text)
        else:
            self.log_error(0, "Missing Example")

        if text:
            text = self.parse_extra_headers(text)

        return {
            "file":self.function.get("file"),
            "function":self.function.get("name"),
            "errors":self.errors,
            "args":self.arguments,
            "example":self.example,
            "extra_headers":self.extra_headers,
        }