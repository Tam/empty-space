pub fn f_mod (x : f32, y : f32) -> f32 {
	return x - y * f32::floor(x / y);
}
