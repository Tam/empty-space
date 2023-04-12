#define_import_path empty_space::starfield
#import empty_space::colour
#import bevy_sprite::mesh2d_view_bindings

// From Resolution independent starfield by AntoineC
// https://www.shadertoy.com/view/tst3WS

// Dave Hoskins hash functions

fn hash_2_2 (p : vec2<f32>) -> vec2<f32> {
	var p3 = fract(vec3(p.xyx) * vec3(443.897, 441.423, 437.195));
	p3 += dot(p3, p3.yzx + 19.19);

	return -1.0 + 2.0 * fract((p3.xx + p3.yz) * p3.zy);
}

fn hash_2_4 (p : vec2<f32>) -> vec4<f32> {
	var p4 = fract(vec4(p.xyxy) * vec4(.1031, .1030, .0973, .1099));
	p4 += dot(p4, p4.wzxy + 19.19);

	return fract((p4.xxyz + p4.yzzw) * p4.zywx) - 0.5;
}

// IQ's Gradient Noise
fn gradient_2d (p : vec2<f32>) -> f32 {
	let i = floor(p);
	let f = fract(p);
	let u = f * f * (3. - 2. * f);

	return mix(
		mix(
			dot(hash_2_2(i + vec2(0., 0.)), f - vec2(0., 0.)),
			dot(hash_2_2(i + vec2(1., 0.)), f - vec2(1., 0.)),
			u.x,
		),
		mix(
			dot(hash_2_2(i + vec2(0., 1.)), f - vec2(0., 1.)),
            dot(hash_2_2(i + vec2(1., 1.)), f - vec2(1., 1.)),
            u.x,
		),
		u.y
	);
}

const COLD : vec3<f32> = vec3<f32>(1., 0.95686275, 0.74117647);
const HOT  : vec3<f32> = vec3<f32>(0.70980392, 0.9254902, 1.);

fn starfield_layer (
	p_in       : vec2<f32>,
	du_in      : f32,
	count      : f32,
	brightness : f32,
	size       : f32,
) -> vec3<f32> {
	// Tiling
	let du = du_in * count;
	var p = p_in * count;
	let pi = floor(p);
	p = fract(p) - 0.5;

	// Rand pos / brighness / spectrum
	let h = hash_2_4(p);

	// Resolution independent radius
	let s = brightness
		  * (0.7 + 0.6 * h.z)
		  * smoothstep(0.8 * du, -0.2 * du, length(p + 0.9 * h.xy) - size * du);

	return s * mix(
		mix(vec3(1.), COLD, min(1., -2. * h.w)),
		HOT,
		max(0., 2. * h.w)
	);
}

fn starfield (uv : vec2<f32>, offset : vec2<f32>) -> vec4<f32> {
	let du = 1. / view.viewport.w;
	let p = ((-1. + 2. * uv) * 0.5) + 1.33 + offset;

	var c = vec3(0.);

//	c  = starfield_layer(p, du, 25.0, 0.18, 0.5);
//	c += starfield_layer(p, du, 15.0, 0.25, 0.5);
	c += starfield_layer(p, du, 12.0, 0.50, 0.5);
	c += starfield_layer(p, du,  5.0, 1.00, 0.5);
	c += starfield_layer(p, du,  3.0, 1.00, 0.9);

	// Cluster
//	let s = 3.5 * (max(0.2, gradient_2d(2. * p * vec2<f32>(1.2, 1.9))) - 0.2) / (1. - 0.2);
//	c += s * starfield_layer(p, du, 160.0, 0.10, 0.5);
//	c += s * starfield_layer(p, du,  80.0, 0.15, 0.5);
//	c += s * starfield_layer(p, du,  40.0, 0.25, 0.5);
//	c += s * starfield_layer(p, du,  30.0, 0.50, 0.5);
//	c += s * starfield_layer(p, du,  20.0, 1.00, 0.5);
//	c += s * starfield_layer(p, du,  10.0, 1.00, 0.9);

//	c *= 1.3;
	c *= 10.3;

	// Resolution independent brightness
	let f = 1. / sqrt(660. * du);

	return vec4<f32>(lin2srgb(f * vec3<f32>(
		min(c.r, 1.),
		min(c.g, 1.),
		min(c.b, 1.),
	)), 1.);
}
