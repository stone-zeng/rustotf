// use std::fmt;

use crate::font::Font;
use crate::util::{Buffer, F2Dot14, ReadBuffer};

/// ## `glyf` &mdash; Glyph Data
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/glyf>.
///
/// This table contains information that describes the glyphs in the font in the TrueType outline
/// format. Information regarding the rasterizer (scaler) refers to the TrueType rasterizer.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_glyf {
    pub glyphs: Vec<Glyph>,
}

impl Font {
    pub fn parse_glyf(&mut self, buffer: &mut Buffer) {
        let start_offset = buffer.offset;
        self.glyf = Some(Table_glyf {
            glyphs: self.loca.as_ref().unwrap().offsets.iter().map(|i| {
                buffer.offset = start_offset + i;
                buffer.get()
            })
            .collect()
        });
    }
}

#[derive(Debug, Default)]
pub struct Glyph {
    number_of_contours: i16,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    contours: Vec<Vec<Point>>,
    components: Vec<Component>,
    instruction_length: u16,
    instructions: Vec<u8>,
}

impl ReadBuffer for Glyph {
    fn read(buffer: &mut Buffer) -> Self {
        let number_of_contours = buffer.get();

        let mut glyph = Glyph {
            x_min: buffer.get(),
            y_min: buffer.get(),
            x_max: buffer.get(),
            y_max: buffer.get(),
            ..Default::default()
        };

        if number_of_contours >= 0 {
            glyph.parse_simple_glyph(buffer, number_of_contours);
        } else {
            glyph.parse_composite_glyph(buffer);
        }
        glyph
    }
}

impl Glyph {
    const ON_CURVE: u8 = 0x01;
    const X_SHORT: u8 = 0x02;
    const Y_SHORT: u8 = 0x04;
    const REPEAT: u8 = 0x08;
    const X_SAME_POSITIVE: u8 = 0x10;
    const Y_SAME_POSITIVE: u8 = 0x20;
    const OVERLAP_SIMPLE: u8 = 0x40;

    const ARG_1_AND_2_ARE_WORDS: u16 = 0x0001;
    const ARGS_ARE_XY_VALUES: u16 = 0x0002;
    const ROUND_XY_TO_GRID: u16 = 0x0004;
    const WE_HAVE_A_SCALE: u16 = 0x0008;
    const MORE_COMPONENTS: u16 = 0x0020;
    const WE_HAVE_AN_X_AND_Y_SCALE: u16 = 0x0040;
    const WE_HAVE_A_TWO_BY_TWO: u16 = 0x0080;
    const WE_HAVE_INSTRUCTIONS: u16 = 0x0100;
    const USE_MY_METRICS: u16 = 0x0200;
    const OVERLAP_COMPOUND: u16 = 0x0400;
    // TODO: not used
    // const SCALED_COMPONENT_OFFSET: u16 = 0x0800;
    // const UNSCALED_COMPONENT_OFFSET: u16 = 0x1000;

    fn parse_simple_glyph(&mut self, buffer: &mut Buffer, number_of_contours: i16) {
        let end_points_of_contours = buffer.get_vec(number_of_contours as usize);
        self.instruction_length = buffer.get();
        self.instructions = buffer.get_vec(self.instruction_length as usize);
        self.parse_contours(buffer, end_points_of_contours);
    }

    fn parse_contours(&mut self, buffer: &mut Buffer, end_points_of_contours: Vec<u16>) {
        let num_points = *end_points_of_contours.last().unwrap_or(&0) + 1;

        let flags = buffer.get_flags(num_points);
        let xs = buffer.get_coordinates(&flags, Self::X_SAME_POSITIVE, Self::X_SHORT);
        let ys = buffer.get_coordinates(&flags, Self::Y_SAME_POSITIVE, Self::Y_SHORT);

        // TODO: https://docs.rs/itertools/0.9.0/itertools/macro.izip.html
        let mut points = xs.iter()
            .zip(ys.iter())
            .zip(flags.iter())
            .map(|((&x, &y), &flag)| Point {
                x,
                y,
                on_curve: flag & Self::ON_CURVE != 0,
                overlap_simple: flag & Self::OVERLAP_SIMPLE != 0,
            })
            .collect::<Vec<Point>>();

        let mut left_len = 0;
        for i in end_points_of_contours {
            let right = points.split_off(i as usize + 1 - left_len);
            left_len += points.len();
            self.contours.push(points);
            points = right;
        }
    }

    fn parse_composite_glyph(&mut self, buffer: &mut Buffer) {
        let mut flags = 0xFFFF;

        while flags & Self::MORE_COMPONENTS != 0 {
            let mut comp: Component = Default::default();

            flags = buffer.get();
            comp.glyph_index = buffer.get();

            // Offsets and anchors
            if flags & Self::ARGS_ARE_XY_VALUES != 0 {
                // Arguments are signed xy value
                if flags & Self::ARG_1_AND_2_ARE_WORDS != 0 {
                    comp.x = buffer.get();
                    comp.y = buffer.get();
                } else {
                    comp.x = buffer.get::<i8>() as i16;
                    comp.y = buffer.get::<i8>() as i16;
                };
            } else {
                // Arguments are unsigned point numbers
                // TODO: not used
                let (outer, inner) = if flags & Self::ARG_1_AND_2_ARE_WORDS != 0 {
                    (buffer.get(), buffer.get())
                } else {
                    (buffer.get::<u8>() as u16, buffer.get::<u8>() as u16)
                };
                println!("[DEBUG] (outer, inner) = {:?}", (outer, inner));
            }

            // Scale
            // TODO: scale matrix is not initialized
            if flags & Self::WE_HAVE_A_SCALE != 0 {
                (comp.scale.0).0 = buffer.get();
                (comp.scale.1).1 = (comp.scale.0).0;
            } else if flags & Self::WE_HAVE_AN_X_AND_Y_SCALE != 0 {
                (comp.scale.0).0 = buffer.get();
                (comp.scale.1).1 = buffer.get();
            } else if flags & Self::WE_HAVE_A_TWO_BY_TWO != 0 {
                (comp.scale.0).0 = buffer.get();
                (comp.scale.0).1 = buffer.get();
                (comp.scale.1).0 = buffer.get();
                (comp.scale.1).1 = buffer.get();
            }

            // Flags
            comp.round_xy_to_grid = flags % Self::ROUND_XY_TO_GRID != 0;
            comp.use_my_metrics = flags & Self::USE_MY_METRICS != 0;
            comp.overlap_compound = flags & Self::OVERLAP_COMPOUND != 0;

            self.components.push(comp);
        }

        self.parse_instructions(buffer, flags);
    }

    fn parse_instructions(&mut self, buffer: &mut Buffer, flags: u16) {
        if flags & Self::WE_HAVE_INSTRUCTIONS != 0 {
            self.instruction_length = buffer.get();
            self.instructions = buffer.get_vec(self.instruction_length as usize);
        }
    }
}

#[derive(Debug, Default)]
pub struct Point {
    x: i16,
    y: i16,
    on_curve: bool,
    overlap_simple: bool,
}

#[derive(Debug, Default)]
pub struct Component {
    glyph_index: u16,
    x: i16,
    y: i16,
    round_xy_to_grid: bool,
    use_my_metrics: bool,
    overlap_compound: bool,
    scale: ((F2Dot14, F2Dot14), (F2Dot14, F2Dot14)),
}

impl Buffer {
    fn get_flags(&mut self, num_points: u16) -> Vec<u8> {
        let mut flags_vec = Vec::new();
        let mut i = 0;
        while i < num_points {
            let flags = self.get::<u8>();
            flags_vec.push(flags);
            // Check repeat flag
            if flags & Glyph::REPEAT == 0 {
                i += 1;
            } else {
                let repeated = self.get::<u8>();
                for _ in 0..repeated {
                    flags_vec.push(flags);
                }
                i += repeated as u16 + 1;
            }
        }
        flags_vec
    }

    fn get_coordinates(&mut self, flags: &Vec<u8>, flag1: u8, flag2: u8) -> Vec<i16> {
        let flag3 = flag1 | flag2;
        flags.iter().map(|flag| match flag & flag3 {
                0 => self.get::<i16>(),
                n if n == flag1 => 0,
                n if n == flag2 => -(self.get::<u8>() as i16),
                n if n == flag3 => self.get::<u8>() as i16,
                _=> unreachable!(),
            })
            .scan(0, |acc, x| { *acc = *acc + x; Some(*acc) })  // Accumulate
            .collect()
    }
}
