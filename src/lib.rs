#![no_std]

use core::slice::Iter;
use heapless::Vec;

pub enum EncodeError {
    NotAsciiString,
}

const MORSE_CODE_ALPHABET: [&[u8]; 26] = [
    b".-",   // A
    b"-...", // B
    b"-.-.", // C
    b"-..",  // D
    b".",    // E
    b"..-.", // F
    b"--.",  // G
    b"....", // H
    b"..",   // I
    b".---", // J
    b"-.-",  // K
    b".-..", // L
    b"--",   // M
    b"-.",   // N
    b"---",  // O
    b".--.", // P
    b"--.-", // Q
    b".-.",  // R
    b"...",  // S
    b"-",    // T
    b"..-",  // U
    b"...-", // V
    b".--",  // W
    b"-..-", // X
    b"-.--", // Y
    b"--..", // Z
];

pub struct MorseCodeDecoder {
    unit_of_time: usize,
    code: Vec<u8, 4>,
    is_active: bool,
}

#[derive(Debug)]
pub enum ToggleOutput {
    Done(u8),
    Char(u8),
}

impl MorseCodeDecoder {
    pub fn new(unit_of_time: usize) -> Self {
        Self {
            unit_of_time,
            code: Vec::new(),
            is_active: false,
        }
    }
    pub fn toggle(&mut self, delta_time: usize) -> Option<ToggleOutput> {
        let units = delta_time as f32 / self.unit_of_time as f32;
        let result = if self.is_active {
            // deactivate
            // 按了 units 个单位的时间，现在松开了
            if units < 2.0 {
                // Dit
                _ = self.code.push(b'.');
            } else if units > 2.0 && units < 4.0 {
                // Dah
                _ = self.code.push(b'-');
            }
            None
        } else {
            // activate
            // 松开了 units 个单位的时间，再次按下
            if units > 2.5 {
                // 字符结束
                // 寻找匹配的字符
                let ch = MORSE_CODE_ALPHABET
                    .into_iter()
                    .position(|seq| seq == &self.code);
                self.code.clear();
                let ch = if let Some(ch) = ch {
                    ch as u8 + 0x41
                } else {
                    b'?'
                };
                if units > 8.0 {
                    // 句子结束
                    Some(ToggleOutput::Done(ch))
                } else {
                    Some(ToggleOutput::Char(ch))
                }
            } else {
                None
            }
        };
        self.is_active = !self.is_active;
        result
    }
}

pub struct Current {
    idx_in_alphabet: u8,
    idx_in_code: u8,
}

pub struct MorseCodeEncoder<'a> {
    text: Iter<'a, u8>,
    current: Option<Current>,
    rest: u8, // 下次需要取消激活多长时间
}

#[derive(Debug)]
pub enum MorseCodeAction {
    Activate(u8),
    Deactivate(u8),
}

impl<'a> MorseCodeEncoder<'a> {
    pub fn new(text: Iter<'a, u8>) -> Self {
        Self {
            text,
            current: None,
            rest: 0,
        }
    }
}

impl<'a> Iterator for MorseCodeEncoder<'a> {
    type Item = MorseCodeAction;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.current.as_ref() {
            if (cur.idx_in_code as usize) >= MORSE_CODE_ALPHABET[cur.idx_in_alphabet as usize].len()
            {
                self.current = None;
                self.rest = 0;
                // 字符间停顿 3 个单位时间。
                return Some(MorseCodeAction::Deactivate(3));
            }
        }

        if self.rest > 0 {
            let rest = self.rest;
            self.rest = 0;
            return Some(MorseCodeAction::Deactivate(rest));
        }

        // 这个 while 循环确保 self.current 下标总是有效。它会尝试寻找有效字符，给 self.current 设置正确的下标。如果找不到它会直接返回 None
        while self.current.is_none() {
            let c = if let Some(&c) = self.text.next() {
                c
            } else {
                return None;
            };

            let idx = if c > 0x41 && c < 0x41 + 26 {
                // 大写字母 A-Z
                (c - 0x41) as usize
            } else if c > 0x61 && c < 0x61 + 26 {
                // 小写字母 a-z
                (c - 0x61) as usize
            } else {
                match c {
                    b' ' => {
                        self.current = None;
                        return Some(MorseCodeAction::Deactivate(7)); // 单词间的空格，停顿 7 个单位时间。
                    }
                    _ => {
                        continue;
                    }
                }
            };

            self.current = Some(Current {
                idx_in_alphabet: idx as u8,
                idx_in_code: 0,
            });
        }

        // cur 的下标总是有效的。
        let cur = self.current.as_mut().unwrap();
        let is_dit =
            MORSE_CODE_ALPHABET[cur.idx_in_alphabet as usize][cur.idx_in_code as usize] == b'.';
        cur.idx_in_code += 1;

        self.rest = 1; // Dit 和 Dash 之后需要停顿 1 个单位时间。

        Some(if is_dit {
            MorseCodeAction::Activate(1)
        } else {
            MorseCodeAction::Activate(3)
        })
    }
}
