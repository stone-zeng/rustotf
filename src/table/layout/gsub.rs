use crate::font::Font;
use crate::util::{Buffer, ReadBuffer, Tag};
use read_buffer_derive::ReadBuffer;

/// ## `GSUB` &mdash; Glyph Substitution Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/gsub>.
///
/// The Glyph Substitution (`GSUB`) table provides data for substition of glyphs for
/// appropriate rendering of scripts, such as cursively-connecting forms in Arabic script,
/// or for advanced typographic effects, such as ligatures.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_GSUB {
    _version: String,
    pub script_list: Vec<ScriptRecord>,
    pub feature_list: Vec<FeatureRecord>,
    pub lookup_list: Vec<Lookup>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_GSUB(&mut self, buffer: &mut Buffer) {
        let gsub_start_offset = buffer.offset;
        let _version = buffer.get_version::<u16>();
        let script_list_offset: u16 = buffer.get();
        let feature_list_offset: u16 = buffer.get();
        let lookup_list_offset: u16 = buffer.get();
        // TODO:
        let _feature_variations_offset: Option<u32> = if _version == "1.1" {
            Some(buffer.get())
        } else {
            None
        };

        let script_list_start_offset = gsub_start_offset + script_list_offset as usize;
        buffer.offset = script_list_start_offset;
        let num_scripts: u16 = buffer.get();
        let mut script_list: Vec<ScriptRecord> = buffer.get_vec(num_scripts as usize);
        script_list.iter_mut().for_each(|rec| {
            buffer.offset = script_list_start_offset + rec.script_offset as usize;
            rec.script = buffer.get();
        });

        let feature_list_start_offset = gsub_start_offset + feature_list_offset as usize;
        buffer.offset = feature_list_start_offset;
        let num_features: u16 = buffer.get();
        let mut feature_list: Vec<FeatureRecord> = buffer.get_vec(num_features as usize);
        feature_list.iter_mut().for_each(|rec| {
            buffer.offset = feature_list_start_offset + rec.feature_offset as usize;
            rec.feature = buffer.get();
        });

        let lookup_list_start_offset = gsub_start_offset + lookup_list_offset as usize;
        buffer.offset = lookup_list_start_offset;
        let num_lookups: u16 = buffer.get();
        let lookup_offsets: Vec<u16> = buffer.get_vec(num_lookups as usize);
        let lookup_list = lookup_offsets
            .iter()
            .map(|&offset| {
                buffer.offset = lookup_list_start_offset + offset as usize;
                buffer.get()
            })
            .collect();

        self.GSUB = Some(Table_GSUB {
            _version,
            script_list,
            feature_list,
            lookup_list,
        });
    }
}

#[derive(Debug, Default)]
pub struct ScriptRecord {
    pub script_tag: Tag,
    pub script: Script,
    script_offset: u16,
}

impl ReadBuffer for ScriptRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            script_tag: buffer.get(),
            script_offset: buffer.get(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Script {
    pub default_lang_sys: Option<LangSys>,
    pub lang_sys: Vec<(Tag, LangSys)>,
}

impl ReadBuffer for Script {
    fn read(buffer: &mut Buffer) -> Self {
        let script_start_offset = buffer.offset;
        let default_lang_sys_offset: u16 = buffer.get();
        let lang_sys_count: u16 = buffer.get();
        let lang_sys_records: Vec<LangSysRecord> = buffer.get_vec(lang_sys_count as usize);
        let default_lang_sys =
            buffer.get_or_none(script_start_offset, default_lang_sys_offset as usize);
        let lang_sys = lang_sys_records
            .iter()
            .map(|rec| {
                buffer.offset = script_start_offset + rec.lang_sys_offset as usize;
                // TODO: simplify
                let tag = Tag::from(rec.lang_sys_tag.to_str());
                (tag, buffer.get())
            })
            .collect();
        Self {
            default_lang_sys,
            lang_sys,
        }
    }
}

#[derive(ReadBuffer)]
struct LangSysRecord {
    lang_sys_tag: Tag,
    lang_sys_offset: u16,
}

#[derive(Debug)]
pub struct LangSys {
    pub required_feature_index: u16,
    pub feature_indices: Vec<u16>,
}

impl ReadBuffer for LangSys {
    fn read(buffer: &mut Buffer) -> Self {
        buffer.skip::<u16>(1); // lookupOrderOffset = NULL
        let required_feature_index = buffer.get();
        let feature_index_count: u16 = buffer.get();
        let feature_indices = buffer.get_vec(feature_index_count as usize);
        Self {
            required_feature_index,
            feature_indices,
        }
    }
}

#[derive(Debug, Default)]
pub struct FeatureRecord {
    pub feature_tag: Tag,
    pub feature: Feature,
    feature_offset: u16,
}

impl ReadBuffer for FeatureRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            feature_tag: buffer.get(),
            feature_offset: buffer.get(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Feature {
    pub feature_params_offset: u16,
    pub lookup_list_indices: Vec<u16>,
}

impl ReadBuffer for Feature {
    fn read(buffer: &mut Buffer) -> Self {
        let feature_params_offset = buffer.get();
        let lookup_index_count: u16 = buffer.get();
        let lookup_list_indices = buffer.get_vec(lookup_index_count as usize);
        Self {
            feature_params_offset,
            lookup_list_indices,
        }
    }
}

#[derive(Debug, Default)]
pub struct Lookup {
    pub lookup_type: u16,
    pub lookup_flag: u16,
    subtable_count: u16,
    subtable_offsets: Vec<u16>,
    pub mark_filtering_set: u16,
}

impl ReadBuffer for Lookup {
    fn read(buffer: &mut Buffer) -> Self {
        let lookup_type = buffer.get();
        let lookup_flag = buffer.get();
        let subtable_count = buffer.get();
        let subtable_offsets = buffer.get_vec(subtable_count as usize);
        let mark_filtering_set = buffer.get();
        Self {
            lookup_type,
            lookup_flag,
            subtable_count,
            subtable_offsets,
            mark_filtering_set,
        }
    }
}
