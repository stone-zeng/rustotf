use crate::font::Font;
use crate::util::{Buffer, ReadBuffer};

/// ## `DSIG` &mdash; Digital Signature Table
///
/// Specification: <https://docs.microsoft.com/en-us/typography/opentype/spec/dsig>.
///
/// The `DSIG` table contains the digital signature of the OpenType font. Signature formats
/// are widely documented and rely on a key pair architecture. Software developers, or publishers
/// posting material on the Internet, create signatures using a private key. Operating systems
/// or applications authenticate the signature using a public key.

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct Table_DSIG {
    version: u32,
    pub num_signatures: u16,
    pub flags: u16,
    pub signature_records: Vec<SignatureRecord>,
}

impl Font {
    #[allow(non_snake_case)]
    pub fn parse_DSIG(&mut self, buffer: &mut Buffer) {
        let dsig_start = buffer.offset();
        let version = buffer.get();
        let num_signatures = buffer.get();
        let flags = buffer.get();
        let mut signature_records: Vec<SignatureRecord> = buffer.get_vec(num_signatures);
        signature_records
            .iter_mut()
            .for_each(|rec| match rec.format {
                1 => {
                    buffer.set_offset_from(dsig_start, rec.signature_block_offset);
                    buffer.skip::<u16>(2);
                    let signature_length: u32 = buffer.get();
                    rec.signature = buffer.get_vec(signature_length);
                }
                _ => unreachable!(),
            });
        self.DSIG = Some(Table_DSIG {
            version,
            num_signatures,
            flags,
            signature_records,
        });
    }
}

#[derive(Debug, Default)]
pub struct SignatureRecord {
    pub format: u32,
    pub length: u32,
    pub signature: Vec<u8>,
    signature_block_offset: u32,
}

impl ReadBuffer for SignatureRecord {
    fn read(buffer: &mut Buffer) -> Self {
        Self {
            format: buffer.get(),
            length: buffer.get(),
            signature_block_offset: buffer.get(),
            ..Default::default()
        }
    }
}
