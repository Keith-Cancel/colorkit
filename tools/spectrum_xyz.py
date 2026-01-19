#!/usr/bin/env python3

import sys
import math

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
                x = float(x)
                if math.isnan(x):
                    x = 0.0
                row.append(float(x))
            if key in data:
                sys.exit("Error: Duplicate wavelength: " + path)
            data[key] = row
    return data

def calc(spec, cmf):
    x_raw = 0.0
    y_raw = 0.0
    z_raw = 0.0
    for wl, p in spec.items():
        p = p[0]
        if wl not in cmf:
            continue
        # http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
        xbar, ybar, zbar = cmf[wl]
        x_raw += xbar * p;
        y_raw += ybar * p;
        z_raw += zbar * p;
    # Normalize to Y
    ratio = 1.0 / y_raw
    return (x_raw * ratio, 1.0, z_raw * ratio)

def xyz_to_xyY(c):
    # http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
    denom = c[0] + c[1] + c[2]
    x = c[0] / denom
    y = c[1] / denom
    Y = c[1]
    return (x, y, Y)

def print_white_point(name, xyz, xyY):
    print("~~~~~~~ " + name + " ~~~~~~~")
    print("================================")
    print("White Point XYZ\n    X: {:.16f}\n    Y: {:.16f}\n    Z: {:.16f}".format(xyz[0], xyz[1], xyz[2]))
    print("White Point Chromaticity\n    x: {:.16f}\n    y: {:.16f}\n    Y: {:.16f}".format(xyY[0], xyY[1], xyY[2]))


# https://cie.co.at/datatable/cie-standard-illuminant-d65
spec = load_csv("./CIE_std_illum_D65.csv", 2)
# https://cie.co.at/datatable/cie-1931-colour-matching-functions-2-degree-observer
cmf = load_csv("./CIE_xyz_1931_2deg.csv", 4)
xyz = calc(spec, cmf)
xyY = xyz_to_xyY(xyz)
print_white_point("D65 2 Degree FOV", xyz, xyY)

# https://cie.co.at/datatable/cie-1964-colour-matching-functions-10-degree-observer
cmf = load_csv("./CIE_xyz_1964_10deg.csv", 4)
xyz = calc(spec, cmf)
xyY = xyz_to_xyY(xyz)
print_white_point("D65 10 Degree FOV", xyz, xyY)
