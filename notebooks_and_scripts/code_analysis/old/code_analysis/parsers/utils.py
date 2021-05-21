def remove_prefix(text, prefix):
    if text.startswith(prefix):
        return text[len(prefix):]
    return text  # or whatever

def partition(text, pattern):
    res, _, text = text.partition(pattern)
    return res, text.lstrip()

def read_line(text):
    return partition(text, "\n")