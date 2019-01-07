//! This the uncomprensed "compresion"
//! We only need move the folder specified to peform a backup

use crate::compressors::{Comprensable, CompressResult};
//use std::io::File;
use std::path::Path;

/// Struct for handle uncomprensed data
#[derive(Copy, Clone, Default)]
pub struct Uncompressed;

impl Comprensable for Uncompressed {
	fn compress(&self, org: &Path, dest: &Path) -> CompressResult<()> {
		println!("Coping {} to {}", org.display(), dest.display());

		Ok(())
	}
}
