use std::fmt;

use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};
use read_buffer_derive::ReadBuffer;

/// ## `CPAL` &mdash; Color Palette Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/cpal>.
///
/// The palette table is a set of one or more palettes, each containing a predefined
/// number of color records. It may also contain `name` table IDs describing the palettes
/// and their entries.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_CPAL {
    _version: u16,
    pub num_palette_entries: u16,
    pub num_palettes: u16,
    pub num_color_records: u16,
    pub palettes: Vec<Palette>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_CPAL(&mut self, buffer: &mut Buffer) {
        let cpal_start_offset = buffer.offset();
        let _version = buffer.get();
        let num_palette_entries = buffer.get();
        let num_palettes = buffer.get();
        let num_color_records = buffer.get();
        let color_records_array_offset: u32 = buffer.get();
        let color_record_indices: Vec<u16> = {
            let mut v = buffer.get_vec(num_palettes);
            v.push(num_color_records);
            v
        };

        let mut palette_types_array_offset = 0u32;
        let mut palette_labels_array_offset = 0u32;
        let mut palette_entry_labels_array_offset = 0u32;
        if _version == 1 {
            palette_types_array_offset = buffer.get();
            palette_labels_array_offset = buffer.get();
            palette_entry_labels_array_offset = buffer.get();
        }

        buffer.set_offset_from(cpal_start_offset, color_records_array_offset);
        let color_records_array: Vec<ColorRecord> = buffer.get_vec(num_color_records);
        let mut palettes: Vec<Palette> = (0..num_palettes)
            .map(|i| {
                let i = i as usize;
                let color_records = (color_record_indices[i]..color_record_indices[i + 1])
                    .map(|j| color_records_array[j as usize])
                    .collect();
                Palette {
                    color_records,
                    ..Default::default()
                }
            })
            .collect();

        if _version == 1 {
            buffer.set_offset_from(cpal_start_offset, palette_types_array_offset);
            let palette_types = buffer.get_vec(num_palettes);
            buffer.set_offset_from(cpal_start_offset, palette_labels_array_offset);
            let palette_labels = buffer.get_vec(num_palettes);
            buffer.set_offset_from(cpal_start_offset, palette_entry_labels_array_offset);
            let palette_entry_labels = buffer.get_vec(num_palettes);

            (0..num_palettes).for_each(|i| {
                let i = i as usize;
                palettes[i].r#type = Some(palette_types[i]);
                palettes[i].label = Some(palette_labels[i]);
                palettes[i].entry_label = Some(palette_entry_labels[i]);
            });
        }

        self.CPAL = Some(Table_CPAL {
            _version,
            num_palette_entries,
            num_palettes,
            num_color_records,
            palettes,
        })
    }
}

#[derive(Debug, Default)]
pub struct Palette {
    color_records: Vec<ColorRecord>,
    r#type: Option<u32>,
    label: Option<u16>,
    entry_label: Option<u16>,
}

/// Each color record has BGRA values. The color space for these values is sRGB.
#[derive(ReadBuffer, Clone, Copy)]
pub struct ColorRecord {
    blue: u8,
    green: u8,
    red: u8,
    alpha: u8,
}

impl fmt::Debug for ColorRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\"#{:02x}{:02x}{:02x}{:02x}\"",
            self.red, self.green, self.blue, self.alpha
        )
    }
}
