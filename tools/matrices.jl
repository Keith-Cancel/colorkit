#!/usr/bin/env -S julia --color=yes --startup-file=no
using Printf

setprecision(BigFloat, 128)

function xyY_to_XYZ(xy::Vector{BigFloat}, Y::BigFloat)
    X = xy[1] / xy[2]
    Z = (1.0 - xy[1] - xy[2]) / xy[2]
    return [X * Y, Y, Z * Y]
end

#http://www.brucelindbloom.com/index.html?Eqn_RGB_XYZ_Matrix.html
function rgb_matrix(p::Array{BigFloat}, wp::Vector{BigFloat})
    r = xyY_to_XYZ(p[1, :], BigFloat(1.0))
    g = xyY_to_XYZ(p[2, :], BigFloat(1.0))
    b = xyY_to_XYZ(p[3, :], BigFloat(1.0))
    S = inv(hcat(r, g, b)) * wp
    M = hcat(r * S[1], g * S[2], b * S[3])
    return M
end

function print_matrix(name::String, m::Matrix{BigFloat})
    println("$(name):")
    @printf("    %19.016lf, %19.016lf, %19.016lf,\n", m[1, 1], m[1, 2], m[1, 3])
    @printf("    %19.016lf, %19.016lf, %19.016lf,\n", m[2, 1], m[2, 2], m[2, 3])
    @printf("    %19.016lf, %19.016lf, %19.016lf,\n", m[3, 1], m[3, 2], m[3, 3])
end

function print_header(name::String)
    name = " $(name) "
    fill = 40 - length(name)
    left = div(fill, 2)
    right = fill - left
    header = repeat("~", left) * name * repeat("~", right)
    println(header)
    println(repeat("=", length(header)))
end

function print_matrices(name::String, m::Matrix{BigFloat})
    print_header(name)
    print_matrix("Into XYZ (M)", m)
    print_matrix("From XYZ (M^-1)", inv(m))
end

function load_matrix(m::Vector{String})
    parsed = Vector{BigFloat}()
    for v in m
        push!(parsed, parse(BigFloat, v))
    end
    return Matrix(reshape(parsed, 3, 3)')
end


#sRGB D65, and primaries.
m = rgb_matrix(
    BigFloat[
        parse(BigFloat, "0.64") parse(BigFloat, "0.33");
        parse(BigFloat, "0.30") parse(BigFloat, "0.60");
        parse(BigFloat, "0.15") parse(BigFloat, "0.06")
    ],
    # This value is from spectrum_xyz.jl
    BigFloat[parse(BigFloat, "0.9504705586542830"), 1.0, parse(BigFloat, "1.0888287363958847")],
)
print_matrices("Linear sRGB Matrices", m)

# OkLab Matrices
# https://bottosson.github.io/posts/oklab/
m1 = load_matrix([
    "0.8189330101", "0.3618667424", "-0.1288597137",
    "0.0329845436", "0.9293118715", "0.0361456387",
    "0.0482003018", "0.2643662691", "0.6338517070"
])
m2 = load_matrix([
    "0.2104542553", "0.7936177850", "-0.0040720468",
    "1.9779984951", "-2.4285922050", "0.4505937099",
    "0.0259040371", "0.7827717662", "-0.8086757660"
])
print_header("Oklab matrices")
print_matrix("M1", m1)
print_matrix("M1^-1", inv(m1))
print_matrix("M2", m2)
print_matrix("M2^-1", inv(m2))
