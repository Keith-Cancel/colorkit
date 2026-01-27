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

function print_matrices(name::String, m::Matrix{BigFloat})
    name = " $(name) "
    fill = 40 - length(name)
    left = div(fill, 2)
    right = fill - left
    header = repeat("~", left) * name * repeat("~", right)
    println(header)
    println(repeat("=", length(header)))
    println("Into XYZ:")
    @printf("    %.016lf, %.016lf, %.016lf,\n", m[1, 1], m[1, 2], m[1, 3])
    @printf("    %.016lf, %.016lf, %.016lf,\n", m[2, 1], m[2, 2], m[2, 3])
    @printf("    %.016lf, %.016lf, %.016lf,\n", m[3, 1], m[3, 2], m[3, 3])
    m = inv(m)
    println("From XYZ:")
    @printf("    %.016lf, %.016lf, %.016lf,\n", m[1, 1], m[1, 2], m[1, 3])
    @printf("    %.016lf, %.016lf, %.016lf,\n", m[2, 1], m[2, 2], m[2, 3])
    @printf("    %.016lf, %.016lf, %.016lf,\n", m[3, 1], m[3, 2], m[3, 3])
end


#sRGB D65
m = rgb_matrix(
    BigFloat[
        parse(BigFloat, "0.64") parse(BigFloat, "0.33");
        parse(BigFloat, "0.30") parse(BigFloat, "0.60");
        parse(BigFloat, "0.15") parse(BigFloat, "0.06")
    ],
    BigFloat[parse(BigFloat, "0.9504705586542830"), 1.0, parse(BigFloat, "1.0888287363958847")],
)
print_matrices("Linear sRGB Matrices", m)
