import sys
import phplus_rust as phplus

def main():
    if len(sys.argv) < 2:
        print("Usage: ezlang <filename.ez>")
        return

    filename = sys.argv[1]

    try:
        with open(filename, 'r') as f:
            code = f.read()

        tokens = phplus.lexer(code)
        php_result = phplus.transpile(tokens)


        output_filename = filename.replace('.ez', '.php')
        with open(output_filename, 'w') as f:
            f.write(php_result)

        print(f"Successfully compiled {filename} to {output_filename}")

    except FileNotFoundError as e:
        print(f"Error: File {filename} not found")
    except RuntimeError as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()