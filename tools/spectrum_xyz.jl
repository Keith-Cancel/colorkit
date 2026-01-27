#!/usr/bin/env -S julia --color=yes --startup-file=no
using Printf

function load_csv(path::String, cols::Int)
    ret = Dict{Int,Vector{Float64}}()
    open(path) do file
        for ln in eachline(file)
            ln = strip(ln)
            if length(ln) == 0 || first(ln) == "#"
                continue
            end

            parts = split(ln, ",")
            if length(parts) != cols
                println(stderr, "Error: Column number mismatch file: '$(path)'")
                exit(1)
            end

            key = parse(Int, parts[1])
            if haskey(ret, key)
                println(stderr, "Error: Duplicate wavelength: '$(path)'")
                exit(1)
            end

            dat = Vector{Float64}()
            for v in parts[2:end]
                v = parse(Float64, v)
                if isnan(v)
                    v = 0.0
                end
                push!(dat, v)
            end
            ret[key] = dat
        end
    end
    return ret
end

function calc(spec, cmf)
    x_raw = 0.0
    y_raw = 0.0
    z_raw = 0.0
    for (wl, p) in spec
        p = p[1]
        if !haskey(cmf, wl)
            continue
        end
        # http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
        xbar, ybar, zbar = cmf[wl]
        x_raw += xbar * p
        y_raw += ybar * p
        z_raw += zbar * p
    end
    ratio = 1.0 / y_raw
    return (x_raw * ratio, 1.0, z_raw * ratio)
end

function xyz_to_xyY(c)
    # http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
    denom = c[1] + c[2] + c[3]
    x = c[1] / denom
    y = c[2] / denom
    Y = c[2]
    return (x, y, Y)
end

function print_white_point(name, xyz, xyY)
    name = " $(name) "
    fill = 40 - length(name)
    left = div(fill, 2)
    right = fill - left
    header = repeat("~", left) * name * repeat("~", right)
    println(header)
    println(repeat("=", length(header)))

    println("White Point XYZ")
    @printf("    const X: f32 = %.016lf;\n", xyz[1])
    @printf("    const Z: f32 = %.016lf;\n", xyz[3])
    @printf("    const Y: f32 = %.016lf;\n", xyz[2])
    println("White Point Chromaticity")
    @printf("    const x_i: f32 = %.016lf;\n", xyY[1])
    @printf("    const y_i: f32 = %.016lf;\n", xyY[2])
    @printf("    const Y:   f32 = %.016lf;\n", xyY[3])
end

# https://cie.co.at/datatable/cie-1931-colour-matching-functions-2-degree-observer
cmf2 = load_csv("./CIE_xyz_1931_2deg.csv", 4)
# https://cie.co.at/datatable/cie-1964-colour-matching-functions-10-degree-observer
cmf10 = load_csv("./CIE_xyz_1964_10deg.csv", 4)

# https://cie.co.at/datatable/cie-standard-illuminant-d65
spec = load_csv("./CIE_std_illum_D65.csv", 2)
xyz = calc(spec, cmf2)
xyY = xyz_to_xyY(xyz)
print_white_point("D65 2 Degree FOV", xyz, xyY)

xyz = calc(spec, cmf10)
xyY = xyz_to_xyY(xyz)
print_white_point("D65 10 Degree FOV", xyz, xyY)

# https://cie.co.at/datatable/cie-standard-illuminant-d50
spec = load_csv("./CIE_std_illum_D50.csv", 2)
xyz = calc(spec, cmf2)
xyY = xyz_to_xyY(xyz)
print_white_point("D50 2 Degree FOV", xyz, xyY)

xyz = calc(spec, cmf10)
xyY = xyz_to_xyY(xyz)
print_white_point("D50 10 Degree FOV", xyz, xyY)

# https://cie.co.at/datatable/relative-spectral-power-distributions-cie-illuminant-d55
spec = load_csv("./CIE_illum_D55.csv", 2)
xyz = calc(spec, cmf2)
xyY = xyz_to_xyY(xyz)
print_white_point("D55 2 Degree FOV", xyz, xyY)

xyz = calc(spec, cmf10)
xyY = xyz_to_xyY(xyz)
print_white_point("D55 10 Degree FOV", xyz, xyY)

# https://cie.co.at/datatable/relative-spectral-power-distributions-cie-illuminant-d55
spec = load_csv("./CIE_illum_D75.csv", 2)
xyz = calc(spec, cmf2)
xyY = xyz_to_xyY(xyz)
print_white_point("D75 2 Degree FOV", xyz, xyY)

xyz = calc(spec, cmf10)
xyY = xyz_to_xyY(xyz)
print_white_point("D75 10 Degree FOV", xyz, xyY)
