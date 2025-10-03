# EpilogLite

EpilogLite is a Rust language library that implements a small, fast, self-contained, high-reliability, full-featured, SQL database engine based on SQLite. EpilogLite is not just a reimplementation; it includes new, updated, and extended capabilities.

SQLite is the most used database engine in the world. The SQLite file format is stable, cross-platform, and backwards compatible. EpilogLite strives to remain compatible with the SQLite file format.

## Documentation Conventions

- The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/info/rfc2119) regardless of casing.

- All data elements conform to the [Rust Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html).

	+ A "byte" is an unsigned, 8-bit integer (u8).
	+ Binary prefixes are used for multiples of bytes:

        | Prefix | Abbreviation | Description                       |
        |--------|--------------|-----------------------------------|
        | kibi   | Ki           | 2<sup>10</sup> (1024)             |
        | mebi   | Mi           | 2<sup>20</sup> (1048576)          |
        | gibi   | Gi           | 2<sup>30</sup> (1073741824)       |
        | tebi   | Ti           | 2<sup>40</sup> (1099511627776)    |
        | pebi   | Pi           | 2<sup>50</sup> (1125899906842624) |

        **Table 1:** Binary prefixes

	+ Integer literals are written as follows, without thousands separators:

    	| Number literals | Example    |
		|-----------------|------------|
		| Decimal         | 65         |
		| Hex             | 0x41       |
		| Octal           | 0o101      |
		| Binary          | 0b01000001 |
		| Byte (u8 only)  | b'A'       |

    	**Table 2:** Representation of integer literals

	+ [Rust escape sequences](https://doc.rust-lang.org/reference/tokens.html) are used in string literals.
