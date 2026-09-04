#!/usr/bin/env python3
import sys
import os
import re

PATTERN = re.compile(
    r'App::debug\(\s*(\d+)\s*,\s*(\d+)\s*,\s*([a-zA-Z0-9_]+|"[^"]*")\s*\)'
)
REPL = r'App::new().size(Size::new(\1, \2)).debug_script(\3)'


def process_file(path):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    new_content, n = PATTERN.subn(REPL, content)
    if n > 0:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        return True
    return False


def main():
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <folder_or_file>")
        sys.exit(1)

    target = sys.argv[1]

    if os.path.isfile(target):
        if target.endswith('.rs') and process_file(target):
            print(target)
    elif os.path.isdir(target):
        for root, _, files in os.walk(target):
            for name in files:
                if name.endswith('.rs'):
                    p = os.path.join(root, name)
                    if process_file(p):
                        print(p)
    else:
        print(f"Error: '{target}' not found")
        sys.exit(1)


if __name__ == '__main__':
    main()