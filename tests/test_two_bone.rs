use approx::*;
use ik_math::two_bone::*;
use vect::prelude::*;

#[cfg(test)]
mod test_calculate_point_between {
	use super::*;

	const E: f64 = 0.000001;

	#[test]
	fn test_calculate_point_between() {
		let result = calculate_point_between_stretched(
			Vector2::new(20.0, 0.0),
			Vector2::new(0.0, 0.0),
			16.6,
			8.6,
			1.0,
			0.0,
			1.0,
		);

		assert_relative_eq!(result.0.x, 15.04, epsilon = E);
		assert_relative_eq!(result.0.y, -7.02555336, epsilon = E);
	}

	#[test]
	fn test_with_different_stretch_factor() {
		let result = calculate_point_between_stretched(
			Vector2::new(20.0, 0.0),
			Vector2::new(0.0, 0.0),
			16.6,
			8.6,
			0.16,
			0.0,
			1.0,
		);

		assert_relative_eq!(result.0.x, 15.04, epsilon = E);
		assert_relative_eq!(result.0.y, -1.12408854, epsilon = E);
	}

	#[test]
	fn test_with_negative_stretch_factor() {
		let result = calculate_point_between_stretched(
			Vector2::new(20.0, 0.0),
			Vector2::new(0.0, 0.0),
			16.6,
			8.6,
			-0.62,
			0.0,
			1.0,
		);

		assert_relative_eq!(result.0.x, 15.04, epsilon = E);
		assert_relative_eq!(result.0.y, 4.35584308, epsilon = E);
	}

	#[test]
	fn test_with_zero_stretch_factor() {
		let result = calculate_point_between_stretched(
			Vector2::new(20.0, 0.0),
			Vector2::new(0.0, 0.0),
			16.6,
			8.6,
			0.0,
			0.0,
			1.0,
		);

		assert_relative_eq!(result.0.x, 15.04, epsilon = E);
		assert_relative_eq!(result.0.y, 0.0, epsilon = E);
	}

	#[test]
	fn test_with_big_stretch_factor() {
		let result = calculate_point_between_stretched(
			Vector2::new(20.0, 0.0),
			Vector2::new(0.0, 0.0),
			16.6,
			8.6,
			3.23,
			0.0,
			1.0,
		);

		assert_relative_eq!(result.0.x, 15.04, epsilon = E);
		assert_relative_eq!(result.0.y, -22.6925373, epsilon = E);
	}

	#[test]
	fn test_when_target_is_too_close() {
		let result = calculate_point_between_stretched(
			Vector2::new(7.0, 0.0),
			Vector2::new(0.0, 0.0),
			16.6,
			8.6,
			1.0,
			0.0,
			1.0,
		);

		assert_relative_eq!(result.0.x, 16.6, epsilon = E);
		assert_relative_eq!(result.0.y, 0.0, epsilon = E);
	}

	#[test]
	fn test_when_target_is_too_far() {
		let result = calculate_point_between_stretched(
			Vector2::new(26.0, 0.0),
			Vector2::new(0.0, 0.0),
			16.6,
			8.6,
			1.0,
			0.0,
			1.0,
		);

		assert_relative_eq!(result.0.x, 16.6, epsilon = E);
		assert_relative_eq!(result.0.y, 0.0, epsilon = E);
	}

	#[test]
	fn test_with_custom_clamp_range() {
		let result = calculate_point_between_stretched(
			Vector2::new(20.0, 0.0),
			Vector2::new(0.0, 0.0),
			16.6,
			8.6,
			1.0,
			0.3,
			0.9,
		);

		assert_relative_eq!(result.0.x, 15.04, epsilon = E);
		assert_relative_eq!(result.0.y, -7.02555336, epsilon = E);
	}

	#[test]
	fn test_when_target_is_too_close_with_custom_clamp_range() {
		let result = calculate_point_between_stretched(
			Vector2::new(9.0, 0.0),
			Vector2::new(0.0, 0.0),
			16.6,
			8.6,
			1.0,
			0.4,
			0.7,
		);

		assert_relative_eq!(result.0.x, 14.2141935483871, epsilon = E);
		assert_relative_eq!(result.0.y, -8.574188111360202, epsilon = E);
	}

	#[test]
	fn test_when_target_is_too_far_with_custom_clamp_range() {
		let result = calculate_point_between_stretched(
			Vector2::new(26.0, 0.0),
			Vector2::new(0.0, 0.0),
			16.6,
			8.6,
			1.0,
			0.4,
			0.7,
		);

		assert_relative_eq!(result.0.x, 15.049940119760482, epsilon = E);
		assert_relative_eq!(result.0.y, -7.0042346042679, epsilon = E);
	}
}
