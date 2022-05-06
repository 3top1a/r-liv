#[cfg(test)]
mod tests {
	use std::path::Path;
	#[test]
	fn test_get_file_stem() {
		assert_eq!(
			crate::utils::get_file_stem("/home/guest/index.html"),
			"index"
		);
		assert_eq!(
			crate::utils::get_file_stem(Path::new("/home/guest/index.html")),
			"index"
		);
	}
}
