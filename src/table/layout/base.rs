use crate::font::Font;
use crate::util::{Buffer, ReadBuffer, Tag};

/// ## `BASE` &mdash; Baseline Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/base>.
///
/// The Baseline table (`BASE`) provides information used to align glyphs of different
/// scripts and sizes in a line of text, whether the glyphs are in the same font or in
/// different fonts. To improve text layout, the Baseline table also provides minimum (min)
/// and maximum (max) glyph extent values for each script, language system, or feature in a font.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_BASE {
    _version: String,
    pub horiz_axis: Option<Axis>,
    pub vert_axis: Option<Axis>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_BASE(&mut self, buffer: &mut Buffer) {
        let base_start_offset = buffer.offset;
        let _version = buffer.get_version::<u16>();
        let horiz_axis_offset: u16 = buffer.get();
        let vert_axis_offset: u16 = buffer.get();
        // TODO: otvar
        #[allow(unused_variables)]
        let item_var_store_offset: Option<u32> = match _version.as_str() {
            "1.1" => Some(buffer.get()),
            _ => None,
        };
        self.BASE = Some(Table_BASE {
            _version,
            horiz_axis: buffer.get_or_none(base_start_offset, horiz_axis_offset as usize),
            vert_axis: buffer.get_or_none(base_start_offset, vert_axis_offset as usize),
        });
    }
}

#[derive(Debug)]
pub struct Axis {
    pub base_tag_list: Vec<Tag>,
    pub base_script_list: Vec<BaseScriptRecord>,
}

impl ReadBuffer for Axis {
    fn read(buffer: &mut Buffer) -> Self {
        let axis_start_offset = buffer.offset;
        let base_tag_list_offset: u16 = buffer.get();
        let base_script_list_offset: u16 = buffer.get();

        buffer.offset = axis_start_offset + base_tag_list_offset as usize;
        let base_tag_count: u16 = buffer.get();
        let base_tag_list = buffer.get_vec(base_tag_count as usize);

        let base_script_list_start_offset = axis_start_offset + base_script_list_offset as usize;
        buffer.offset = base_script_list_start_offset;
        let base_script_count: u16 = buffer.get();
        let mut base_script_list: Vec<BaseScriptRecord> =
            buffer.get_vec(base_script_count as usize);
        base_script_list.iter_mut().for_each(|rec| {
            buffer.offset = base_script_list_start_offset + rec.base_script_offset as usize;
            rec.base_script = buffer.get();
        });

        Self {
            base_tag_list,
            base_script_list,
        }
    }
}

#[derive(Debug, Default)]
pub struct BaseScriptRecord {
    pub base_script_tag: Tag,
    pub base_script: BaseScript,
    base_script_offset: u16,
}

impl ReadBuffer for BaseScriptRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            base_script_tag: buffer.get(),
            base_script_offset: buffer.get(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct BaseScript {
    pub base_values: Option<BaseValues>,
    pub default_min_max: Option<MinMax>,
    pub base_lang_sys_records: Vec<BaseLangSysRecord>,
}

impl ReadBuffer for BaseScript {
    fn read(buffer: &mut Buffer) -> Self {
        let start_offset = buffer.offset;
        let base_values_offset: u16 = buffer.get();
        let default_min_max_offset: u16 = buffer.get();
        let base_lang_sys_count: u16 = buffer.get();
        let mut base_lang_sys_records: Vec<BaseLangSysRecord> =
            buffer.get_vec(base_lang_sys_count as usize);
        base_lang_sys_records.iter_mut().for_each(|rec| {
            buffer.offset = start_offset + rec.min_max_offset as usize;
            rec.min_max = Some(buffer.get());
        });
        Self {
            base_values: buffer.get_or_none(start_offset, base_values_offset as usize),
            default_min_max: buffer.get_or_none(start_offset, default_min_max_offset as usize),
            base_lang_sys_records,
        }
    }
}

#[derive(Debug, Default)]
pub struct BaseLangSysRecord {
    pub base_lang_sys_tag: Tag,
    pub min_max: Option<MinMax>,
    min_max_offset: u16,
}

impl ReadBuffer for BaseLangSysRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            base_lang_sys_tag: buffer.get(),
            min_max_offset: buffer.get(),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct BaseValues {
    pub default_baseline_index: u16,
    pub base_coords: Vec<BaseCoord>,
}

impl ReadBuffer for BaseValues {
    fn read(buffer: &mut Buffer) -> Self {
        let start_offset = buffer.offset;
        let default_baseline_index = buffer.get();
        let base_coord_count: u16 = buffer.get();
        let base_coord_offsets: Vec<u16> = buffer.get_vec(base_coord_count as usize);
        let base_coords = base_coord_offsets
            .iter()
            .map(|&offset| {
                buffer.offset = start_offset + offset as usize;
                buffer.get()
            })
            .collect();
        Self {
            default_baseline_index,
            base_coords,
        }
    }
}

#[derive(Debug)]
pub struct MinMax {
    pub min_coord: Option<BaseCoord>,
    pub max_coord: Option<BaseCoord>,
    pub feat_min_max_records: Vec<FeatureMinMaxRecord>,
}

impl ReadBuffer for MinMax {
    fn read(buffer: &mut Buffer) -> Self {
        let start_offset = buffer.offset;
        let min_coord_offset: u16 = buffer.get();
        let max_coord_offset: u16 = buffer.get();
        let feat_min_max_count: u16 = buffer.get();
        let feat_min_max_records = buffer.get_vec(feat_min_max_count as usize);
        Self {
            min_coord: buffer.get_or_none(start_offset, min_coord_offset as usize),
            max_coord: buffer.get_or_none(start_offset, max_coord_offset as usize),
            feat_min_max_records,
        }
    }
}

#[derive(Debug)]
pub struct FeatureMinMaxRecord {
    pub feature_table_tag: Tag,
    pub min_coord: Option<BaseCoord>,
    pub max_coord: Option<BaseCoord>,
}

impl ReadBuffer for FeatureMinMaxRecord {
    fn read(buffer: &mut Buffer) -> Self {
        let start_offset = buffer.offset;
        let feature_table_tag = buffer.get();
        let min_coord_offset: u16 = buffer.get();
        let max_coord_offset: u16 = buffer.get();
        Self {
            feature_table_tag,
            min_coord: buffer.get_or_none(start_offset, min_coord_offset as usize),
            max_coord: buffer.get_or_none(start_offset, max_coord_offset as usize),
        }
    }
}

#[derive(Debug, Default)]
pub struct BaseCoord {
    pub format: u16,
    pub coordinate: i16,
    pub reference_glyph: Option<u16>,
    pub base_coord_point: Option<u16>,
    pub device_offset: Option<u16>,
}

impl ReadBuffer for BaseCoord {
    fn read(buffer: &mut Buffer) -> Self {
        let format = buffer.get();
        let coordinate = buffer.get();
        let mut base_coord = Self {
            format,
            coordinate,
            ..Default::default()
        };
        match format {
            1 => {}
            2 => {
                base_coord.reference_glyph = Some(buffer.get());
                base_coord.base_coord_point = Some(buffer.get());
            }
            3 => base_coord.device_offset = Some(buffer.get()),
            _ => unreachable!(),
        }
        base_coord
    }
}

impl Buffer {
    fn get_or_none<T: ReadBuffer>(&mut self, start_offset: usize, offset: usize) -> Option<T> {
        if offset != 0 {
            self.offset = start_offset + offset;
            Some(self.get::<T>())
        } else {
            None
        }
    }
}
