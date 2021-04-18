import re
from typing import *

def parse_identifier(text: str) -> (str, str):
    """Parse an identifier, return the not-used text and the parsed identifier"""
    match = re.match(r"([a-zA-Z][a-zA-Z0-9_]*|_[a-zA-Z0-9_])")
    
    if match is None:
        raise ValueError("Cannot parse the current text as an identifier. The text is: %s"%text.split("\n")[0])

    return text[m.span()[1]:], match.group()    

def skip_to_match(text:str) -> (str, str):
    """Find the next matching parenthesis and return all the text whitin."""
    # Get the parenthesis to match
    par_wanted = text[0]
    # Try to get the matching parenthesis
    closing_wanted = {
        "{":"}",
        "<":">",
        "[":"]",
        "(":")",
    }.get(par_wanted, None)
    # If we could not retreive the matching parenthesis, 
    # then there is no parenthesis to match
    if closing_wanted is None:
        return text, None
    
    # Counter of how many parenthesis we encounter
    wanted = 0

    skipped = ""
    while True:
        # Get the next char
        current_char, *text = text
        skipped += current_char

        # when we encounter an open, increment
        if text[0] == par_wanted:
            wanted += 1                
        # when we encounter a close, decrement
        elif text[0] == closing_wanted:
            wanted -= 1
        # If wante is 0, we matched the closing of the initial open.
        if wanted == 0:
            break

    # Remove the outer parenthesis
    skipped = skipped[1:-1]
    return text, skipped

def skip_whitespace(text: str) -> str:
    while text[0] in " \t\n\r":
        _, text = *text
    return text
