#!/usr/bin/env python3
"""
Parse Fibonacci reductions from debug logs.
Extracts the fibonacci numbers that were subtracted during range reduction.
"""

import json
import sys


def parse_fibonacci_reductions(log_file):
    """Parse fibonacci reductions from log file and output them line-delimited."""
    fib_numbers = []

    with open(log_file, 'r') as f:
        for line in f:
            line = line.strip()
            # Skip empty lines and non-JSON lines
            if not line or not line.startswith('{'):
                continue

            try:
                log_entry = json.loads(line)

                # Check if this is a fibonacci subtraction log entry
                if (log_entry.get('level') == 'DEBUG' and
                    log_entry.get('fields', {}).get('message') == 'Found Fibonacci number to subtract'):

                    fib = log_entry['fields'].get('fib')
                    fib_index = log_entry['fields'].get('fib_index')

                    if fib is not None:
                        fib_numbers.append((fib_index, fib))

            except json.JSONDecodeError:
                # Skip lines that aren't valid JSON
                continue

    return fib_numbers


def main():
    if len(sys.argv) < 2:
        print("Usage: python parse_fib_reductions.py <log_file>")
        print("Example: python parse_fib_reductions.py run_output_debug.log")
        sys.exit(1)

    log_file = sys.argv[1]
    fib_numbers = parse_fibonacci_reductions(log_file)

    if not fib_numbers:
        print("No Fibonacci reductions found in log file.", file=sys.stderr)
        sys.exit(1)

    # Output each fibonacci index and number on its own line
    for fib_index, fib in fib_numbers:
        print(f"{fib_index}\t{fib}")

    # Print summary to stderr so it doesn't interfere with the output
    print(f"\nTotal reductions: {len(fib_numbers)}", file=sys.stderr)


if __name__ == '__main__':
    main()
