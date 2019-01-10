//! This the gzip compresion
//! We only need move the folder specified to peform a backup

use crate::backup::backup::Backup;
use crate::compressors::{Comprensable, CompressResult};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use zip;

/// Struct for handle uncomprensed data
#[derive(Default)]
pub struct Zip {
	/// Options for write the zip file
	pub options: Option<zip::write::FileOptions>,
	destination: Option<zip::ZipWriter<File>>,
}

impl Comprensable for Zip {
	fn init(&mut self, bkp: &Backup) {
		self.destination = Some(zip::ZipWriter::new(
			File::create(&bkp.destination).expect("Cannot create file"),
		));
	}

	fn compress(&mut self, org: &Path, _dest: &Path) -> CompressResult {
		if org.is_dir() {
			return Ok(());
		}

		let mut dst = self.destination.take().unwrap();
		dst.start_file(format!("{}", org.display()), self.options.unwrap())
			.unwrap();

		let mut buffer = Vec::new();
		let mut f = File::open(org).expect("Error opening file");

		f.read_to_end(&mut buffer).expect("Error reading file");
		dst.write_all(&*buffer).expect("Error writing file");
		buffer.clear();

		self.destination = Some(dst);
		Ok(())
	}

	fn finish(&mut self) {
		let mut dst = self.destination.take().unwrap();
		dst.finish().unwrap();
	}
}
