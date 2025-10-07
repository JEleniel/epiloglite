/// Integration tests for WAL mode

use epiloglite::eplite::persistence::pager::{JournalMode, Pager};
use epiloglite::eplite::persistence::wal::{CheckpointMode, WalFrame, WalReader, WalWriter};

#[test]
fn test_wal_write_read_cycle() {
	let mut writer = WalWriter::new(4096);

	// Write multiple pages
	for i in 1..=5 {
		let mut data = vec![0u8; 4096];
		data[0] = i as u8;
		data[1] = (i * 2) as u8;
		let frame = WalFrame::new(i, data, 0, 0);
		writer.add_frame(frame).unwrap();
	}

	writer.commit(5).unwrap();

	// Serialize and read back
	let bytes = writer.to_bytes();
	let reader = WalReader::from_bytes(&bytes).unwrap();

	// Verify all pages
	for i in 1..=5 {
		let page = reader.get_page(i).unwrap();
		assert_eq!(page[0], i as u8);
		assert_eq!(page[1], (i * 2) as u8);
	}

	assert_eq!(reader.frame_count(), 5);
}

#[test]
fn test_wal_multiple_transactions() {
	let mut writer = WalWriter::new(4096);

	// First transaction
	let frame1 = WalFrame::new(1, vec![1u8; 4096], 0, 0);
	writer.add_frame(frame1).unwrap();
	writer.commit(1).unwrap();

	let bytes1 = writer.to_bytes();
	let reader1 = WalReader::from_bytes(&bytes1).unwrap();
	assert_eq!(reader1.get_page(1).unwrap()[0], 1);

	// Second transaction (after reset)
	writer.reset();
	let frame2 = WalFrame::new(2, vec![2u8; 4096], 0, 0);
	writer.add_frame(frame2).unwrap();
	writer.commit(2).unwrap();

	let bytes2 = writer.to_bytes();
	let reader2 = WalReader::from_bytes(&bytes2).unwrap();
	assert_eq!(reader2.get_page(2).unwrap()[0], 2);
	assert!(reader2.get_page(1).is_none()); // Page 1 not in this transaction
}

#[test]
fn test_wal_page_versioning() {
	let mut writer = WalWriter::new(4096);

	// Write page 1 three times with different values
	for version in 1..=3 {
		let mut data = vec![0u8; 4096];
		data[0] = version;
		data[100] = version * 10;
		let frame = WalFrame::new(1, data, 0, 0);
		writer.add_frame(frame).unwrap();
	}

	writer.commit(1).unwrap();

	let bytes = writer.to_bytes();
	let reader = WalReader::from_bytes(&bytes).unwrap();

	// Should get the latest version (3)
	let page = reader.get_page(1).unwrap();
	assert_eq!(page[0], 3);
	assert_eq!(page[100], 30);
}

#[test]
fn test_wal_concurrent_readers() {
	let mut writer = WalWriter::new(4096);

	// Transaction 1: Write pages 1 and 2
	let frame1 = WalFrame::new(1, vec![1u8; 4096], 0, 0);
	writer.add_frame(frame1).unwrap();
	let frame2 = WalFrame::new(2, vec![2u8; 4096], 0, 0);
	writer.add_frame(frame2).unwrap();
	writer.commit(2).unwrap();

	let bytes = writer.to_bytes();

	// Simulate multiple concurrent readers
	let reader1 = WalReader::from_bytes(&bytes).unwrap();
	let reader2 = WalReader::from_bytes(&bytes).unwrap();

	// Both readers should see the same data
	assert_eq!(reader1.get_page(1).unwrap()[0], 1);
	assert_eq!(reader2.get_page(1).unwrap()[0], 1);
	assert_eq!(reader1.get_page(2).unwrap()[0], 2);
	assert_eq!(reader2.get_page(2).unwrap()[0], 2);
}

#[test]
#[cfg(feature = "std")]
fn test_pager_wal_mode_integration() {
	use epiloglite::eplite::os::file::DefaultFile;

	let temp_dir = std::env::temp_dir();
	let db_path = temp_dir.join("test_wal_db.db");
	let wal_path = temp_dir.join("test_wal_db.db-wal");

	// Create pager with WAL mode
	let db_file = DefaultFile::open(&db_path, true, true, true).unwrap();
	let wal_file = DefaultFile::open(&wal_path, true, true, true).unwrap();

	let mut pager = Pager::with_file(4096, Box::new(db_file)).unwrap();
	pager
		.set_journal_mode(JournalMode::Wal, Some(Box::new(wal_file)))
		.unwrap();

	// Begin transaction and write some data
	pager.begin_transaction().unwrap();

	let page1 = pager.get_page_mut(1).unwrap();
	page1.write(0, b"Hello WAL!").unwrap();

	let page2 = pager.get_page_mut(2).unwrap();
	page2.write(0, b"WAL Mode Works!").unwrap();

	// Commit
	pager.commit_transaction().unwrap();

	// Verify data is accessible
	let page1 = pager.get_page(1).unwrap();
	assert_eq!(&page1.data[0..10], b"Hello WAL!");

	// Cleanup
	drop(pager);
	let _ = std::fs::remove_file(&db_path);
	let _ = std::fs::remove_file(&wal_path);
}

#[test]
#[cfg(feature = "std")]
fn test_checkpoint_integration() {
	use epiloglite::eplite::os::file::DefaultFile;

	let temp_dir = std::env::temp_dir();
	let db_path = temp_dir.join("test_checkpoint.db");
	let wal_path = temp_dir.join("test_checkpoint.db-wal");

	// Create pager with WAL mode
	let db_file = DefaultFile::open(&db_path, true, true, true).unwrap();
	let wal_file = DefaultFile::open(&wal_path, true, true, true).unwrap();

	let mut pager = Pager::with_file(4096, Box::new(db_file)).unwrap();
	pager
		.set_journal_mode(JournalMode::Wal, Some(Box::new(wal_file)))
		.unwrap();

	// Write some data
	pager.begin_transaction().unwrap();
	let page = pager.get_page_mut(1).unwrap();
	page.write(0, b"Test Data").unwrap();
	pager.commit_transaction().unwrap();

	// Perform checkpoint
	pager.checkpoint(CheckpointMode::Full).unwrap();

	// Cleanup
	drop(pager);
	let _ = std::fs::remove_file(&db_path);
	let _ = std::fs::remove_file(&wal_path);
}

#[test]
fn test_wal_recovery_scenario() {
	use epiloglite::eplite::persistence::wal::WalRecovery;

	let mut writer = WalWriter::new(4096);

	// Simulate a crash scenario - write data but don't explicitly commit
	for i in 1u32..=3 {
		let mut data = vec![0u8; 4096];
		data[0] = (i * 10) as u8;
		let frame = WalFrame::new(i, data, 0, 0);
		writer.add_frame(frame).unwrap();
	}

	writer.commit(3).unwrap();

	// Serialize WAL
	let wal_bytes = writer.to_bytes();

	// Simulate recovery
	let recovery = WalRecovery::new(4096);
	let recovered_pages = recovery.recover(&wal_bytes).unwrap();

	assert_eq!(recovered_pages.len(), 3);

	// Verify recovered data
	for (page_num, data) in recovered_pages {
		assert_eq!(data[0], page_num as u8 * 10);
	}
}

#[test]
fn test_wal_large_transaction() {
	let mut writer = WalWriter::new(4096);

	// Write many pages in a single transaction
	for i in 1u32..=100 {
		let mut data = vec![0u8; 4096];
		data[0] = (i % 256) as u8;
		data[4095] = ((i * 7) % 256) as u8;
		let frame = WalFrame::new(i, data, 0, 0);
		writer.add_frame(frame).unwrap();
	}

	writer.commit(100).unwrap();

	let bytes = writer.to_bytes();
	let reader = WalReader::from_bytes(&bytes).unwrap();

	assert_eq!(reader.frame_count(), 100);

	// Spot check some pages
	let page1 = reader.get_page(1).unwrap();
	assert_eq!(page1[0], 1);

	let page50 = reader.get_page(50).unwrap();
	assert_eq!(page50[0], 50);

	let page100 = reader.get_page(100).unwrap();
	assert_eq!(page100[0], 100);
}

#[test]
fn test_wal_interleaved_pages() {
	let mut writer = WalWriter::new(4096);

	// Write pages in non-sequential order with updates
	let pages_to_write: Vec<(u32, u8)> = vec![
		(5, 1),
		(3, 1),
		(1, 1),
		(3, 2), // Update page 3
		(7, 1),
		(1, 2), // Update page 1
	];

	for (page_num, version) in pages_to_write {
		let mut data = vec![0u8; 4096];
		data[0] = page_num as u8;
		data[1] = version;
		let frame = WalFrame::new(page_num, data, 0, 0);
		writer.add_frame(frame).unwrap();
	}

	writer.commit(7).unwrap();

	let bytes = writer.to_bytes();
	let reader = WalReader::from_bytes(&bytes).unwrap();

	// Verify we get the latest versions
	let page1 = reader.get_page(1).unwrap();
	assert_eq!(page1[0], 1);
	assert_eq!(page1[1], 2); // Latest version

	let page3 = reader.get_page(3).unwrap();
	assert_eq!(page3[0], 3);
	assert_eq!(page3[1], 2); // Latest version

	let page5 = reader.get_page(5).unwrap();
	assert_eq!(page5[1], 1); // Only one version

	let page7 = reader.get_page(7).unwrap();
	assert_eq!(page7[1], 1); // Only one version
}
