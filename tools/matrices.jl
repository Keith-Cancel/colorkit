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

function load_vector(vec::Vector{String}, typ=BigFloat)
    parsed = Vector{typ}()
    for v in vec
        push!(parsed, parse(typ, v))
    end
    return parsed
end

function load_matrix3x3(m::Vector{String}, typ=BigFloat)
    return Matrix(reshape(load_vector(m, typ), 3, 3)')
end

# This value is from spectrum_xyz.jl printed with more precision than probably necessary.
D65 = load_vector(["0.9504705586542829699097014291854001056630", "1.0", "1.0888287363958846603044439158088782923057"])
#sRGB D65, and primaries.
m = rgb_matrix(
    BigFloat[
        parse(BigFloat, "0.64") parse(BigFloat, "0.33");
        parse(BigFloat, "0.30") parse(BigFloat, "0.60");
        parse(BigFloat, "0.15") parse(BigFloat, "0.06")
    ],
    D65
)
print_matrices("Linear sRGB Matrices", m)

# OkLab Matrices
# https://bottosson.github.io/posts/oklab/
# Aftering notcing some oddities, I know that OkLab was
# added as a CSS spec. Looking at the spec I found this
# github issue:
# https://github.com/w3c/csswg-drafts/issues/6642
# When reading what people were saying there I ended up finding
# this by the author of OkLab:
# https://github.com/w3c/csswg-drafts/issues/6642#issuecomment-945714988
# I should be able to use this generate better M1 and M1^-1
m0 = load_matrix3x3([
    "0.77849780", "0.34399940", "-0.12249720",
    "0.03303601", "0.93076195", "0.03620204",
    "0.05092917", "0.27933344", "0.66973739"
])
# calculate the matrix, using the white point.
# Now m1 * D65 = [1.0, 1.0, 1.0]
m1 = m0 ./ reshape((m0 * D65), :, 1)
#println(m1 * D65)

# Looking at what people have done for M2:
# https://github.com/w3c/csswg-drafts/issues/6642
# Seems like for M2 it's best to use the inverse in the sample code at:
# https://bottosson.github.io/posts/oklab/
# interpret that matrix as an f32 and then invert the inverse to get M2.
m2_inv = convert(Matrix{BigFloat}, load_matrix3x3([
        "1.0", "0.3963377774", "0.2158037573",
        "1.0", "-0.1055613458", "-0.0638541728",
        "1.0", "-0.0894841775", "-1.2914855480"
    ],
    Float32
))
m2 = inv(m2_inv)
print_header("Oklab matrices")
print_matrix("M1", m1)
print_matrix("M1^-1", inv(m1))
print_matrix("M2", m2)
print_matrix("M2^-1", m2_inv)


#=
Don't uses these directly.
m1 = load_matrix3x3([
    "0.8189330101", "0.3618667424", "-0.1288597137",
    "0.0329845436", "0.9293118715", "0.0361456387",
    "0.0482003018", "0.2643662691", "0.6338517070"
])
m2 = load_matrix3x3([
    "0.2104542553", "0.7936177850", "-0.0040720468",
    "1.9779984951", "-2.4285922050", "0.4505937099",
    "0.0259040371", "0.7827717662", "-0.8086757660"
])
# The of inv(m2) did not look quite correct to.
# The first column is almost 1.0, shouldn't it be 1.0?
# Looking here that is the case, gonna need to dig in a little more.
# https://www.w3.org/TR/css-color-4/#color-conversion-code
=#
