use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};

/// ## `CFF` &mdash; Compact Font Format table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/cff>.
///
/// This table contains a Compact Font Format font representation (also known as a PostScript
/// Type 1, or CIDFont) and is structured according to
/// [*Adobe Technical Note #5176: The Compact Font Format Specification*](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/font/pdfs/5176.CFF.pdf)
/// and [*Adobe Technical Note #5177: Type 2 Charstring Format*](https://wwwimages2.adobe.com/content/dam/acom/en/devnet/font/pdfs/5177.Type2.pdf).

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_CFF_ {
    _version: String,
    header_size: u8,
    offset_size: u8,
    name: Vec<String>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_CFF_(&mut self, buffer: &mut Buffer) {
        let cff_start_offset = buffer.offset;
        let _version = buffer.get_version::<u8>();
        let header_size = buffer.get::<u8>();
        let offset_size = buffer.get::<u8>();
        buffer.offset = cff_start_offset + header_size as usize;
        let name = buffer.get::<Index>().data
            .iter()
            .map(|i| String::from_utf8(i.to_vec()).unwrap())
            .collect();
        self.CFF_ = Some(Table_CFF_ {
            _version,
            header_size,
            offset_size,
            name,
        });
    }
}

// Card8 = u8
// Card16 = u16
// OffSize = u8 (range in 1..4)
// Offset: dependent on OffSize

// struct Dict {
// }

#[derive(Debug, Default)]
struct Index {
    count: usize,  // Actual type is `u16`
    offset_size: u8,
    offset: Vec<usize>,  // Actual type is `Offset[]`
    data: Vec<Vec<u8>>,
}

impl ReadBuffer for Index {
    fn read(buffer: &mut Buffer) -> Self {
        let count = buffer.get::<u16>() as usize;
        if count == 0 {
            Self {
                count,
                ..Default::default()
            }
        } else {
            let offset_size = buffer.get::<u8>();
            macro_rules! _get_offset {
                ($t:ty) => {
                    buffer.get_vec::<$t>(count + 1).iter().map(|&i| i as usize).collect()
                }
            }
            let offset: Vec<usize> = match offset_size {
                1 => _get_offset!(u8),
                2 => _get_offset!(u16),
                3 => _get_offset!(u32),
                4 => _get_offset!(u64),
                _ => unreachable!(),
            };
            let data = (0..count)
                .map(|i| buffer.get_vec::<u8>(offset[i + 1] - offset[i]))
                .collect();
            Self {
                count,
                offset_size,
                offset,
                data,
            }
        }
    }
}
