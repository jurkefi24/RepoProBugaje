import sys
import re


# --- 1. LEXER ---
def lexer(code):
    # Basic tokens: Keywords, Identifiers, Strings, and Operators
    token_specs = [
        ('PRINT', r'print\b'),  # print keyword
        ('LET', r'let\b'),  # let keyword
        ('STRING', r'"[^"]*"'),  # "text"
        ('ID', r'[a-zA-Z_]\w*'),  # variable names
        ('ASSIGN', r'='),  # assignment
        ('OP', r'\+'),  # plus operator
        ('NEWLINE', r'\n'),  # line breaks
        ('SKIP', r'[ \t]+'),  # spaces/tabs
        ('MISMATCH', r'.'),  # any other character
    ]

    tokens = []
    # Join all patterns into one regex
    master_re = '|'.join(f'(?P<{name}>{pattern})' for name, pattern in token_specs)

    for match in re.finditer(master_re, code):
        kind = match.lastgroup
        value = match.group()
        if kind == 'SKIP' or kind == 'NEWLINE':
            continue
        elif kind == 'MISMATCH':
            raise RuntimeError(f'Unexpected character: {value}')
        tokens.append((kind, value))
    return tokens


# --- 2. PARSER & GENERATOR ---
# For a simple school project, we can combine parsing and generating
# into one "Transpiler" class to keep the code short.

class EzTranspiler:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos = 0

    def peek(self):
        return self.tokens[self.pos] if self.pos < len(self.tokens) else (None, None)

    def consume(self):
        token = self.peek()
        self.pos += 1
        return token

    def transpile(self):
        php_output = "<?php\n\n"
        while self.pos < len(self.tokens):
            php_output += self.statement() + ";\n"
        return php_output

    def statement(self):
        kind, value = self.peek()

        if kind == 'LET':
            self.consume()  # consume 'let'
            _, var_name = self.consume()  # get variable name
            self.consume()  # consume '='
            _, val = self.consume()  # get value (string or id)
            # Convert to PHP variable style ($name)
            return f"${var_name} = {val}"

        if kind == 'PRINT':
            self.consume()  # consume 'print'
            _, val = self.consume()
            # If it's an ID, add a $ for PHP
            if _ == 'ID': val = f"${val}"
            return f"echo {val}"

        return ""


# --- 3. MAIN ENTRY POINT ---
def main():
    if len(sys.argv) < 2:
        print("Usage: ezlang <filename.ez>")
        return

    filename = sys.argv[1]

    try:
        with open(filename, 'r') as f:
            code = f.read()

        # Run the compiler phases
        tokens = lexer(code)
        transpiler = EzTranspiler(tokens)
        php_result = transpiler.transpile()

        # Output the result to a .php file
        output_filename = filename.replace('.ez', '.php')
        with open(output_filename, 'w') as f:
            f.write(php_result)

        print(f"Successfully compiled {filename} to {output_filename}")

    except FileNotFoundError:
        print(f"Error: File '{filename}' not found.")


if __name__ == "__main__":
    main()