import sys
import os
import time
from pyais.stream import FileReaderStream

def main():
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <file_path>")
        sys.exit(1)

    input_file = sys.argv[1]
    output_dir = "/home/sabil/Desktop/stage/pyais_parsing_from_file/ais_parsed"
    os.makedirs(output_dir, exist_ok=True)
    output_file = os.path.join(output_dir, os.path.basename(input_file))

    # Get the size of the input file
    input_file_size = os.path.getsize(input_file)

    unknown_or_invalid_messages = 0
    parsed_messages = 0
    start_time = time.time()

    try:
        with open(output_file, 'w') as out_file:
            for msg in FileReaderStream(input_file):
                try:
                    decoded = msg.decode()
                    out_file.write(f"{decoded}\n")
                    parsed_messages += 1
                except Exception:
                    unknown_or_invalid_messages += 1

    except Exception as e:
        print(f"Error reading or decoding file: {e}")
        sys.exit(1)

    end_time = time.time()
    elapsed_time = end_time - start_time

    # Get the size of the output file
    output_file_size = os.path.getsize(output_file)

    print(f"Size of input file: {input_file_size} bytes")
    print(f"Size of output file: {output_file_size} bytes")
    print(f"Parsed messages: {parsed_messages}")
    print(f"Unknown or invalid messages: {unknown_or_invalid_messages}")
    print(f"Time taken to parse messages: {elapsed_time:.2f} seconds")

if __name__ == "__main__":
    main()
