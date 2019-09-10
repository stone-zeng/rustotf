use crate::font::Font;
use crate::util::{Buffer, Tag};

/// ## `OS/2` &mdash; OS/2 and Windows Metrics Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/os2>.
///
/// The `OS/2` table consists of a set of metrics and other data that are
/// required in OpenType fonts.
///
/// Six versions of the `OS/2` table have been defined: versions 0 to 5.
/// All versions are supported, but use of version 4 or later is strongly
/// recommended.
///
/// **Note:** Documentation for `OS/2` version 0 in Apple's TrueType Reference
/// Manual stops at the `usLastCharIndex` field and does not include the last
/// five fields of the table as it was defined by Microsoft. Some legacy
/// TrueType fonts may have been built with a shortened version 0 `OS/2` table.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_OS_2 {
    _version: u16,
    // Version 0
    pub x_avg_char_width: i16,
    pub us_weight_class: u16,
    pub us_width_class: u16,
    pub fs_type: u16,
    pub y_subscript_x_size: i16,
    pub y_subscript_y_size: i16,
    pub y_subscript_x_offset: i16,
    pub y_subscript_y_offset: i16,
    pub y_superscript_x_size: i16,
    pub y_superscript_y_size: i16,
    pub y_superscript_x_offset: i16,
    pub y_superscript_y_offset: i16,
    pub y_strikeout_size: i16,
    pub y_strikeout_position: i16,
    pub s_family_class: i16,
    pub panose: Vec<u8>,
    pub ul_unicode_range1: u32,
    pub ul_unicode_range2: u32,
    pub ul_unicode_range3: u32,
    pub ul_unicode_range4: u32,
    pub ach_vend_i_d: Tag,
    pub fs_selection: u16,
    pub us_first_char_index: u16,
    pub us_last_char_index: u16,
    // Version 0 (Microsoft)
    pub s_typo_ascender: Option<i16>,
    pub s_typo_descender: Option<i16>,
    pub s_typo_line_gap: Option<i16>,
    pub us_win_ascent: Option<u16>,
    pub us_win_descent: Option<u16>,
    // Version 1
    pub ul_code_page_range1: Option<u32>,
    pub ul_code_page_range2: Option<u32>,
    // Version 2, 3, 4
    pub sx_height: Option<i16>,
    pub s_cap_height: Option<i16>,
    pub us_default_char: Option<u16>,
    pub us_break_char: Option<u16>,
    pub us_max_context: Option<u16>,
    // Version 5
    pub us_lower_optical_point_size: Option<u16>,
    pub us_upper_optical_point_size: Option<u16>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_OS_2(&mut self, buffer: &mut Buffer) {
        // TODO: make it more elegant.
        let os_2_length = self.get_table_len(*b"OS/2");
        // Version 0
        let mut table = Table_OS_2 {
            _version: buffer.get::<u16>(),
            x_avg_char_width: buffer.get::<i16>(),
            us_weight_class: buffer.get::<u16>(),
            us_width_class: buffer.get::<u16>(),
            fs_type: buffer.get::<u16>(),
            y_subscript_x_size: buffer.get::<i16>(),
            y_subscript_y_size: buffer.get::<i16>(),
            y_subscript_x_offset: buffer.get::<i16>(),
            y_subscript_y_offset: buffer.get::<i16>(),
            y_superscript_x_size: buffer.get::<i16>(),
            y_superscript_y_size: buffer.get::<i16>(),
            y_superscript_x_offset: buffer.get::<i16>(),
            y_superscript_y_offset: buffer.get::<i16>(),
            y_strikeout_size: buffer.get::<i16>(),
            y_strikeout_position: buffer.get::<i16>(),
            s_family_class: buffer.get::<i16>(),
            panose: buffer.get_vec::<u8>(10),
            ul_unicode_range1: buffer.get::<u32>(),
            ul_unicode_range2: buffer.get::<u32>(),
            ul_unicode_range3: buffer.get::<u32>(),
            ul_unicode_range4: buffer.get::<u32>(),
            ach_vend_i_d: buffer.get::<Tag>(),
            fs_selection: buffer.get::<u16>(),
            us_first_char_index: buffer.get::<u16>(),
            us_last_char_index: buffer.get::<u16>(),
            s_typo_ascender: None,
            s_typo_descender: None,
            s_typo_line_gap: None,
            us_win_ascent: None,
            us_win_descent: None,
            ul_code_page_range1: None,
            ul_code_page_range2: None,
            sx_height: None,
            s_cap_height: None,
            us_default_char: None,
            us_break_char: None,
            us_max_context: None,
            us_lower_optical_point_size: None,
            us_upper_optical_point_size: None,
        };
        // Version 0 (Microsoft)
        if os_2_length >= 78 {
            table.s_typo_ascender = Some(buffer.get::<i16>());
            table.s_typo_descender = Some(buffer.get::<i16>());
            table.s_typo_line_gap = Some(buffer.get::<i16>());
            table.us_win_ascent = Some(buffer.get::<u16>());
            table.us_win_descent = Some(buffer.get::<u16>());
        }
        // Version 1
        if table._version >= 1 {
            table.ul_code_page_range1 = Some(buffer.get::<u32>());
            table.ul_code_page_range2 = Some(buffer.get::<u32>());
        }
        // Version 2, 3, 4
        if table._version >= 2 {
            table.sx_height = Some(buffer.get::<i16>());
            table.s_cap_height = Some(buffer.get::<i16>());
            table.us_default_char = Some(buffer.get::<u16>());
            table.us_break_char = Some(buffer.get::<u16>());
            table.us_max_context = Some(buffer.get::<u16>());
        }
        // Version 5
        if table._version >= 5 {
            table.us_lower_optical_point_size = Some(buffer.get::<u16>());
            table.us_upper_optical_point_size = Some(buffer.get::<u16>());
        }
        self.OS_2 = Some(table);
    }
}
