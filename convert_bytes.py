#!/usr/bin/env python3
"""
Python script to convert bytes (from command line arguments, in hex form) to integer/utf-8
Also prefixes them with a 0 and removes commas to work with Rust's hex formatting
"""

import sys


def add_zero(byte: str):
    byte = byte.replace(",", "")
    if len(byte) == 1:
        byte = "0" + byte
    return "\\x" + byte


raw = "".join(list(map(add_zero, sys.argv[1:])))

print(raw)
print()
print(bytes.fromhex(raw.replace("\\x", "")).decode("utf-8"))
