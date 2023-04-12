#import bevy_core_pipeline::fullscreen_vertex_shader
#import empty_space::starfield

@group(0) @binding(2)
var screen_texture : texture_2d<f32>;
@group(0) @binding(3)
var texture_sampler : sampler;

// Color

const Epsilon : f32 = 1e-10;
fn rgb_to_hsv (rgb : vec3<f32>) -> vec3<f32> {
	var  P = vec4(0.);
	if (rgb.g < rgb.b) { P = vec4(rgb.bg, -1.0, 2.0/3.0); } else { P = vec4(rgb.gb, 0.0, -1.0/3.0); }
    var  Q   = vec4(0.);
    if (rgb.r < P.x) { Q = vec4(P.xyw, rgb.r); } else { Q = vec4(rgb.r, P.yzx); }
    let C   = Q.x - min(Q.w, Q.y);
    let H   = abs((Q.w - Q.y) / (6.0 * C + Epsilon) + Q.z);
    let  HCV = vec3(H, C, Q.x);
    let S   = HCV.y / (HCV.z + Epsilon);
    return vec3(HCV.x, S, HCV.z);
}

fn hsv_to_rgb (hsv : vec3<f32>) -> vec3<f32> {
	let H   = hsv.x;
    let R   = clamp(abs(H * 6.0 - 3.0) - 1.0, 0., 1.);
    let G   = clamp(2.0 - abs(H * 6.0 - 2.0), 0., 1.);
    let B   = clamp(2.0 - abs(H * 6.0 - 4.0), 0., 1.);
    let  RGB = vec3(R,G,B);
    return ((RGB - 1.0) * hsv.y + 1.0) * hsv.z;
}

fn tweak_saturation (in : vec4<f32>, amount : f32) -> vec4<f32> {
	var hsv = rgb_to_hsv(in.rgb);
	hsv.z *= amount;
	return vec4<f32>(hsv_to_rgb(hsv.rgb), in.a);
}

// Frag

@fragment
fn fragment (
	in : FullscreenVertexOutput,
	@builtin(sample_index) sample_index : u32,
) -> @location(0) vec4<f32> {
	let base_color = textureSample(screen_texture, texture_sampler, in.uv);

	let x = in.uv.x - 0.5;
	let y = in.uv.y - 0.5;
	let x2 = x * x;
	let y2 = y * y;
	let c = x2 + y2;

	if (c > 0.25) {
		return vec4<f32>(0.);
	}

	// Ring Thin
	if ((c - 0.245) * (c - 0.24) <= 0.) {
		return tweak_saturation(vec4<f32>(0.192, 0.196, 0.212, 1.), 0.1);
	}

	// Ring Thick
	if ((c - 0.235) * (c - 0.226) <= 0.) {
		return tweak_saturation(vec4<f32>(0.384, 0.38, 0.408, 1.), 0.1);
	}

	// Ring BG
	let bg = tweak_saturation(vec4<f32>(0.09, 0.098, 0.11, 1.), 0.1);
	if (c > 0.22) {
		return bg;
	}

	if (base_color.a < 1.) {
		// TODO: offset starfield by camera pos
		return max(max(bg, starfield(in.uv, vec2(0.))), base_color);
	}

	return base_color;
}

