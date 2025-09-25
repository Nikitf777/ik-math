use vect::prelude::*;

use crate::vector2::*;

pub fn calculate_point_between_stretched(
	target: Vector2,
	first_point: Vector2,
	first_len: f64,
	second_len: f64,
	stretch_factor: f64,
) -> Vector2 {
	let vect = target - first_point;
	let dist = vect.length();
	let min_len = (first_len - second_len).abs();
	let max_len = first_len + second_len;

	let vect_normalized = vect / dist;
	let dist_clamped = dist.clamp(min_len, max_len);

	let first_len_squared = first_len.powi(2);
	let ah_cathetus_length =
		(first_len_squared - second_len.powi(2) + dist_clamped.powi(2)) / (2.0 * dist_clamped);
	let height = (first_len_squared - ah_cathetus_length.powi(2)).sqrt();
	let h_point = first_point + vect_normalized * ah_cathetus_length;
	h_point
		+ Vector2::new(
			height * vect_normalized.y * stretch_factor,
			-height * vect_normalized.x * stretch_factor,
		)
}
