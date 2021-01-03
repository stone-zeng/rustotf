#[derive(Debug)]
pub struct CharString {
    data: Vec<u8>,
}

impl CharString {
    pub fn from(data: Vec<u8>) -> Self {
        Self { data }
    }

    #[allow(unused_variables)]
    pub fn parse(&mut self, global_subrs: &mut Vec<CharString>, subrs: &mut Vec<CharString>) {}
}

/*
#[derive(Default)]
struct Subrs {
    data: Vec<CharString>,
}

impl fmt::Debug for Subrs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl Subrs {
    fn from(data: Vec<CharString>) -> Self {
        Self { data }
    }

    fn get_mut(&mut self, index: i32) -> &CharString {
        // TODO: we assume CharstringType == 2
        let bias = if self.data.len() < 1240 {
            107
        } else if self.data.len() < 33900 {
            1131
        } else {
            32768
        };
        &self.data[(index + bias) as usize]
    }
}

// #[derive(Debug)]
struct CharString {
    data: Vec<u8>,
    commands: Vec<CharStringCommand>,
}

impl fmt::Debug for CharString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.commands)
    }
}

impl CharString {
    fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            commands: Vec::new(),
        }
    }

    fn parse(&mut self, global_subrs: &mut Subrs, local_subrs: &mut Subrs) {

        println!("{0}{0}{0}{0}{0}{0}{0}{0}", "==========");

        // let mut seq = Vec::new();
        let mut i = 0;

        let mut hint_num = 0;

        let mut number_stack: Vec<CharStringValue> = Vec::new();
        let mut commands: Vec<CharStringCommand> = Vec::new();

        macro_rules! _push_str {
            ($s:literal) => {
                eprintln!($s);
                // self.commands
                //     .push(CharStringValue::Operator($s.to_string()))
            };
        }

        macro_rules! _set_width {
            () => {
                commands.push(CharStringCommand::new(vec![number_stack[0]], CharStringOperator::op_width))
            };
        }

        // TODO: width and hintmask bytes are not considered
        while i < self.data.len() {
            let b0 = self.data[i];
            match b0 {
                // Numbers
                28 => {
                    let b1 = self.data[i + 1] as i16;
                    let b2 = self.data[i + 2] as i16;
                    i += 2;
                    number_stack.push(CharStringValue::Int((b1 << 8 | b2) as i32));
                }
                32..=246 => {
                    let b0 = b0 as i32;
                    number_stack.push(CharStringValue::Int(b0 - 139));
                }
                247..=250 => {
                    let b0 = b0 as i32;
                    let b1 = self.data[i + 1] as i32;
                    i += 1;
                    number_stack.push(CharStringValue::Int((b0 - 247) * 256 + b1 + 108));
                }
                251..=254 => {
                    let b0 = b0 as i32;
                    let b1 = self.data[i + 1] as i32;
                    i += 1;
                    number_stack.push(CharStringValue::Int(-(b0 - 251) * 256 - b1 - 108));
                }
                255 => {
                    let b1 = self.data[i + 1] as i16;
                    let b2 = self.data[i + 2] as i16;
                    let b3 = self.data[i + 3] as u16;
                    let b4 = self.data[i + 4] as u16;
                    i += 4;
                    number_stack.push(CharStringValue::Fixed(b1 << 8 | b2, b3 << 8 | b4));
                }

                // Operators

                21 => {
                    if number_stack.len() == 3 {
                        _set_width!();
                        number_stack = number_stack.split_off(1);
                    }
                    let cmd = CharStringCommand::new(number_stack.clone(), CharStringOperator::op_rmoveto);
                    println!("{:?}", cmd);
                    commands.push(cmd);
                    number_stack.clear();
                }
                22 => {
                    if number_stack.len() == 2 {
                        _set_width!();
                        number_stack = number_stack.split_off(1);
                    }
                    let n = number_stack.pop().unwrap();
                    let cmd = CharStringCommand::new(vec![n], CharStringOperator::op_hmoveto);
                    println!("{:?}", cmd);
                    commands.push(cmd);
                }
                4 => {
                    if number_stack.len() == 2 {
                        _set_width!();
                        number_stack = number_stack.split_off(1);
                    }
                    let n = number_stack.pop().unwrap();
                    let cmd = CharStringCommand::new(vec![n], CharStringOperator::op_vmoveto);
                    println!("{:?}", cmd);
                    commands.push(cmd);
                }

                1 | 3 | 18 | 23 => {
                    if number_stack.len() % 2 == 1 {
                        _set_width!();
                        number_stack = number_stack.split_off(1);
                    }
                    hint_num += number_stack.len() / 2;
                    let cmd = CharStringCommand::new(
                        number_stack.clone(),
                        match b0 {
                            1 => CharStringOperator::op_hstem,
                            3 => CharStringOperator::op_vstem,
                            18 => CharStringOperator::op_hstemhm,
                            23 => CharStringOperator::op_vstemhm,
                            _ => unreachable!(),
                        }
                    );
                    println!("{:?}", cmd);
                    commands.push(cmd);
                    number_stack.clear();
                }

                14 => {
                    if !number_stack.is_empty() {
                        _set_width!();
                        number_stack.clear();
                    }
                    commands.push(CharStringCommand::new(vec![], CharStringOperator::op_endchar));
                }

                19 => {
                    let hint_bytes = (hint_num + number_stack.len() + 7) / 8;
                    let cmd = CharStringCommand {
                        args: number_stack.clone(),
                        operator: CharStringOperator::op_hintmask,
                        mask: (0..hint_bytes).map(|j| self.data[i + j + 1]).collect(),
                    };
                    println!("{:?}", cmd);
                    commands.push(cmd);
                    number_stack.clear();
                    i += hint_bytes;
                    hint_num = 0;
                }
                20 => {
                    let hint_bytes = (hint_num + number_stack.len() + 7) / 8;
                    let cmd = CharStringCommand {
                        args: number_stack.clone(),
                        operator: CharStringOperator::op_cntrmask,
                        mask: (0..hint_bytes).map(|j| self.data[i + j + 1]).collect(),
                    };
                    println!("{:?}", cmd);
                    commands.push(cmd);
                    number_stack.clear();
                    i += hint_bytes;
                    hint_num = 0;
                }


                5 => {
                    commands.push(CharStringCommand::new(number_stack.clone(), CharStringOperator::op_rlineto));
                    number_stack.clear();
                }
                6 => {
                    commands.push(CharStringCommand::new(number_stack.clone(), CharStringOperator::op_hlineto));
                    number_stack.clear();
                }
                7 => {
                    commands.push(CharStringCommand::new(number_stack.clone(), CharStringOperator::op_vlineto));
                    number_stack.clear();
                }
                8 => {
                    commands.push(CharStringCommand::new(number_stack.clone(), CharStringOperator::op_rrcurveto));
                    number_stack.clear();
                }
                27 => {
                    commands.push(CharStringCommand::new(number_stack.clone(), CharStringOperator::op_hhcurveto));
                    number_stack.clear();
                }
                31 => {
                    commands.push(CharStringCommand::new(number_stack.clone(), CharStringOperator::op_hvcurveto));
                    number_stack.clear();
                }
                24 => {
                    commands.push(CharStringCommand::new(number_stack.clone(), CharStringOperator::op_rcurveline));
                    number_stack.clear();
                }
                25 => {
                    commands.push(CharStringCommand::new(number_stack.clone(), CharStringOperator::op_rlinecurve));
                    number_stack.clear();
                }
                30 => {
                    commands.push(CharStringCommand::new(number_stack.clone(), CharStringOperator::op_vhcurveto));
                    number_stack.clear();
                }
                26 => {
                    commands.push(CharStringCommand::new(number_stack.clone(), CharStringOperator::op_vvcurveto));
                    number_stack.clear();
                }

                // 10 => _push_str!("callsubr"),
                10 => {
                    let cmd = CharStringCommand::new(number_stack.clone(), CharStringOperator::op_callsubr);
                    println!("{:?}", cmd);
                    commands.push(cmd);
                    let index = match number_stack.pop().unwrap() {
                        CharStringValue::Int(n) => n,
                        _ => unreachable!(),
                    };
                    let x = local_subrs.get_mut(index);
                    println!("LOCAL_SUBRS: {:?}", x.data);
                    // x.parse(global_subrs, local_subrs);
                }
                // 29 => _push_str!("callgsubr"),
                29 => {
                    let cmd = CharStringCommand::new(number_stack.clone(), CharStringOperator::op_callgsubr);
                    println!("{:?}", cmd);
                    commands.push(cmd);
                    let index = match number_stack.pop().unwrap() {
                        CharStringValue::Int(n) => n,
                        _ => unreachable!(),
                    };
                    let x = global_subrs.get_mut(index);
                    println!("GLOBAL_SUBRS: {:?}", x.data);
                }


                11 => _push_str!("return"),
                12 => {
                    // let b1 = self.data[i + 1];
                    // let op_str = match b1 {
                    //     3 => "and",
                    //     4 => "or",
                    //     5 => "not",
                    //     9 => "abs",
                    //     10 => "add",
                    //     11 => "sub",
                    //     12 => "div",
                    //     14 => "neg",
                    //     15 => "eq",
                    //     18 => "drop",
                    //     20 => "put",
                    //     21 => "get",
                    //     22 => "ifelse",
                    //     23 => "random",
                    //     24 => "mul",
                    //     26 => "sqrt",
                    //     27 => "dup",
                    //     28 => "exch",
                    //     29 => "index",
                    //     30 => "roll",
                    //     34 => "hflex",
                    //     35 => "flex",
                    //     36 => "hflex1",
                    //     37 => "flex1",
                    //     _ => "[TODO] hint_mask_bytes",
                    // };
                    i += 1;
                    // self.commands
                    //     .push(CharStringValue::Operator(op_str.to_string()));
                }
                _ => _push_str!("[TODO] hint_mask_bytes"),
            }
            i += 1;
        }

        self.commands = commands;
    }
}

struct CharStringCommand {
    args: Vec<CharStringValue>,
    operator: CharStringOperator,
    mask: Vec<u8>,
}

impl fmt::Debug for CharStringCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.mask.is_empty() {
            write!(f, "{:?} \"{:?}\"", self.args, self.operator)
        } else {
            let hintmask_str = self.mask
                .iter()
                .map(|i| format!("{:08b}", i))
                .collect::<Vec<String>>()
                .join("_");
            write!(f, "{:?} \"{:?}\" {}", self.args, self.operator, hintmask_str)
        }
    }
}

impl CharStringCommand {
    fn new(args: Vec<CharStringValue>, operator: CharStringOperator) -> Self {
        Self {
            args,
            operator,
            mask: Vec::new()
        }
    }
}

// FIXME:
#[derive(Clone, Copy)]
enum CharStringValue {
    Int(i32),
    Fixed(i16, u16),
}

impl fmt::Debug for CharStringValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(n) => write!(f, "{}", n),
            Self::Fixed(i, u) => write!(f, "{}", *i as f64 + *u as f64 / 65536.0),
        }
    }
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug)]
enum CharStringOperator {
    // One-byte operators
    op_hstem, // = 0x01,
    op_vstem, // = 0x03,
    op_vmoveto, // = 0x04,
    op_rlineto, // = 0x05,
    op_hlineto, // = 0x06,
    op_vlineto, // = 0x07,
    op_rrcurveto, // = 0x08,
    op_callsubr, // = 0x0a,
    op_return, // = 0x0b,
    // escape = 0x0c
    op_endchar, // = 0x0d,
    op_hstemhm, // = 0x12,
    op_hintmask, // = 0x13,
    op_cntrmask, // = 0x14,
    op_rmoveto, // = 0x15,
    op_hmoveto, // = 0x16,
    op_vstemhm, // = 0x17,
    op_rcurveline, // = 0x18,
    op_rlinecurve, // = 0x19,
    op_vvcurveto, // = 0x1a,
    op_hhcurveto, // = 0x1b,
    op_callgsubr, // = 0x1d,
    op_vhcurveto, // = 0x1e,
    op_hvcurveto, // = 0x1f,
    // Two-byte operators
    op_and, // = 0x0c_03,
    op_or, // = 0x0c_04,
    op_not, // = 0x0c_05,
    op_abs, // = 0x0c_09,
    op_add, // = 0x0c_0a,
    op_sub, // = 0x0c_0b,
    op_div, // = 0x0c_0c,
    op_neg, // = 0x0c_0e,
    op_eq, // = 0x0c_0f,
    op_drop, // = 0x0c_12,
    op_put, // = 0x0c_14,
    op_get, // = 0x0c_15,
    op_ifelse, // = 0x01_6c,
    op_random, // = 0x0c_17,
    op_mul, // = 0x0c_18,
    op_sqrt, // = 0x0c_1a,
    op_dup, // = 0x0c_1b,
    op_exch, // = 0x0c_1c,
    op_index, // = 0x0c_1d,
    op_roll, // = 0x0c_1e,
    op_hflex, // = 0x0c_22,
    op_flex, // = 0x0c_23,
    op_hflex1, // = 0x0c_24,
    op_flex1, // = 0x0c_25,
    //
    op_width,
}
*/
