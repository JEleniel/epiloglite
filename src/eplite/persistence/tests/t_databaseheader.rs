#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{
        CInt,
        persistence::{
            CURRENT_FORMAT_VERSION, DEFAULT_PAGE_SIZE_EXPONENT, DatabaseHeader, EPLITE_SIGNATURE,
            HeaderError, MAX_HEADER_SIZE,
        },
    };

    #[test]
    fn test_default_header_is_valid() {
        let header = DatabaseHeader::default();
        assert_eq!(header.signature, EPLITE_SIGNATURE);
        assert_eq!(header.format_version, CURRENT_FORMAT_VERSION);
        assert_eq!(header.page_size_exponent, DEFAULT_PAGE_SIZE_EXPONENT);
        assert_eq!(header.page_size(), 1 << DEFAULT_PAGE_SIZE_EXPONENT);
        assert_eq!(header.flags, CInt::zero());
        assert_eq!(header.freelist_offset.page_number, CInt::zero());
        assert_eq!(
            header.freelist_offset.offset,
            CInt::from(MAX_HEADER_SIZE as u128)
        );
        assert_eq!(header.application_id, CInt::zero());
        assert_eq!(header.migration_version, CInt::zero());
        assert_eq!(header.crc, header.calculate_crc());
    }

    #[test]
    fn test_header_serialization_and_deserialization() {
        let header = DatabaseHeader::default();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let parsed = DatabaseHeader::try_from(&mut cursor).unwrap();
        assert_eq!(header, parsed);
    }

    #[test]
    fn test_invalid_signature() {
        let mut header = DatabaseHeader::default();
        header.signature = vec![b'W', b'r', b'o', b'n', b'g', b'M', b'a', b'g', b'i', b'c'];
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let err = DatabaseHeader::try_from(&mut cursor).unwrap_err();
        match err {
            HeaderError::InvalidHeaderSignature(s) => assert_eq!(s, "WrongMagic"),
            _ => panic!("Expected InvalidHeaderSignature"),
        }
    }

    #[test]
    fn test_format_too_new() {
        let mut header = DatabaseHeader::default();
        header.format_version = CURRENT_FORMAT_VERSION + 1;
        header.crc = header.calculate_crc();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let err = DatabaseHeader::try_from(&mut cursor).unwrap_err();
        match err {
            HeaderError::FormatTooNew(v) => assert_eq!(v, CURRENT_FORMAT_VERSION + 1),
            _ => panic!("Expected FormatTooNew"),
        }
    }

    #[test]
    fn test_invalid_page_size() {
        let mut header = DatabaseHeader::default();
        header.page_size_exponent = 8; // 2^8 = 256, below valid range
        header.crc = header.calculate_crc();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let err = DatabaseHeader::try_from(&mut cursor).unwrap_err();
        match err {
            HeaderError::InvalidPageSize(size) => assert_eq!(size, 1 << 8),
            _ => panic!("Expected InvalidPageSize"),
        }
    }

    #[test]
    fn test_invalid_freelist_offset_page_number() {
        let mut header = DatabaseHeader::default();
        header.freelist_offset.page_number = CInt::from(1);
        header.crc = header.calculate_crc();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        assert!(DatabaseHeader::try_from(&mut cursor).is_err());
    }

    #[test]
    fn test_invalid_freelist_offset_offset() {
        let mut header = DatabaseHeader::default();
        header.freelist_offset.offset = CInt::from(1); // less than MAX_HEADER_SIZE
        header.crc = header.calculate_crc();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        match DatabaseHeader::try_from(&mut cursor) {
            Ok(_) => panic!("Expected error due to invalid freelist offset"),
            Err(HeaderError::InvalidFreelistOffset(_page, offset, _expected)) => {
                assert_eq!(offset, CInt::from(1));
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        };
    }

    #[test]
    fn test_invalid_crc() {
        let mut header = DatabaseHeader::default();
        header.crc = 0xDEADBEEF; // wrong CRC
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let err = DatabaseHeader::try_from(&mut cursor).unwrap_err();
        match err {
            HeaderError::InvalidCRC(calc, expected) => {
                assert_ne!(calc, expected);
                assert_eq!(expected, 0xDEADBEEF);
            }
            _ => panic!("Expected InvalidCRC"),
        }
    }
}
