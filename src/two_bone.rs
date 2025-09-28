use vect::prelude::*;

use crate::vector2::*;

pub fn calculate_point_between_stretched(
	target: Vector2,
	first_point: Vector2,
	first_len: f64,
	second_len: f64,
	stretch_factor: f64,
	min_target_distance_ratio: f64,
	max_target_distance_ratio: f64,
) -> (Vector2, Vector2) {
	let vect = target - first_point;
	let dist = vect.length();
	let min_length = (first_len - second_len).abs();
	let max_length = first_len + second_len;
	let len_diff = max_length - min_length;

	let min_length_clamped = min_length + len_diff * min_target_distance_ratio.clamp(0.0, 1.0);
	let max_length_clamped =
		max_length - len_diff * (1.0 - max_target_distance_ratio.clamp(0.0, 1.0));

	let vect_normalized = vect / dist;
	let (vect_clamped, dist_clamped) = if dist < min_length_clamped {
		(vect_normalized * min_length_clamped, min_length_clamped)
	} else if dist > max_length_clamped {
		(vect_normalized * max_length_clamped, max_length_clamped)
	} else {
		(vect, dist)
	};

	let first_len_squared = first_len.powi(2);
	let ah_cathetus_length =
		(first_len_squared - second_len.powi(2) + dist_clamped.powi(2)) / (2.0 * dist_clamped);
	let height = (first_len_squared - ah_cathetus_length.powi(2))
		.abs()
		.sqrt();
	let h_point = first_point + vect_normalized * ah_cathetus_length;
	(
		h_point
			+ Vector2::new(
				height * vect_normalized.y * stretch_factor,
				-height * vect_normalized.x * stretch_factor,
			),
		vect_clamped,
	)
}
