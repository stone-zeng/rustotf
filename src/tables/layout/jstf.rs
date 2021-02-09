use crate::font::Font;
use crate::types::Tag;
use crate::util::{Buffer, ReadBuffer};

/// ## `JSTF` &mdash; Justification Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/jstf>.
///
/// The Justification table (`JSTF`) provides font developers with additional control
/// over glyph substitution and positioning in justified text. Text-processing clients
/// now have more options to expand or shrink word and glyph spacing so text fills the
/// specified line length.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_JSTF {
    version: String,
    pub jstf_script_records: Vec<JstfScriptRecord>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_JSTF(&mut self, buffer: &mut Buffer) {
        let jstf_start = buffer.offset();
        let version = buffer.get_version::<u16>();
        let jstf_script_count: u16 = buffer.get();
        let mut jstf_script_records: Vec<JstfScriptRecord> = buffer.get_vec(jstf_script_count);
        jstf_script_records.iter_mut().for_each(|rec| {
            buffer.set_offset_from(jstf_start, rec.jstf_script_offset);
            rec.jstf_script = Some(buffer.get());
        });
        self.JSTF = Some(Table_JSTF {
            version,
            jstf_script_records,
        });
    }
}

#[derive(Debug, Default)]
pub struct JstfScriptRecord {
    pub jstf_script_tag: Tag,
    pub jstf_script: Option<JstfScript>,
    jstf_script_offset: u16,
}

impl ReadBuffer for JstfScriptRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            jstf_script_tag: buffer.get(),
            jstf_script_offset: buffer.get(),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct JstfScript {
    pub extender_glyphs: Vec<u16>,
    pub default_jstf_lang_sys: Option<JstfLangSysRecord>,
    pub jstf_lang_sys_records: Vec<JstfLangSysRecord>,
}

impl ReadBuffer for JstfScript {
    fn read(buffer: &mut Buffer) -> Self {
        let start = buffer.offset();
        let extender_glyphs_offset: u16 = buffer.get();
        let default_jstf_lang_sys_offset: u16 = buffer.get();
        let jstf_lang_sys_count: u16 = buffer.get();
        let mut jstf_lang_sys_records: Vec<JstfLangSysRecord> = buffer.get_vec(jstf_lang_sys_count);

        let extender_glyphs = if extender_glyphs_offset != 0 {
            buffer.set_offset_from(start, extender_glyphs_offset);
            let extender_glyph_count: u16 = buffer.get();
            buffer.get_vec(extender_glyph_count)
        } else {
            Vec::new()
        };

        let default_jstf_lang_sys = if default_jstf_lang_sys_offset != 0 {
            buffer.set_offset_from(start, default_jstf_lang_sys_offset);
            let mut rec: JstfLangSysRecord = buffer.get();
            buffer.set_offset_from(start, rec.jstf_lang_sys_offset);
            rec.jstf_lang_sys = buffer.get();
            Some(rec)
        } else {
            None
        };

        jstf_lang_sys_records.iter_mut().for_each(|rec| {
            buffer.set_offset_from(start, rec.jstf_lang_sys_offset);
            rec.jstf_lang_sys = buffer.get();
        });

        Self {
            extender_glyphs,
            default_jstf_lang_sys,
            jstf_lang_sys_records,
        }
    }
}

#[derive(Debug, Default)]
pub struct JstfLangSysRecord {
    pub jstf_lang_sys_tag: Tag,
    pub jstf_lang_sys: JstfLangSys,
    jstf_lang_sys_offset: u16,
}

impl ReadBuffer for JstfLangSysRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            jstf_lang_sys_tag: buffer.get(),
            jstf_lang_sys_offset: buffer.get(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct JstfLangSys {
    pub jstf_priorities: Vec<JstfPriority>,
}

impl ReadBuffer for JstfLangSys {
    fn read(buffer: &mut Buffer) -> Self {
        let start = buffer.offset();
        let jstf_priority_count: u16 = buffer.get();
        let jstf_priority_offsets: Vec<u16> = buffer.get_vec(jstf_priority_count);
        let jstf_priorities: Vec<JstfPriority> = jstf_priority_offsets
            .iter()
            .map(|&offset| {
                buffer.set_offset_from(start, offset);
                buffer.get()
            })
            .collect();
        Self { jstf_priorities }
    }
}

#[derive(Debug)]
pub struct JstfPriority {
    pub gsub_shrinkage_enable: Option<JstfGsubModList>,
    pub gsub_shrinkage_disable: Option<JstfGsubModList>,
    pub gpos_shrinkage_enable: Option<JstfGposModList>,
    pub gpos_shrinkage_disable: Option<JstfGposModList>,
    pub shrinkage_jstf_max: Option<JstfMax>,
    pub gsub_extension_enable: Option<JstfGsubModList>,
    pub gsub_extension_disable: Option<JstfGsubModList>,
    pub gpos_extension_enable: Option<JstfGposModList>,
    pub gpos_extension_disable: Option<JstfGposModList>,
    pub extension_jstf_max: Option<JstfMax>,
}

impl ReadBuffer for JstfPriority {
    fn read(buffer: &mut Buffer) -> Self {
        let start = buffer.offset();
        let gsub_shrinkage_enable_offset: u16 = buffer.get();
        let gsub_shrinkage_disable_offset: u16 = buffer.get();
        let gpos_shrinkage_enable_offset: u16 = buffer.get();
        let gpos_shrinkage_disable_offset: u16 = buffer.get();
        let shrinkage_jstf_max_offset: u16 = buffer.get();
        let gsub_extension_enable_offset: u16 = buffer.get();
        let gsub_extension_disable_offset: u16 = buffer.get();
        let gpos_extension_enable_offset: u16 = buffer.get();
        let gpos_extension_disable_offset: u16 = buffer.get();
        let extension_jstf_max_offset: u16 = buffer.get();
        Self {
            gsub_shrinkage_enable: buffer.get_or_none(start, gsub_shrinkage_enable_offset),
            gsub_shrinkage_disable: buffer.get_or_none(start, gsub_shrinkage_disable_offset),
            gpos_shrinkage_enable: buffer.get_or_none(start, gpos_shrinkage_enable_offset),
            gpos_shrinkage_disable: buffer.get_or_none(start, gpos_shrinkage_disable_offset),
            shrinkage_jstf_max: buffer.get_or_none(start, shrinkage_jstf_max_offset),
            gsub_extension_enable: buffer.get_or_none(start, gsub_extension_enable_offset),
            gsub_extension_disable: buffer.get_or_none(start, gsub_extension_disable_offset),
            gpos_extension_enable: buffer.get_or_none(start, gpos_extension_enable_offset),
            gpos_extension_disable: buffer.get_or_none(start, gpos_extension_disable_offset),
            extension_jstf_max: buffer.get_or_none(start, extension_jstf_max_offset),
        }
    }
}

#[derive(Debug)]
pub struct JstfGsubModList {
    pub gsub_lookup_indices: Vec<u16>,
}

impl ReadBuffer for JstfGsubModList {
    fn read(buffer: &mut Buffer) -> Self {
        let lookup_count: u16 = buffer.get();
        Self {
            gsub_lookup_indices: buffer.get_vec(lookup_count),
        }
    }
}

#[derive(Debug)]
pub struct JstfGposModList {
    pub gpos_lookup_indices: Vec<u16>,
}

impl ReadBuffer for JstfGposModList {
    fn read(buffer: &mut Buffer) -> Self {
        let lookup_count: u16 = buffer.get();
        Self {
            gpos_lookup_indices: buffer.get_vec(lookup_count),
        }
    }
}

#[derive(Debug)]
pub struct JstfMax {
    // TODO:
    // pub lookups: Vec<Lookup>,
    lookup_offsets: Vec<u16>,
}

impl ReadBuffer for JstfMax {
    fn read(buffer: &mut Buffer) -> Self {
        let lookup_count: u16 = buffer.get();
        Self {
            lookup_offsets: buffer.get_vec(lookup_count),
        }
    }
}
