#!/usr/bin/env python3

import sys

def load_csv(path, cols):
    data = { }
    with open(path, "r") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split(",")
            if len(parts) != cols:
                sys.exit("Error: Column number mismatch file: " + path)
            row = []
            key = 0
            for i, x in enumerate(parts):
                if i == 0:
                    key = int(x)
                    continue
                row.append(float(x))
            if key in data:
                sys.exit("Error: Duplicate wavelength: " + path)
            data[key] = row
    return data


spec = load_csv("./CIE_std_illum_D65.csv", 2)
cmf = load_csv("./CIE_xyz_1931_2deg.csv", 4)
