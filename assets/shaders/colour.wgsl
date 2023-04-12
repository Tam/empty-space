#define_import_path empty_space::colour

// Convert linear colour-space to sRGB
// https://www.shadertoy.com/view/Wd2yRt
fn lin2srgb(cl : vec3<f32>) -> vec3<f32> {
    let c_lo = 12.92 * cl;
    let c_hi = 1.055 * pow(cl,vec3(0.41666)) - 0.055;
    let s = step(vec3(0.0031308), cl);
    return mix(c_lo, c_hi, s);
}
