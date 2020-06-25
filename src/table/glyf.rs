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
    contour: Option<Contour>,
    component: Option<Component>,
}

impl ReadBuffer for Glyph {
    fn read(buffer: &mut Buffer) -> Self {
        let number_of_contours = buffer.get::<i16>();
        let x_min = buffer.get::<i16>();
        let y_min = buffer.get::<i16>();
        let x_max = buffer.get::<i16>();
        let y_max = buffer.get::<i16>();

        let mut contour = None;
        let component = None;
        if number_of_contours >= 0 {
            contour = Some(buffer.get_contour(number_of_contours));
        }

        Self {
            number_of_contours,
            x_min,
            y_min,
            x_max,
            y_max,
            contour,
            component,
        }
    }
}

impl Buffer {
    fn get_contour(&mut self, number_of_contours: i16) -> Contour {
        let end_pts_of_contours = self.get_vec::<u16>(number_of_contours as usize);
        let num_points = *end_pts_of_contours.last().unwrap_or(&0) + 1;
        let instruction_length = self.get::<u16>();
        let instructions = self.get_vec::<u8>(instruction_length as usize);

        let flags = self.get_flags(num_points);
        let x_coordinates = self.get_coordinates(&flags, 0x10, 0x02);
        let y_coordinates = self.get_coordinates(&flags, 0x20, 0x04);

        let mut points = Vec::new();

        // TODO: https://docs.rs/itertools/0.9.0/itertools/macro.izip.html
        for ((&x, &y), &flag) in x_coordinates.iter().zip(y_coordinates.iter()).zip(flags.iter()) {
            points.push(Point {
                x,
                y,
                on_curve: flag & 1 != 0,
                overlap_simple: flag & 0x40 != 0,
            });
        }

        let mut contours: Vec<Vec<Point>> = Vec::new();
        let mut left_len = 0;
        for i in end_pts_of_contours {
            let right = points.split_off(i as usize + 1 - left_len);
            left_len += points.len();
            contours.push(points);
            points = right;
        }

        Contour {
            instruction_length,
            instructions,
            contours,
        }
    }

    fn get_flags(&mut self, num_points: u16) -> Vec<u8> {
        let mut flags = Vec::new();
        let mut i = 0;
        while i < num_points {
            let flag_byte = self.get::<u8>();
            flags.push(flag_byte);
            // Check repeat flag
            if flag_byte & 0x08 == 0 {
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
        let mut coordinates = Vec::new();
        let flag3 = flag1 + flag2;
        for flag in flags {
            let delta = match flag & flag3 {
                0               => self.get::<i16>(),
                n if n == flag1 => 0,
                n if n == flag2 => -(self.get::<u8>() as i16),
                n if n == flag3 => self.get::<u8>() as i16,
                _               => unreachable!(),
            };
            coordinates.push(delta);
        }
        // Accumulate
        coordinates.iter()
                   .scan(0, |acc, &x| {
                       *acc = *acc + x;
                       Some(*acc)
                   })
                   .collect()
    }
}

#[derive(Debug)]
pub struct Contour {
    instruction_length: u16,
    instructions: Vec<u8>,
    contours: Vec<Vec<Point>>,
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
