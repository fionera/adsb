use std::iter::Iterator;

#[derive(Debug, PartialEq)]
pub enum FrameType {
    ModeAC,

    /// Mode-S short frame consisting of
    /// - 1 byte RSSI
    /// - 7 byte Mode-S short data
    ModeSShort,

    /// Mode-S long frame consisting of
    /// - 1 byte RSSI
    /// - 14 byte Mode-S short data
    ModeSLong,
}

#[derive(Debug, PartialEq)]
pub struct BeastFrame {
    pub frame_type: FrameType,
    pub timestamp: u64,
    pub rssi: u8,
    pub data: Vec<u8>,
}

pub struct BeastDecoder<I: Iterator<Item=u8>> {
    input: I,
}

impl<I: Iterator<Item=u8>> BeastDecoder<I> {
    pub fn new(input: I) -> Self {
        BeastDecoder { input }
    }

    fn read_encoded(&mut self, len: usize) -> Result<Vec<u8>, &'static str> {
        let mut escaped = false;
        let mut data = Vec::with_capacity(len);

        while data.len() < data.capacity() {
            let byte = self.input.next().ok_or("failed to read next byte")?;
            if escaped {
                if byte == 0x1A {
                    data.push(byte);
                } else {
                    return Err("Invalid escape sequence");
                }
                escaped = false;
            } else if byte == 0x1A {
                escaped = true;
            } else {
                data.push(byte);
            }
        }

        if escaped {
            return Err("Unterminated escape sequence");
        }

        Ok(data)
    }
}

impl<I: Iterator<Item=u8>> Iterator for BeastDecoder<I> {
    type Item = BeastFrame;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let byte = self.input.next()?;
            if byte != 0x1a {
                panic!("invalid begin: {:02x}", byte);
            }

            let frame_type_byte = self.input.next().unwrap();
            let frame_type = match frame_type_byte {
                0x31 => FrameType::ModeAC,
                0x32 => FrameType::ModeSShort,
                0x33 => FrameType::ModeSLong,
                0xe3 => {
                    let _receiver_id = self.read_encoded(8).unwrap();
                    //println!("receiver: {}", hex::encode(receiver_id));
                    continue;
                }
                _ => panic!("invalid frametype {:02x}", frame_type_byte),
            };

            let mut timestamp: u64 = 0;
            let timestamp_data = self.read_encoded(6).unwrap();
            for v in timestamp_data {
                timestamp = (timestamp << 8) | (v as u64);
            }

            let rssi = *self.read_encoded(1).unwrap().first().unwrap();

            let data_len = match frame_type {
                FrameType::ModeAC => 2,
                FrameType::ModeSShort => 7,
                FrameType::ModeSLong => 14,
            };

            let data = self.read_encoded(data_len).unwrap();

            return Some(BeastFrame {
                frame_type,
                timestamp,
                rssi,
                data,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    fn assert_decode(input: Vec<u8>, expected: Vec<BeastFrame>) {
        let decoder = BeastDecoder::new(input.into_iter());

        let result: Vec<_> = decoder.collect();

        assert_eq!(result.len(), expected.len(), "length difference result: {} - expected: {}", result.len(), expected.len());

        result.iter()
            .zip(expected.iter())
            .for_each(|(res, exp)| {
                assert_eq!(res, exp)
            });
    }

    #[test]
    fn test_single_mode_s_short() {
        let input = hex!("1a3200064f79a383405d3c48e8acfc49").to_vec();
        let expected = vec![BeastFrame {
            frame_type: FrameType::ModeSShort,
            timestamp: 27103175555,
            rssi: 64,
            data: hex!("5d3c48e8acfc49").to_vec(),
        }];
        assert_decode(input, expected);
    }

    #[test]
    fn test_single_mode_s_long() {
        let input = hex!("1a3300064f783335a58d4009da9909a902d8048bd7e4de").to_vec();
        let expected = vec![BeastFrame {
            frame_type: FrameType::ModeSLong,
            timestamp: 27103081269,
            rssi: 165,
            data: hex!("8d4009da9909a902d8048bd7e4de").to_vec(),
        }];
        assert_decode(input, expected);
    }

    #[test]
    fn test_id_frame_then_mode_s_long() {
        let input = hex!("1ae31347cbefb81a1a8a981a320008ffd55bae742020058b4fc582").to_vec();
        let expected = vec![BeastFrame {
            frame_type: FrameType::ModeSShort,
            timestamp: 38651911086,
            rssi: 116,
            data: hex!("2020058b4fc582").to_vec(),
        }];
        assert_decode(input, expected);
    }

    #[test]
    fn test_unescape_from_doc() {
        let input = vec![0x1a, 0x32, 0x08, 0x3e, 0x27, 0xb6, 0xcb, 0x6a, 0x1a, 0x1a, 0x00, 0xa1, 0x84, 0x1a, 0x1a, 0xc3, 0xb3, 0x1d];

        let expected = vec![BeastFrame {
            frame_type: FrameType::ModeSShort,
            timestamp: 9063047285610,
            rssi: 0x1a,
            data: vec![0x00, 0xa1, 0x84, 0x1a, 0xc3, 0xb3, 0x1d],
        }];
        assert_decode(input, expected);
    }

    #[test]
    fn test_multiple_frames() {
        let input = concat!(
        "1a3300064f783335a58d4009da9909a902d8048bd7e4de",
        "1a3300064f784c23538d4d22c560ab02fa7b8d29c51792",
        "1a3200064f79a383405d3c48e8acfc49",
        );

        let input = hex::decode(input).unwrap().to_vec();

        let expected = vec![BeastFrame {
            frame_type: FrameType::ModeSLong,
            timestamp: 27103081269,
            rssi: 165,
            data: hex!("8d4009da9909a902d8048bd7e4de").to_vec(),
        }, BeastFrame {
            frame_type: FrameType::ModeSLong,
            timestamp: 27103087651,
            rssi: 83,
            data: hex!("8d4d22c560ab02fa7b8d29c51792").to_vec(),
        }, BeastFrame {
            frame_type: FrameType::ModeSShort,
            timestamp: 27103175555,
            rssi: 64,
            data: hex!("5d3c48e8acfc49").to_vec(),
        }, ];
        assert_decode(input, expected);
    }
}