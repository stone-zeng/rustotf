use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};

/// ## `gasp` &mdash; Grid-fitting and Scan-conversion Procedure Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/gasp>.
///
/// This table contains information which describes the preferred rasterization techniques for
/// the typeface when it is rendered on grayscale-capable devices. This table also has some use
/// for monochrome devices, which may use the table to turn off hinting at very large or
/// small sizes, to improve performance.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_gasp {
    version: u16,
    num_ranges: u16,
    gasp_ranges: Vec<GaspRange>
}

impl Font {
    pub fn parse_gasp(&mut self, buffer: &mut Buffer) {
        let version = buffer.get();
        let num_ranges = buffer.get();
        let gasp_ranges = buffer.get_vec(num_ranges as usize);
        self.gasp = Some(Table_gasp {
            version,
            num_ranges,
            gasp_ranges,
        });
    }
}

#[derive(Debug)]
struct GaspRange {
    range_max_ppem: u16,
    range_gasp_behavior: u16,
}

impl ReadBuffer for GaspRange {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            range_max_ppem: buffer.get(),
            range_gasp_behavior: buffer.get(),
        }
    }
}
