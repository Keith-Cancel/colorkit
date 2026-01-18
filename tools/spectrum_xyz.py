#!/usr/bin/env python3

import sys

def load_csv(path, cols):
    rows = []
    with open(path, "r") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split(",")
            if len(parts) != cols:
                sys.exit("Error: Column number mismatch file: " + path)
                continue
            row = []
            
            rows.append([float(p) for p in parts])
    return rows


spec = load_csv("./CIE_std_illum_D65.csv", 2)
print(spec)
