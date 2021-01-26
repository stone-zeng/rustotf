use std::fmt;

use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};
use read_buffer_derive::ReadBuffer;

/// ## `MATH` &mdash; The Mathematical Typesetting Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/math>.
///
/// Mathematical formulas are complex text objects in which multiple elements with various
/// metric, style or positioning attributes are combined. In order for a math-layout engine
/// to support layout of mathematical formulas, several types of font-specific information
/// particular to the layout of formulas are required. The `MATH` table provides this
/// font-specific information necessary for math formula layout.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_MATH {
    _version: String,
    pub math_constants: MathConstants,
    pub math_glyph_info: MathGlyphInfo,
    pub math_variants: MathVariants,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_MATH(&mut self, buffer: &mut Buffer) {
        let math_start_offset = buffer.offset;
        let _version = buffer.get_version::<u16>();
        let math_constants_offset: u16 = buffer.get();
        let math_glyph_info_offset: u16 = buffer.get();
        let math_variants_offset: u16 = buffer.get();

        buffer.offset = math_start_offset + math_constants_offset as usize;
        let math_constants = buffer.get();

        buffer.offset = math_start_offset + math_glyph_info_offset as usize;
        let math_glyph_info = buffer.get();

        buffer.offset = math_start_offset + math_variants_offset as usize;
        let math_variants = buffer.get();

        self.MATH = Some(Table_MATH {
            _version,
            math_constants,
            math_glyph_info,
            math_variants,
        });
    }
}

#[derive(Debug, ReadBuffer)]
pub struct MathConstants {
    pub script_percent_scale_down: i16,
    pub script_script_percent_scale_down: i16,
    pub delimited_sub_formula_min_height: u16,
    pub display_operator_min_height: u16,
    pub math_leading: MathValueRecord,
    pub axis_height: MathValueRecord,
    pub accent_base_height: MathValueRecord,
    pub flattened_accent_base_height: MathValueRecord,
    pub subscript_shift_down: MathValueRecord,
    pub subscript_top_max: MathValueRecord,
    pub subscript_baseline_drop_min: MathValueRecord,
    pub superscript_shift_up: MathValueRecord,
    pub superscript_shift_up_cramped: MathValueRecord,
    pub superscript_bottom_min: MathValueRecord,
    pub superscript_baseline_drop_max: MathValueRecord,
    pub sub_superscript_gap_min: MathValueRecord,
    pub superscript_bottom_max_with_subscript: MathValueRecord,
    pub space_after_script: MathValueRecord,
    pub upper_limit_gap_min: MathValueRecord,
    pub upper_limit_baseline_rise_min: MathValueRecord,
    pub lower_limit_gap_min: MathValueRecord,
    pub lower_limit_baseline_drop_min: MathValueRecord,
    pub stack_top_shift_up: MathValueRecord,
    pub stack_top_display_style_shift_up: MathValueRecord,
    pub stack_bottom_shift_down: MathValueRecord,
    pub stack_bottom_display_style_shift_down: MathValueRecord,
    pub stack_gap_min: MathValueRecord,
    pub stack_display_style_gap_min: MathValueRecord,
    pub stretch_stack_top_shift_up: MathValueRecord,
    pub stretch_stack_bottom_shift_down: MathValueRecord,
    pub stretch_stack_gap_above_min: MathValueRecord,
    pub stretch_stack_gap_below_min: MathValueRecord,
    pub fraction_numerator_shift_up: MathValueRecord,
    pub fraction_numerator_display_style_shift_up: MathValueRecord,
    pub fraction_denominator_shift_down: MathValueRecord,
    pub fraction_denominator_display_style_shift_down: MathValueRecord,
    pub fraction_numerator_gap_min: MathValueRecord,
    pub fraction_num_display_style_gap_min: MathValueRecord,
    pub fraction_rule_thickness: MathValueRecord,
    pub fraction_denominator_gap_min: MathValueRecord,
    pub fraction_denom_display_style_gap_min: MathValueRecord,
    pub skewed_fraction_horizontal_gap: MathValueRecord,
    pub skewed_fraction_vertical_gap: MathValueRecord,
    pub overbar_vertical_gap: MathValueRecord,
    pub overbar_rule_thickness: MathValueRecord,
    pub overbar_extra_ascender: MathValueRecord,
    pub underbar_vertical_gap: MathValueRecord,
    pub underbar_rule_thickness: MathValueRecord,
    pub underbar_extra_descender: MathValueRecord,
    pub radical_vertical_gap: MathValueRecord,
    pub radical_display_style_vertical_gap: MathValueRecord,
    pub radical_rule_thickness: MathValueRecord,
    pub radical_extra_ascender: MathValueRecord,
    pub radical_kern_before_degree: MathValueRecord,
    pub radical_kern_after_degree: MathValueRecord,
    pub radical_degree_bottom_raise_percent: i16,
}

#[derive(Debug)]
pub struct MathGlyphInfo {
    pub math_italics_correction_info: MathItalicsCorrectionInfo,
    pub math_top_accent_attachment: MathTopAccentAttachment,
    pub extended_shape_coverage: Coverage,
    pub math_kern_info: MathKernInfo,
}

impl ReadBuffer for MathGlyphInfo {
    fn read(buffer: &mut Buffer) -> Self {
        let start_offset = buffer.offset;
        let math_italics_correction_info_offset: u16 = buffer.get();
        let math_top_accent_attachment_offset: u16 = buffer.get();
        let extended_shape_coverage_offset: u16 = buffer.get();
        let math_kern_info_offset: u16 = buffer.get();

        macro_rules! _get {
            ($offset:expr) => {{
                buffer.offset = start_offset + $offset as usize;
                buffer.get()
            }};
        }

        Self {
            math_italics_correction_info: _get!(math_italics_correction_info_offset),
            math_top_accent_attachment: _get!(math_top_accent_attachment_offset),
            extended_shape_coverage: _get!(extended_shape_coverage_offset),
            math_kern_info: _get!(math_kern_info_offset),
        }
    }
}

#[derive(Debug)]
pub struct MathItalicsCorrectionInfo {
    pub italics_correction_coverage: Coverage,
    pub italics_correction: Vec<MathValueRecord>,
}

impl ReadBuffer for MathItalicsCorrectionInfo {
    fn read(buffer: &mut Buffer) -> Self {
        let start_offset = buffer.offset;
        let italics_correction_coverage_offset: u16 = buffer.get();
        let italics_correction_count: u16 = buffer.get();
        let italics_correction = buffer.get_vec(italics_correction_count as usize);
        buffer.offset = start_offset + italics_correction_coverage_offset as usize;
        let italics_correction_coverage = buffer.get();
        Self {
            italics_correction_coverage,
            italics_correction,
        }
    }
}

#[derive(Debug)]
pub struct MathTopAccentAttachment {
    pub top_accent_attachment_coverage: Coverage,
    pub top_accent_attachment: Vec<MathValueRecord>,
}

impl ReadBuffer for MathTopAccentAttachment {
    fn read(buffer: &mut Buffer) -> Self {
        let start_offset = buffer.offset;
        let top_accent_attachment_coverage_offset: u16 = buffer.get();
        let top_accent_attachment_count: u16 = buffer.get();
        let top_accent_attachment = buffer.get_vec(top_accent_attachment_count as usize);
        buffer.offset = start_offset + top_accent_attachment_coverage_offset as usize;
        let top_accent_attachment_coverage = buffer.get();
        Self {
            top_accent_attachment_coverage,
            top_accent_attachment,
        }
    }
}

#[derive(Debug)]
pub struct MathKernInfo {
    pub math_kern_coverage: Coverage,
    pub math_kern: Vec<MathKernInfoRecord>,
}

impl ReadBuffer for MathKernInfo {
    fn read(buffer: &mut Buffer) -> Self {
        let start_offset = buffer.offset;
        let math_kern_coverage_offset: u16 = buffer.get();
        let math_kern_count: u16 = buffer.get();
        let mut math_kern: Vec<MathKernInfoRecord> = buffer.get_vec(math_kern_count as usize);

        macro_rules! _get_math_kern {
            ($offset:expr) => {
                match $offset {
                    0 => None,
                    _ => {
                        buffer.offset = start_offset + $offset as usize;
                        Some(buffer.get())
                    }
                }
            };
        }

        math_kern.iter_mut().for_each(|rec| {
            rec.top_right_math_kern = _get_math_kern!(rec.top_right_math_kern_offset);
            rec.top_left_math_kern = _get_math_kern!(rec.top_left_math_kern_offset);
            rec.bottom_right_math_kern = _get_math_kern!(rec.bottom_right_math_kern_offset);
            rec.bottom_left_math_kern = _get_math_kern!(rec.bottom_left_math_kern_offset);
        });

        buffer.offset = start_offset + math_kern_coverage_offset as usize;
        let math_kern_coverage = buffer.get();
        Self {
            math_kern_coverage,
            math_kern,
        }
    }
}

#[derive(Debug, Default)]
pub struct MathKernInfoRecord {
    pub top_right_math_kern: Option<MathKern>,
    pub top_left_math_kern: Option<MathKern>,
    pub bottom_right_math_kern: Option<MathKern>,
    pub bottom_left_math_kern: Option<MathKern>,
    top_right_math_kern_offset: u16,
    top_left_math_kern_offset: u16,
    bottom_right_math_kern_offset: u16,
    bottom_left_math_kern_offset: u16,
}

impl ReadBuffer for MathKernInfoRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            top_right_math_kern_offset: buffer.get(),
            top_left_math_kern_offset: buffer.get(),
            bottom_right_math_kern_offset: buffer.get(),
            bottom_left_math_kern_offset: buffer.get(),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct MathKern {
    pub height_count: u16,
    pub correction_height: Vec<MathValueRecord>,
    pub kern_values: Vec<MathValueRecord>,
}

impl ReadBuffer for MathKern {
    fn read(buffer: &mut Buffer) -> Self {
        let height_count = buffer.get();
        let correction_height = buffer.get_vec(height_count as usize);
        let kern_values = buffer.get_vec(height_count as usize + 1);
        Self {
            height_count,
            correction_height,
            kern_values,
        }
    }
}

#[derive(Debug)]
pub struct MathVariants {
    pub min_connector_overlap: u16,
    pub vert_glyph_coverage: Coverage,
    pub horiz_glyph_coverage: Coverage,
    pub vert_glyph_constructions: Vec<MathGlyphConstruction>,
    pub horiz_glyph_constructions: Vec<MathGlyphConstruction>,
}

impl ReadBuffer for MathVariants {
    fn read(buffer: &mut Buffer) -> Self {
        let start_offset = buffer.offset;
        let min_connector_overlap = buffer.get();
        let vert_glyph_coverage_offset: u16 = buffer.get();
        let horiz_glyph_coverage_offset: u16 = buffer.get();
        let vert_glyph_count: u16 = buffer.get();
        let horiz_glyph_count: u16 = buffer.get();
        let vert_glyph_construction_offsets: Vec<u16> = buffer.get_vec(vert_glyph_count as usize);
        let horiz_glyph_construction_offsets: Vec<u16> = buffer.get_vec(horiz_glyph_count as usize);
        let vert_glyph_coverage = {
            buffer.offset = start_offset + vert_glyph_coverage_offset as usize;
            buffer.get()
        };
        let horiz_glyph_coverage = {
            buffer.offset = start_offset + horiz_glyph_coverage_offset as usize;
            buffer.get()
        };
        let vert_glyph_constructions = vert_glyph_construction_offsets
            .iter()
            .map(|&offset| {
                buffer.offset = start_offset + offset as usize;
                buffer.get()
            })
            .collect();
        let horiz_glyph_constructions = horiz_glyph_construction_offsets
            .iter()
            .map(|&offset| {
                buffer.offset = start_offset + offset as usize;
                buffer.get()
            })
            .collect();
        Self {
            min_connector_overlap,
            vert_glyph_coverage,
            horiz_glyph_coverage,
            vert_glyph_constructions,
            horiz_glyph_constructions,
        }
    }
}

#[derive(Debug)]
pub struct MathGlyphConstruction {
    pub glyph_assembly: Option<GlyphAssembly>,
    pub math_glyph_variant_records: Vec<MathGlyphVariantRecord>,
}

impl ReadBuffer for MathGlyphConstruction {
    fn read(buffer: &mut Buffer) -> Self {
        let start_offset = buffer.offset;
        let glyph_assembly_offset: u16 = buffer.get();
        let variant_count: u16 = buffer.get();
        let math_glyph_variant_records = buffer.get_vec(variant_count as usize);
        let glyph_assembly = match glyph_assembly_offset {
            0 => None,
            _ => {
                buffer.offset = start_offset + glyph_assembly_offset as usize;
                Some(buffer.get())
            }
        };
        Self {
            glyph_assembly,
            math_glyph_variant_records,
        }
    }
}

#[derive(Debug)]
pub struct GlyphAssembly {
    pub italics_correction: MathValueRecord,
    pub part_records: Vec<GlyphPartRecord>,
}

impl ReadBuffer for GlyphAssembly {
    fn read(buffer: &mut Buffer) -> Self {
        let italics_correction = buffer.get();
        let part_count: u16 = buffer.get();
        let part_records = buffer.get_vec(part_count as usize);
        Self {
            italics_correction,
            part_records,
        }
    }
}

#[derive(Debug, ReadBuffer)]
pub struct GlyphPartRecord {
    pub glyph_id: u16,
    pub start_connector_length: u16,
    pub end_connector_length: u16,
    pub full_advance: u16,
    pub part_flags: u16,
}

#[derive(Debug, ReadBuffer)]
pub struct MathGlyphVariantRecord {
    pub variant_glyph: u16,
    pub advance_measurement: u16,
}

// Shared Formats

#[derive(ReadBuffer)]
pub struct MathValueRecord {
    pub value: i16,
    device_offset: u16,
}

impl fmt::Debug for MathValueRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.device_offset {
            0 => write!(f, "{}", self.value),
            _ => write!(f, "[{}, 0x{:0x}]", self.value, self.device_offset),
        }
    }
}

#[derive(Debug)]
pub struct Coverage {
    format: u16,
    pub glyph_array: Vec<u16>,
}

impl ReadBuffer for Coverage {
    fn read(buffer: &mut Buffer) -> Self {
        let format = buffer.get();
        let glyph_array = match format {
            1 => {
                let glyph_count: u16 = buffer.get();
                buffer.get_vec(glyph_count as usize)
            }
            2 => {
                let range_count: u16 = buffer.get();
                let range_records: Vec<RangeRecord> = buffer.get_vec(range_count as usize);
                let mut array = Vec::new();
                range_records.iter().for_each(|rec| {
                    (rec.start_glyph_id..=rec.end_glyph_id).for_each(|id| array.push(id));
                });
                array
            }
            _ => unreachable!(),
        };
        Self {
            format,
            glyph_array,
        }
    }
}

#[derive(ReadBuffer)]
struct RangeRecord {
    start_glyph_id: u16,
    end_glyph_id: u16,
    // TODO:
    _start_coverage_index: u16,
}
