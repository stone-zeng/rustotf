use std::usize;

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
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_GSUB(&mut self, buffer: &mut Buffer) {
        let gsub_start_offset = buffer.offset;
        let _version = buffer.get_version::<u16>();
        let script_list_offset: u16 = buffer.get();
        let feature_list_offset: u16 = buffer.get();
        let lookup_list_offset: u16 = buffer.get();
        let feature_variations_offset: Option<u32> = if _version == "1.1" {
            Some(buffer.get())
        } else {
            None
        };

        let script_list_start_offset = gsub_start_offset + script_list_offset as usize;
        buffer.offset = script_list_start_offset;
        let num_scripts: u16 = buffer.get();
        let mut script_list: Vec<ScriptRecord> = buffer.get_vec(num_scripts as usize);
        script_list.iter_mut().for_each(|rec| {
            let script_start_offset = script_list_start_offset + rec.script_offset as usize;
            buffer.offset = script_start_offset;
            rec.script = buffer.get();
        });

        println!("feature_list_offset       = {}", feature_list_offset);
        println!("lookup_list_offset        = {}", lookup_list_offset);
        println!("feature_variations_offset = {:?}", feature_variations_offset);

        self.GSUB = Some(Table_GSUB {
            _version,
            script_list,
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

        let default_lang_sys = match default_lang_sys_offset {
            0 => None,
            _ => {
                buffer.offset = script_start_offset + default_lang_sys_offset as usize;
                Some(buffer.get())
            }
        };
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
        let feature_index_count = buffer.get::<u16>() as usize;
        let feature_indices = buffer.get_vec(feature_index_count);
        Self {
            required_feature_index,
            feature_indices,
        }
    }
}
