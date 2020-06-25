// use std::fmt;

use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};

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
        let mut glyphs = Vec::new();
        for i in &self.loca.as_ref().unwrap().offsets {
            buffer.offset = start_offset + *i;
            glyphs.push(buffer.get::<Glyph>());
        }
        self.glyf = Some(Table_glyf { glyphs });
    }
}

#[derive(Debug)]
pub struct Glyph {
    number_of_contours: i16,
    x_min: i16,
    y_min: i16,
    x_max: i16,
    y_max: i16,
    instruction_length: u16,
    instructions: Vec<u8>,
    contours: Vec<Vec<Point>>,
    component: Option<Component>,
}

impl ReadBuffer for Glyph {
    fn read(buffer: &mut Buffer) -> Self {
        let number_of_contours = buffer.get::<i16>();
        let x_min = buffer.get::<i16>();
        let y_min = buffer.get::<i16>();
        let x_max = buffer.get::<i16>();
        let y_max = buffer.get::<i16>();

        let mut instruction_length = 0;
        let mut instructions = Vec::new();
        let mut contours = Vec::new();

        let component = None;

        if number_of_contours >= 0 {
            let end_points_of_contours = buffer.get_vec::<u16>(number_of_contours as usize);
            let num_points = *end_points_of_contours.last().unwrap_or(&0) + 1;
            instruction_length = buffer.get::<u16>();
            instructions = buffer.get_vec::<u8>(instruction_length as usize);
            contours = buffer.get_contours(end_points_of_contours, num_points);
        } else {
            // component = Some(buffer.get_component());
        }

        Self {
            number_of_contours,
            x_min,
            y_min,
            x_max,
            y_max,
            instruction_length,
            instructions,
            contours,
            component,
        }
    }
}

impl Buffer {
    fn get_contours(&mut self, end_points_of_contours: Vec<u16>, num_points: u16) -> Vec<Vec<Point>> {
        let flags = self.get_flags(num_points);
        let xs = self.get_coordinates(&flags, X_SAME_POSITIVE_FLAG, X_SHORT_FLAG);
        let ys = self.get_coordinates(&flags, Y_SAME_POSITIVE_FLAG, Y_SHORT_FLAG);

        // TODO: https://docs.rs/itertools/0.9.0/itertools/macro.izip.html
        let mut points = xs.iter()
            .zip(ys.iter())
            .zip(flags.iter())
            .map(|((&x, &y), &flag)| Point {
                x,
                y,
                on_curve: flag & ON_CURVE_FLAG != 0,
                overlap_simple: flag & OVERLAP_SIMPLE_FLAG != 0,
            })
            .collect::<Vec<Point>>();

        let mut contours = Vec::new();
        let mut left_len = 0;
        for i in end_points_of_contours {
            let right = points.split_off(i as usize + 1 - left_len);
            left_len += points.len();
            contours.push(points);
            points = right;
        }

        contours
    }

    fn get_flags(&mut self, num_points: u16) -> Vec<u8> {
        let mut flags = Vec::new();
        let mut i = 0;
        while i < num_points {
            let flag_byte = self.get::<u8>();
            flags.push(flag_byte);
            // Check repeat flag
            if flag_byte & REPEAT_FLAG == 0 {
                i += 1;
            } else {
                let repeated = self.get::<u8>();
                for _ in 0..repeated {
                    flags.push(flag_byte);
                }
                i += repeated as u16 + 1;
            }
        }
        flags
    }

    fn get_coordinates(&mut self, flags: &Vec<u8>, flag1: u8, flag2: u8) -> Vec<i16> {
        let flag3 = flag1 + flag2;
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

#[derive(Debug)]
pub struct Component {
}

#[derive(Debug)]
pub struct Point {
    x: i16,
    y: i16,
    on_curve: bool,
    overlap_simple: bool,
}

const ON_CURVE_FLAG: u8 = 0x01;
const X_SHORT_FLAG: u8 = 0x02;
const Y_SHORT_FLAG: u8 = 0x04;
const REPEAT_FLAG: u8 = 0x08;
const X_SAME_POSITIVE_FLAG: u8 = 0x10;
const Y_SAME_POSITIVE_FLAG: u8 = 0x20;
const OVERLAP_SIMPLE_FLAG: u8 = 0x40;
