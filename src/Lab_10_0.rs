/* DEBUG
use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}



1) В main вызывается rot.read_to_string(&mut result)
2) Rust видит, что RotDecoder<R> реализует Read, а Read имеет метод read_to_string
3) read_to_string использует read, который был реализован
4) Сначала он читает данные через self.input.read(buf),
5) Затем применяет replace_letter ко всем байтам.

use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}


// Интерфейс RotDecoder на основе std::io::Read
impl <R : Read> Read for RotDecoder<R> {
    // Чтение и замена символов, возвращает размер считанной строки
    // Вызывается в main read_to_string, который вызывает read -
    // а вот для него уже написана частная версия, которую можно видеть ниже :)
    // Такая вот многоходовочка, зато тесты теперь работают
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Чтение символов в buf и подсчёт считанных символов
        let bytes_read_count = self.input.read(buf)?;
        // Проверка по каждому символу в UTF-8
        for cur_byte in &mut buf[..bytes_read_count] {
            // Замена при необходимости в ROT
            *cur_byte = Self::replace_letter(*cur_byte, self.rot);
        }

        // В случае отсутствия ошибок возвращает количество символов
        Ok(bytes_read_count)
    }
}

// Так как функции replace_letter изначально нет в Read, то для неё пишется отдельный интерфейс
impl<R : Read> RotDecoder<R> {
    fn replace_letter(byte_value: u8, rot_value: u8) -> u8 {
        // Больше, чем сдвиг на 26, делать не получится - английский алфавит меньше :)
        let rot = rot_value % 26;
        match byte_value {
            // Малые и большие буквы сдвигаются
            b'a'..=b'z' => b'a' + (byte_value - b'a' + rot) % 26,
            b'A'..=b'Z' => b'A' + (byte_value - b'A' + rot) % 26,
            // Остальные символы не меняются
            _ => byte_value,
        }
    }
}

fn main() {
    let mut rot = RotDecoder {
        input: "Gb trg gb gur bgure fvqr!".as_bytes(),
        rot: 13
    };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }

    #[test]
    fn dop_test() {
        let mut rot = RotDecoder {
            input: "Nafs Rnhmfnqtanhm, efhmynyj qfgtwfytwszd!".as_bytes(),
            rot: 21
        };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "Ivan Michailovich, zachtite laboratornuy!");
    }
}




//fn main() {
//    println!("Hello, world!");
//}

