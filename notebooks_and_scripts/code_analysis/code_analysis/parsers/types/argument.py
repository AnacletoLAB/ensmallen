class Argument:
    def __init__(self, arg_name: str, arg_type: str):
        self.arg_name = arg_name
        self.arg_type = arg_type

    def is_self(self):
        return self.arg_name