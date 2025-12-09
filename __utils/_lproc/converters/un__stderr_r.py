#!/usr/bin/env python3
import sys

def main():
    data = sys.stdin.buffer.read()
    sys.stdout.buffer.write(data)

if __name__ == "__main__":
    main()

