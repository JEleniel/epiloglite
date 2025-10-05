/// Write-Ahead Log (WAL) implementation for concurrent access and durability

use crate::eplite::error::{Error, Result};

#[cfg(not(feature = "std"))]
use alloc::{format, string::ToString, vec, vec::Vec};

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap as HashMap;

/// WAL magic number for big-endian checksum
const WAL_MAGIC_BE: u32 = 0x377f0683;
/// WAL magic number for little-endian checksum
const WAL_MAGIC_LE: u32 = 0x377f0682;
/// WAL file format version
const WAL_VERSION: u32 = 3007000;

/// WAL header size in bytes
const WAL_HEADER_SIZE: usize = 32;
/// WAL frame header size in bytes
const WAL_FRAME_HEADER_SIZE: usize = 24;

/// WAL file header (32 bytes)
#[derive(Debug, Clone)]
pub struct WalHeader {
	/// Magic number (0x377f0682 or 0x377f0683)
	pub magic: u32,
	/// File format version (currently 3007000)
	pub version: u32,
	/// Database page size
	pub page_size: u32,
	/// Checkpoint sequence number
	pub checkpoint_seq: u32,
	/// Salt-1: random integer incremented with each checkpoint
	pub salt1: u32,
	/// Salt-2: different random number for each checkpoint
	pub salt2: u32,
	/// Checksum-1: first part of checksum on first 24 bytes
	pub checksum1: u32,
	/// Checksum-2: second part of checksum on first 24 bytes
	pub checksum2: u32,
}

impl WalHeader {
	/// Create a new WAL header with default values
	pub fn new(page_size: u32) -> Self {
		// Use big-endian magic number
		let magic = WAL_MAGIC_BE;
		
		WalHeader {
			magic,
			version: WAL_VERSION,
			page_size,
			checkpoint_seq: 0,
			salt1: Self::generate_salt(),
			salt2: Self::generate_salt(),
			checksum1: 0,
			checksum2: 0,
		}
	}

	/// Generate a random salt value
	fn generate_salt() -> u32 {
		#[cfg(feature = "std")]
		{
			use crate::eplite::os;
			let bytes = os::random_bytes(4);
			if bytes.len() >= 4 {
				u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
			} else {
				// Fallback to timestamp-based
				use std::time::{SystemTime, UNIX_EPOCH};
				SystemTime::now()
					.duration_since(UNIX_EPOCH)
					.unwrap_or_default()
					.as_secs() as u32
			}
		}
		#[cfg(not(feature = "std"))]
		{
			// Simple pseudo-random for no-std
			0x12345678
		}
	}

	/// Serialize header to bytes
	pub fn to_bytes(&self) -> [u8; WAL_HEADER_SIZE] {
		let mut bytes = [0u8; WAL_HEADER_SIZE];
		
		bytes[0..4].copy_from_slice(&self.magic.to_be_bytes());
		bytes[4..8].copy_from_slice(&self.version.to_be_bytes());
		bytes[8..12].copy_from_slice(&self.page_size.to_be_bytes());
		bytes[12..16].copy_from_slice(&self.checkpoint_seq.to_be_bytes());
		bytes[16..20].copy_from_slice(&self.salt1.to_be_bytes());
		bytes[20..24].copy_from_slice(&self.salt2.to_be_bytes());
		bytes[24..28].copy_from_slice(&self.checksum1.to_be_bytes());
		bytes[28..32].copy_from_slice(&self.checksum2.to_be_bytes());
		
		bytes
	}

	/// Deserialize header from bytes
	pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
		if bytes.len() < WAL_HEADER_SIZE {
			return Err(Error::InvalidFormat(format!(
				"WAL header too small: {} bytes, expected {}",
				bytes.len(),
				WAL_HEADER_SIZE
			)));
		}

		let magic = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
		
		// Validate magic number
		if magic != WAL_MAGIC_BE && magic != WAL_MAGIC_LE {
			return Err(Error::InvalidFormat(format!(
				"Invalid WAL magic number: 0x{:08x}",
				magic
			)));
		}

		Ok(WalHeader {
			magic,
			version: u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
			page_size: u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
			checkpoint_seq: u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
			salt1: u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]),
			salt2: u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
			checksum1: u32::from_be_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
			checksum2: u32::from_be_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]),
		})
	}

	/// Update checksums for the header
	pub fn update_checksums(&mut self) {
		let bytes = self.to_bytes();
		let (s0, s1) = compute_checksum(&bytes[0..24], 0, 0, self.magic == WAL_MAGIC_BE);
		self.checksum1 = s0;
		self.checksum2 = s1;
	}
}

/// WAL frame header (24 bytes)
#[derive(Debug, Clone)]
pub struct WalFrameHeader {
	/// Page number
	pub page_number: u32,
	/// Database size after commit (0 for non-commit frames)
	pub db_size: u32,
	/// Salt-1 from WAL header
	pub salt1: u32,
	/// Salt-2 from WAL header
	pub salt2: u32,
	/// Checksum-1: cumulative checksum including this frame
	pub checksum1: u32,
	/// Checksum-2: second half of cumulative checksum
	pub checksum2: u32,
}

impl WalFrameHeader {
	/// Create a new frame header
	pub fn new(page_number: u32, db_size: u32, salt1: u32, salt2: u32) -> Self {
		WalFrameHeader {
			page_number,
			db_size,
			salt1,
			salt2,
			checksum1: 0,
			checksum2: 0,
		}
	}

	/// Serialize frame header to bytes
	pub fn to_bytes(&self) -> [u8; WAL_FRAME_HEADER_SIZE] {
		let mut bytes = [0u8; WAL_FRAME_HEADER_SIZE];
		
		bytes[0..4].copy_from_slice(&self.page_number.to_be_bytes());
		bytes[4..8].copy_from_slice(&self.db_size.to_be_bytes());
		bytes[8..12].copy_from_slice(&self.salt1.to_be_bytes());
		bytes[12..16].copy_from_slice(&self.salt2.to_be_bytes());
		bytes[16..20].copy_from_slice(&self.checksum1.to_be_bytes());
		bytes[20..24].copy_from_slice(&self.checksum2.to_be_bytes());
		
		bytes
	}

	/// Deserialize frame header from bytes
	pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
		if bytes.len() < WAL_FRAME_HEADER_SIZE {
			return Err(Error::InvalidFormat(format!(
				"WAL frame header too small: {} bytes",
				bytes.len()
			)));
		}

		Ok(WalFrameHeader {
			page_number: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
			db_size: u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
			salt1: u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
			salt2: u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
			checksum1: u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]),
			checksum2: u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
		})
	}

	/// Check if this frame is a commit frame
	pub fn is_commit(&self) -> bool {
		self.db_size > 0
	}
}

/// Complete WAL frame (header + page data)
#[derive(Debug, Clone)]
pub struct WalFrame {
	pub header: WalFrameHeader,
	pub data: Vec<u8>,
}

impl WalFrame {
	/// Create a new WAL frame
	pub fn new(page_number: u32, data: Vec<u8>, salt1: u32, salt2: u32) -> Self {
		WalFrame {
			header: WalFrameHeader::new(page_number, 0, salt1, salt2),
			data,
		}
	}

	/// Mark this frame as a commit frame
	pub fn mark_commit(&mut self, db_size: u32) {
		self.header.db_size = db_size;
	}
}

/// Compute WAL checksum using Fibonacci-weighted algorithm
/// 
/// The checksum is computed by interpreting input as an even number of u32 integers.
/// Returns (s0, s1) checksum values.
fn compute_checksum(data: &[u8], s0: u32, s1: u32, big_endian: bool) -> (u32, u32) {
	let mut sum0 = s0;
	let mut sum1 = s1;
	
	// Process data in 8-byte (two u32) chunks
	let mut i = 0;
	while i + 7 < data.len() {
		let x0 = if big_endian {
			u32::from_be_bytes([data[i], data[i+1], data[i+2], data[i+3]])
		} else {
			u32::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3]])
		};
		
		let x1 = if big_endian {
			u32::from_be_bytes([data[i+4], data[i+5], data[i+6], data[i+7]])
		} else {
			u32::from_le_bytes([data[i+4], data[i+5], data[i+6], data[i+7]])
		};
		
		sum0 = sum0.wrapping_add(x0).wrapping_add(sum1);
		sum1 = sum1.wrapping_add(x1).wrapping_add(sum0);
		
		i += 8;
	}
	
	(sum0, sum1)
}

/// WAL writer for appending frames
#[derive(Debug)]
pub struct WalWriter {
	header: WalHeader,
	frames: Vec<WalFrame>,
	current_checksum: (u32, u32),
}

impl WalWriter {
	/// Create a new WAL writer
	pub fn new(page_size: u32) -> Self {
		let mut header = WalHeader::new(page_size);
		header.update_checksums();
		
		// Initialize checksum with header
		let header_bytes = header.to_bytes();
		let initial_checksum = compute_checksum(
			&header_bytes[0..24],
			0,
			0,
			header.magic == WAL_MAGIC_BE
		);
		
		WalWriter {
			header,
			frames: Vec::new(),
			current_checksum: initial_checksum,
		}
	}

	/// Add a frame to the WAL
	pub fn add_frame(&mut self, mut frame: WalFrame) -> Result<()> {
		// Validate page data size
		if frame.data.len() != self.header.page_size as usize {
			return Err(Error::InvalidFormat(format!(
				"Frame data size {} doesn't match page size {}",
				frame.data.len(),
				self.header.page_size
			)));
		}

		// Update salts in frame header
		frame.header.salt1 = self.header.salt1;
		frame.header.salt2 = self.header.salt2;

		// Compute cumulative checksum for frame header and data
		let frame_header_bytes = frame.header.to_bytes();
		let (s0, s1) = compute_checksum(
			&frame_header_bytes[0..8],
			self.current_checksum.0,
			self.current_checksum.1,
			self.header.magic == WAL_MAGIC_BE
		);
		
		let (s0, s1) = compute_checksum(
			&frame.data,
			s0,
			s1,
			self.header.magic == WAL_MAGIC_BE
		);

		// Update frame checksums
		frame.header.checksum1 = s0;
		frame.header.checksum2 = s1;

		// Update current checksum state
		self.current_checksum = (s0, s1);

		self.frames.push(frame);
		Ok(())
	}

	/// Mark the last frame as a commit
	pub fn commit(&mut self, db_size: u32) -> Result<()> {
		if self.frames.is_empty() {
			return Err(Error::Internal("No frames to commit".to_string()));
		}

		// We need to recompute the checksum for the last frame with updated db_size
		// Remove the last frame temporarily
		let mut last_frame = self.frames.pop().unwrap();
		
		// Recalculate checksum from the beginning
		let header_bytes = self.header.to_bytes();
		let mut current_checksum = compute_checksum(
			&header_bytes[0..24],
			0,
			0,
			self.header.magic == WAL_MAGIC_BE
		);

		// Recompute checksums for all frames up to the second-to-last
		for frame in &self.frames {
			let frame_header_bytes = frame.header.to_bytes();
			let (s0, s1) = compute_checksum(
				&frame_header_bytes[0..8],
				current_checksum.0,
				current_checksum.1,
				self.header.magic == WAL_MAGIC_BE
			);
			
			current_checksum = compute_checksum(
				&frame.data,
				s0,
				s1,
				self.header.magic == WAL_MAGIC_BE
			);
		}

		// Now update the last frame with commit marker
		last_frame.mark_commit(db_size);

		// Compute checksum for the updated last frame
		let frame_header_bytes = last_frame.header.to_bytes();
		let (s0, s1) = compute_checksum(
			&frame_header_bytes[0..8],
			current_checksum.0,
			current_checksum.1,
			self.header.magic == WAL_MAGIC_BE
		);
		
		let (s0, s1) = compute_checksum(
			&last_frame.data,
			s0,
			s1,
			self.header.magic == WAL_MAGIC_BE
		);

		last_frame.header.checksum1 = s0;
		last_frame.header.checksum2 = s1;
		
		self.frames.push(last_frame);
		self.current_checksum = (s0, s1);
		
		Ok(())
	}

	/// Get the header
	pub fn header(&self) -> &WalHeader {
		&self.header
	}

	/// Get all frames
	pub fn frames(&self) -> &[WalFrame] {
		&self.frames
	}

	/// Serialize the entire WAL to bytes
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
		
		// Write header
		bytes.extend_from_slice(&self.header.to_bytes());
		
		// Write frames
		for frame in &self.frames {
			bytes.extend_from_slice(&frame.header.to_bytes());
			bytes.extend_from_slice(&frame.data);
		}
		
		bytes
	}

	/// Reset the WAL for a new transaction
	pub fn reset(&mut self) {
		self.frames.clear();
		self.header.checkpoint_seq += 1;
		self.header.salt1 = WalHeader::generate_salt();
		self.header.salt2 = WalHeader::generate_salt();
		self.header.update_checksums();
		
		// Reinitialize checksum
		let header_bytes = self.header.to_bytes();
		self.current_checksum = compute_checksum(
			&header_bytes[0..24],
			0,
			0,
			self.header.magic == WAL_MAGIC_BE
		);
	}
}

/// WAL reader for concurrent access
pub struct WalReader {
	header: WalHeader,
	/// Map from page number to latest frame index
	page_map: HashMap<u32, usize>,
	frames: Vec<WalFrame>,
	/// Maximum frame index for this reader
	max_frame: usize,
}

/// Checkpoint mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointMode {
	/// Passive checkpoint - checkpoint what we can without blocking
	Passive,
	/// Full checkpoint - complete checkpoint even if readers exist
	Full,
	/// Restart checkpoint - checkpoint and restart the WAL
	Restart,
	/// Truncate checkpoint - checkpoint and truncate the WAL file
	Truncate,
}

/// Checkpoint result information
#[derive(Debug, Clone)]
pub struct CheckpointResult {
	/// Number of frames in WAL
	pub wal_frames: usize,
	/// Number of frames checkpointed
	pub checkpointed_frames: usize,
	/// Whether checkpoint completed fully
	pub completed: bool,
}

/// WAL checkpoint manager
pub struct WalCheckpoint {
	page_size: u32,
}

impl WalCheckpoint {
	/// Create a new checkpoint manager
	pub fn new(page_size: u32) -> Self {
		WalCheckpoint { page_size }
	}

	/// Perform a checkpoint operation
	/// Transfers valid frames from WAL to the database
	pub fn checkpoint(
		&self,
		reader: &WalReader,
		mode: CheckpointMode,
	) -> Result<(Vec<(u32, Vec<u8>)>, CheckpointResult)> {
		let frames = reader.frames();
		
		// Build a map of page_number -> latest frame data
		let mut page_updates: HashMap<u32, Vec<u8>> = HashMap::new();
		
		// Find the last commit
		let mut last_commit_idx: Option<usize> = None;
		for (idx, frame) in frames.iter().enumerate() {
			if frame.header.is_commit() {
				last_commit_idx = Some(idx);
			}
		}

		let checkpointed_count = if let Some(last_commit) = last_commit_idx {
			// Collect all pages up to and including the last commit
			for (idx, frame) in frames.iter().enumerate() {
				if idx > last_commit {
					break;
				}
				page_updates.insert(frame.header.page_number, frame.data.clone());
			}
			last_commit + 1
		} else {
			0
		};

		// Convert to sorted vector for deterministic ordering
		let mut updates: Vec<(u32, Vec<u8>)> = page_updates.into_iter().collect();
		updates.sort_by_key(|(page_num, _)| *page_num);

		let result = CheckpointResult {
			wal_frames: frames.len(),
			checkpointed_frames: checkpointed_count,
			completed: match mode {
				CheckpointMode::Passive => checkpointed_count > 0,
				CheckpointMode::Full | CheckpointMode::Restart | CheckpointMode::Truncate => {
					checkpointed_count == frames.len()
				}
			},
		};

		Ok((updates, result))
	}
}

/// WAL recovery manager
pub struct WalRecovery {
	page_size: u32,
}

impl WalRecovery {
	/// Create a new recovery manager
	pub fn new(page_size: u32) -> Self {
		WalRecovery { page_size }
	}

	/// Recover database state from WAL file
	/// Returns list of (page_number, page_data) to apply to database
	pub fn recover(&self, wal_bytes: &[u8]) -> Result<Vec<(u32, Vec<u8>)>> {
		// Read the WAL
		let reader = WalReader::from_bytes(wal_bytes)?;
		
		// Use checkpoint logic to extract committed pages
		let checkpoint = WalCheckpoint::new(self.page_size);
		let (updates, _result) = checkpoint.checkpoint(&reader, CheckpointMode::Full)?;
		
		Ok(updates)
	}

	/// Check if WAL needs recovery
	pub fn needs_recovery(wal_bytes: &[u8]) -> bool {
		if wal_bytes.len() < WAL_HEADER_SIZE {
			return false;
		}
		
		// Try to read the header
		if let Ok(header) = WalHeader::from_bytes(wal_bytes) {
			// Check if there's any frame data beyond the header
			let min_frame_size = WAL_FRAME_HEADER_SIZE + header.page_size as usize;
			wal_bytes.len() >= WAL_HEADER_SIZE + min_frame_size
		} else {
			false
		}
	}
}

impl WalReader {
	/// Create a WAL reader from bytes
	pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
		if bytes.len() < WAL_HEADER_SIZE {
			return Err(Error::InvalidFormat("WAL file too small".to_string()));
		}

		let header = WalHeader::from_bytes(&bytes[0..WAL_HEADER_SIZE])?;
		
		// Verify header checksum
		let header_bytes = header.to_bytes();
		let (s0, s1) = compute_checksum(
			&header_bytes[0..24],
			0,
			0,
			header.magic == WAL_MAGIC_BE
		);
		
		if s0 != header.checksum1 || s1 != header.checksum2 {
			return Err(Error::InvalidFormat("WAL header checksum mismatch".to_string()));
		}

		let mut frames = Vec::new();
		let mut page_map = HashMap::new();
		let mut current_checksum = (s0, s1);
		let mut offset = WAL_HEADER_SIZE;

		// Read all valid frames
		while offset + WAL_FRAME_HEADER_SIZE + header.page_size as usize <= bytes.len() {
			let frame_header = WalFrameHeader::from_bytes(&bytes[offset..offset+WAL_FRAME_HEADER_SIZE])?;
			
			// Verify salts match
			if frame_header.salt1 != header.salt1 || frame_header.salt2 != header.salt2 {
				// Invalid frame, stop reading
				break;
			}

			let data_start = offset + WAL_FRAME_HEADER_SIZE;
			let data_end = data_start + header.page_size as usize;
			let data = bytes[data_start..data_end].to_vec();

			// Verify frame checksum
			let frame_header_bytes = frame_header.to_bytes();
			let (s0, s1) = compute_checksum(
				&frame_header_bytes[0..8],
				current_checksum.0,
				current_checksum.1,
				header.magic == WAL_MAGIC_BE
			);
			
			let (s0, s1) = compute_checksum(
				&data,
				s0,
				s1,
				header.magic == WAL_MAGIC_BE
			);

			if s0 != frame_header.checksum1 || s1 != frame_header.checksum2 {
				// Invalid checksum, stop reading
				break;
			}

			current_checksum = (s0, s1);

			let frame = WalFrame {
				header: frame_header,
				data,
			};

			// Update page map
			page_map.insert(frame.header.page_number, frames.len());
			frames.push(frame);

			offset = data_end;
		}

		let max_frame = frames.len();

		Ok(WalReader {
			header,
			page_map,
			frames,
			max_frame,
		})
	}

	/// Get the page data for a given page number
	/// Returns None if page not in WAL
	pub fn get_page(&self, page_number: u32) -> Option<&[u8]> {
		// Find the last commit frame index
		let mut last_commit_idx: Option<usize> = None;
		for (idx, frame) in self.frames.iter().enumerate() {
			if idx >= self.max_frame {
				break;
			}
			if frame.header.is_commit() {
				last_commit_idx = Some(idx);
			}
		}

		// If no commits, no valid pages
		let last_commit = last_commit_idx?;

		// Find the latest frame for this page up to and including the last commit
		let mut latest_frame_idx: Option<usize> = None;
		for (idx, frame) in self.frames.iter().enumerate() {
			if idx > last_commit {
				break;
			}
			if frame.header.page_number == page_number {
				latest_frame_idx = Some(idx);
			}
		}

		latest_frame_idx.map(|idx| self.frames[idx].data.as_slice())
	}

	/// Get the WAL header
	pub fn header(&self) -> &WalHeader {
		&self.header
	}

	/// Get the number of valid frames
	pub fn frame_count(&self) -> usize {
		self.max_frame
	}

	/// Get all frames for checkpoint
	pub fn frames(&self) -> &[WalFrame] {
		&self.frames[0..self.max_frame]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_wal_header_creation() {
		let header = WalHeader::new(4096);
		assert_eq!(header.page_size, 4096);
		assert_eq!(header.version, WAL_VERSION);
		assert!(header.magic == WAL_MAGIC_BE || header.magic == WAL_MAGIC_LE);
	}

	#[test]
	fn test_wal_header_serialization() {
		let mut header = WalHeader::new(4096);
		header.update_checksums();
		
		let bytes = header.to_bytes();
		assert_eq!(bytes.len(), WAL_HEADER_SIZE);
		
		let decoded = WalHeader::from_bytes(&bytes).unwrap();
		assert_eq!(decoded.page_size, header.page_size);
		assert_eq!(decoded.version, header.version);
		assert_eq!(decoded.checksum1, header.checksum1);
		assert_eq!(decoded.checksum2, header.checksum2);
	}

	#[test]
	fn test_wal_frame_header() {
		let frame_header = WalFrameHeader::new(1, 0, 0x12345678, 0x87654321);
		assert_eq!(frame_header.page_number, 1);
		assert_eq!(frame_header.db_size, 0);
		assert!(!frame_header.is_commit());
		
		let bytes = frame_header.to_bytes();
		let decoded = WalFrameHeader::from_bytes(&bytes).unwrap();
		assert_eq!(decoded.page_number, frame_header.page_number);
	}

	#[test]
	fn test_wal_frame_commit() {
		let mut frame = WalFrame::new(1, vec![0u8; 4096], 0x12345678, 0x87654321);
		assert!(!frame.header.is_commit());
		
		frame.mark_commit(10);
		assert!(frame.header.is_commit());
		assert_eq!(frame.header.db_size, 10);
	}

	#[test]
	fn test_checksum_algorithm() {
		let data = vec![0u8; 16];
		let (s0, s1) = compute_checksum(&data, 0, 0, true);
		assert_eq!(s0, 0);
		assert_eq!(s1, 0);
		
		let data = vec![1u8, 0, 0, 0, 2, 0, 0, 0];
		let (s0, s1) = compute_checksum(&data, 0, 0, true);
		assert!(s0 > 0 || s1 > 0);
	}

	#[test]
	fn test_wal_writer_basic() {
		let mut writer = WalWriter::new(4096);
		assert_eq!(writer.frames().len(), 0);
		
		let frame = WalFrame::new(1, vec![0u8; 4096], 0, 0);
		writer.add_frame(frame).unwrap();
		assert_eq!(writer.frames().len(), 1);
	}

	#[test]
	fn test_wal_writer_commit() {
		let mut writer = WalWriter::new(4096);
		
		let frame1 = WalFrame::new(1, vec![0u8; 4096], 0, 0);
		writer.add_frame(frame1).unwrap();
		
		let frame2 = WalFrame::new(2, vec![1u8; 4096], 0, 0);
		writer.add_frame(frame2).unwrap();
		
		writer.commit(2).unwrap();
		
		let last_frame = writer.frames().last().unwrap();
		assert!(last_frame.header.is_commit());
		assert_eq!(last_frame.header.db_size, 2);
	}

	#[test]
	fn test_wal_writer_invalid_page_size() {
		let mut writer = WalWriter::new(4096);
		let frame = WalFrame::new(1, vec![0u8; 2048], 0, 0);
		assert!(writer.add_frame(frame).is_err());
	}

	#[test]
	fn test_wal_reader_basic() {
		let mut writer = WalWriter::new(4096);
		
		let frame = WalFrame::new(1, vec![42u8; 4096], 0, 0);
		writer.add_frame(frame).unwrap();
		writer.commit(1).unwrap();
		
		let bytes = writer.to_bytes();
		let reader = WalReader::from_bytes(&bytes).unwrap();
		
		assert_eq!(reader.frame_count(), 1);
		assert_eq!(reader.header().page_size, 4096);
	}

	#[test]
	fn test_wal_reader_get_page() {
		let mut writer = WalWriter::new(4096);
		
		let mut data = vec![0u8; 4096];
		data[0] = 42;
		let frame = WalFrame::new(1, data, 0, 0);
		writer.add_frame(frame).unwrap();
		writer.commit(1).unwrap();
		
		let bytes = writer.to_bytes();
		let reader = WalReader::from_bytes(&bytes).unwrap();
		
		let page = reader.get_page(1).unwrap();
		assert_eq!(page[0], 42);
		assert_eq!(page.len(), 4096);
	}

	#[test]
	fn test_wal_reader_missing_page() {
		let mut writer = WalWriter::new(4096);
		
		let frame = WalFrame::new(1, vec![0u8; 4096], 0, 0);
		writer.add_frame(frame).unwrap();
		writer.commit(1).unwrap();
		
		let bytes = writer.to_bytes();
		let reader = WalReader::from_bytes(&bytes).unwrap();
		
		assert!(reader.get_page(2).is_none());
	}

	#[test]
	fn test_wal_writer_reset() {
		let mut writer = WalWriter::new(4096);
		
		let frame = WalFrame::new(1, vec![0u8; 4096], 0, 0);
		writer.add_frame(frame).unwrap();
		
		let old_seq = writer.header().checkpoint_seq;
		writer.reset();
		
		assert_eq!(writer.frames().len(), 0);
		assert_eq!(writer.header().checkpoint_seq, old_seq + 1);
	}

	#[test]
	fn test_wal_multiple_pages() {
		let mut writer = WalWriter::new(4096);
		
		// Write multiple pages
		for i in 1..=5 {
			let mut data = vec![0u8; 4096];
			data[0] = i as u8;
			let frame = WalFrame::new(i, data, 0, 0);
			writer.add_frame(frame).unwrap();
		}
		
		writer.commit(5).unwrap();
		
		let bytes = writer.to_bytes();
		let reader = WalReader::from_bytes(&bytes).unwrap();
		
		// Verify all pages
		for i in 1..=5 {
			let page = reader.get_page(i).unwrap();
			assert_eq!(page[0], i as u8);
		}
	}

	#[test]
	fn test_wal_page_updates() {
		let mut writer = WalWriter::new(4096);
		
		// Write page 1 with value 42
		let frame1 = WalFrame::new(1, vec![42u8; 4096], 0, 0);
		writer.add_frame(frame1).unwrap();
		
		// Update page 1 with value 99
		let frame2 = WalFrame::new(1, vec![99u8; 4096], 0, 0);
		writer.add_frame(frame2).unwrap();
		
		writer.commit(1).unwrap();
		
		let bytes = writer.to_bytes();
		let reader = WalReader::from_bytes(&bytes).unwrap();
		
		// Should get the latest version
		let page = reader.get_page(1).unwrap();
		assert_eq!(page[0], 99);
	}

	#[test]
	fn test_checkpoint_basic() {
		let mut writer = WalWriter::new(4096);
		
		// Write some pages
		for i in 1..=3 {
			let mut data = vec![0u8; 4096];
			data[0] = i as u8;
			let frame = WalFrame::new(i, data, 0, 0);
			writer.add_frame(frame).unwrap();
		}
		
		writer.commit(3).unwrap();
		
		let bytes = writer.to_bytes();
		let reader = WalReader::from_bytes(&bytes).unwrap();
		
		let checkpoint = WalCheckpoint::new(4096);
		let (updates, result) = checkpoint.checkpoint(&reader, CheckpointMode::Full).unwrap();
		
		assert_eq!(updates.len(), 3);
		assert_eq!(result.wal_frames, 3);
		assert_eq!(result.checkpointed_frames, 3);
		assert!(result.completed);
		
		// Verify the updates
		for (page_num, data) in &updates {
			assert_eq!(data[0], *page_num as u8);
		}
	}

	#[test]
	fn test_checkpoint_page_updates() {
		let mut writer = WalWriter::new(4096);
		
		// Write page 1 twice
		let frame1 = WalFrame::new(1, vec![42u8; 4096], 0, 0);
		writer.add_frame(frame1).unwrap();
		
		let frame2 = WalFrame::new(1, vec![99u8; 4096], 0, 0);
		writer.add_frame(frame2).unwrap();
		
		writer.commit(1).unwrap();
		
		let bytes = writer.to_bytes();
		let reader = WalReader::from_bytes(&bytes).unwrap();
		
		let checkpoint = WalCheckpoint::new(4096);
		let (updates, _result) = checkpoint.checkpoint(&reader, CheckpointMode::Full).unwrap();
		
		// Should only have one update for page 1 with the latest value
		assert_eq!(updates.len(), 1);
		assert_eq!(updates[0].0, 1);
		assert_eq!(updates[0].1[0], 99);
	}

	#[test]
	fn test_checkpoint_passive() {
		let mut writer = WalWriter::new(4096);
		
		let frame = WalFrame::new(1, vec![42u8; 4096], 0, 0);
		writer.add_frame(frame).unwrap();
		writer.commit(1).unwrap();
		
		let bytes = writer.to_bytes();
		let reader = WalReader::from_bytes(&bytes).unwrap();
		
		let checkpoint = WalCheckpoint::new(4096);
		let (updates, result) = checkpoint.checkpoint(&reader, CheckpointMode::Passive).unwrap();
		
		assert_eq!(updates.len(), 1);
		assert!(result.completed);
	}

	#[test]
	fn test_recovery_basic() {
		let mut writer = WalWriter::new(4096);
		
		// Write some pages
		for i in 1..=3 {
			let mut data = vec![0u8; 4096];
			data[0] = i as u8;
			let frame = WalFrame::new(i, data, 0, 0);
			writer.add_frame(frame).unwrap();
		}
		
		writer.commit(3).unwrap();
		
		let bytes = writer.to_bytes();
		
		let recovery = WalRecovery::new(4096);
		let updates = recovery.recover(&bytes).unwrap();
		
		assert_eq!(updates.len(), 3);
		
		// Verify recovered data
		for (page_num, data) in &updates {
			assert_eq!(data[0], *page_num as u8);
		}
	}

	#[test]
	fn test_recovery_needs_recovery() {
		let mut writer = WalWriter::new(4096);
		
		// Empty WAL
		let bytes = writer.to_bytes();
		assert!(!WalRecovery::needs_recovery(&bytes));
		
		// WAL with data
		let frame = WalFrame::new(1, vec![0u8; 4096], 0, 0);
		writer.add_frame(frame).unwrap();
		writer.commit(1).unwrap();
		
		let bytes = writer.to_bytes();
		assert!(WalRecovery::needs_recovery(&bytes));
	}

	#[test]
	fn test_recovery_page_updates() {
		let mut writer = WalWriter::new(4096);
		
		// Write page 1 multiple times
		let frame1 = WalFrame::new(1, vec![1u8; 4096], 0, 0);
		writer.add_frame(frame1).unwrap();
		
		let frame2 = WalFrame::new(1, vec![2u8; 4096], 0, 0);
		writer.add_frame(frame2).unwrap();
		
		let frame3 = WalFrame::new(1, vec![3u8; 4096], 0, 0);
		writer.add_frame(frame3).unwrap();
		
		writer.commit(1).unwrap();
		
		let bytes = writer.to_bytes();
		
		let recovery = WalRecovery::new(4096);
		let updates = recovery.recover(&bytes).unwrap();
		
		// Should get only the latest version
		assert_eq!(updates.len(), 1);
		assert_eq!(updates[0].0, 1);
		assert_eq!(updates[0].1[0], 3);
	}

	#[test]
	fn test_checkpoint_no_commit() {
		let mut writer = WalWriter::new(4096);
		
		// Write frames but don't commit
		let frame = WalFrame::new(1, vec![42u8; 4096], 0, 0);
		writer.add_frame(frame).unwrap();
		
		let bytes = writer.to_bytes();
		let reader = WalReader::from_bytes(&bytes).unwrap();
		
		let checkpoint = WalCheckpoint::new(4096);
		let (updates, result) = checkpoint.checkpoint(&reader, CheckpointMode::Full).unwrap();
		
		// No committed frames, so no updates
		assert_eq!(updates.len(), 0);
		assert_eq!(result.checkpointed_frames, 0);
		assert!(!result.completed);
	}
}
