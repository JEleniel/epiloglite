# EpilogLite Unicode Conformance

**Collation**: [Unicode Collation Algorithm 16.0.0](https://www.unicode.org/reports/tr10/tr10-51.html)[^2]
**Normalization**: [Unicode Normalization Forms](https://www.unicode.org/reports/tr15/tr15-69.html)

EpilogLite supports the full Unicode 16.0.0[^1] specification for characters. The default encoding is UTF-8 and the default collation is the Unicode Collation Algorithm (UCA) version 16.0.0[^2]. The default normalization form is NFC (Normalization Form C)[^3].

String literals may include any valid Unicode character.

**Encoding**:

| Encoding | Endienness | BOM Allowed |
|----------|------------|-------------|
| UTF-8    | N/A        | No          |
| UTF-16 | Either | Yes         |
| UTF-16BE | Big Endian | No |
| UTF-16LE | Little Endian | No |
| UTF-32 | Either | Yes |
| UTF-32BE | Big Endian | No |
| UTF-32LE | Little Endian | No |

[^1]: The Unicode Consortium. The Unicode Standard, Version 16.0.0, (South San Francisco: The Unicode Consortium, 2024. ISBN 978-1-936213-34-4) <https://www.unicode.org/versions/Unicode16.0.0/>
[^2]: The Unicode Consortium. Unicode® Technical Standard #10: Unicode Collation Algorithm 16.0.0, (Ken Whistler, Markus Scherer) <https://www.unicode.org/reports/tr10/tr10-51.html>
[^3]: The Unicode Consortium. Unicode® Technical Standard #15: Unicode Normalization Forms, (Markus Scherer) <https://www.unicode.org/reports/tr15/tr15-69.html>
