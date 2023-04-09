#import bevy_sprite::mesh2d_view_bindings

const PI: f32 = 3.141592653589793;
const TINT : vec3<f32> = vec3<f32>(0.12941176, 0.16862745, 0.21176471);

struct FragmentInput {
	#import bevy_sprite::mesh2d_vertex_output
}

@fragment
fn fragment (in: FragmentInput) -> @location(0) vec4<f32> {
	let st = -1. + 2. * in.uv;
	var color = vec3<f32>(0.);
	var m = 0.;

	// Rings
	var rings = length(st * 3.);
	rings = 1. - step(0.02, 1. - fract(rings));
	color = vec3<f32>(rings);

	// Sweep
	var a = atan2(st.y, st.x); // [-pi, pi]
	a = (a + PI) / (2. * PI);  // [0, 1]
	a = f_mod(globals.time * 0.5 - a, 1.);
	let r = 1. - length(st);
	color = max(color, vec3<f32>(a * r)) * TINT * 0.5;

	// Lines
	for (var i = 0; i < 4; i++) {
		var temp_st = st * rotate_2d(45. * PI / 180.);
		temp_st *= rotate_2d((f32(i) * 90.) * PI / 180.);
		var l = sdf_line(temp_st, vec2<f32>(0., 0.), vec2<f32>(0., 1.));
		l = sharpen(l, 0.003, view.viewport.zw);
		m += l;
	}
	color = max(vec3(m) * TINT * 0.5, color);

	// Blips
	let blip = st * 30. - vec2(-8., -4.);
	let r_blip = length(blip);
	let circle = smoothstep(a, a + 0.7, r_blip) - smoothstep(a + 0.35, a + 0.9, r_blip);
	color = max(color, vec3(circle) * vec3(1., 0., 0.));

	return vec4<f32>(color, 1.);
}

fn rotate_2d (angle : f32) -> mat2x2<f32> {
	return mat2x2<f32>(
		cos(angle), -sin(angle),
		sin(angle),  cos(angle),
	);
}

fn sdf_line (p : vec2<f32>, a : vec2<f32>, b : vec2<f32>) -> f32 {
	let pa = p - a;
	let ba = b - a;
	let h = clamp(dot(pa, ba) / dot(ba, ba), 0., 1.);

	return length(pa - ba * h);
}

fn sharpen (d : f32, w : f32, resolution : vec2<f32>) -> f32 {
	let e = 1. / min(resolution.y, resolution.x);

	return 1. - smoothstep(-e, e, d - w);
}

fn f_mod (x : f32, y : f32) -> f32 {
	return x - y * floor(x / y);
}
