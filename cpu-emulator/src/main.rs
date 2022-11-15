// const BIAS: i32 = 127;
// const RADIX: f32 = 2.0;

// fn main() {
//     let n: f32 = 42.42;

//     let (sign, exp, frac) = to_parts(n);
//     let (sign_, exp_, mant) = decode(sign, exp, frac);
//     let n_ = from_parts(sign_, exp_, mant);

//     println!("{} -> {}", n, n_);
//     println!("field     |   as bits     | as real number");
//     println!("sign      |   {:01b}      |   {}", sign, sign_);
//     println!("exponent  |   {:08b}      |   {}", exp, exp_);
//     println!("mantissa  |   {:023b}     |   {}", frac, mant);
// }

// fn to_parts(n: f32) -> (u32, u32, u32) {
//     let bits = n.to_bits();

//     let sign = (bits >> 31) & 1;
//     let exponent = (bits >> 23) & 0xff;
//     let fraction = bits & 0x7fffff;

//     (sign, exponent, fraction)
// }

// fn decode(
//     sign: u32,
//     exponent: u32,
//     fraction: u32
// ) -> (f32, f32, f32) {
//     let signed_1 = (-1.0_f32).powf(sign as f32);

//     let exponent = (exponent as i32) - BIAS;
//     let exponent = RADIX.powf(exponent as f32);

//     let mut mantissa: f32 = 1.0;
//     for i in 0..23 {
//         let mask = 1 << i;
//         let one_at_bit_i = fraction & mask;
//         if one_at_bit_i != 0 {
//             let i_ = i as f32;
//             let weight = 2_f32.powf(i_ - 23.0);
//             mantissa += weight;
//         }
//     }

//     (signed_1, exponent, mantissa)
// }

// fn from_parts(
//     sign: f32,
//     exponent: f32,
//     mantissa: f32,
// ) -> f32 {
//     sign * exponent * mantissa
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Q7(i8);

// impl From<f64> for Q7 {
//     fn from (n: f64) -> Self {
//         // assert!(n >= -1.0);
//         // assert!(n <= 1.0);
//         if n >= 1.0 {
//             Q7(127)
//         } else if n <= -1.0 {
//             Q7(-128)
//         } else {
//             Q7((n * 128.0) as i8)
//         }
//     }
// }

// impl From<Q7> for f64 {
//     fn from(n: Q7) -> f64 {
//         (n.0 as f64) * 2_f64.powf(-7.0)
//     }
// }

// impl From<f32> for Q7 {
//     fn from (n: f32) -> Self {
//         Q7::from(n as f64)
//     }
// }

// impl From<Q7> for f32 {
//     fn from(n: Q7) -> f32 {
//         f64::from(n) as f32
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn out_of_bounds() {
//         assert_eq!(Q7::from(10.), Q7::from(1.));
//         assert_eq!(Q7::from(-10.), Q7::from(-1.));
//     }

//     #[test]
//     fn f32_to_q7() {
//         let n1: f32 = 0.7;
//         let q1 = Q7::from(n1);

//         let n2 = -0.4;
//         let q2 = Q7::from(n2);

//         let n3 = 123.0;
//         let q3 = Q7::from(n3);

//         assert_eq!(q1, Q7(89));
//         assert_eq!(q2, Q7(-51));
//         assert_eq!(q3, Q7(127))
//     }

//     #[test]
//     fn q7_to_f32() {
//         let q1 = Q7::from(0.7);
//         let n1 = f32::from(q1);
//         assert_eq!(n1, 0.6953125);

//         let q2 = Q7::from(n1);
//         let n2 = f32::from(q2);
//         assert_eq!(n1, n2);
//     }
// }

// fn mock_rand(n: u8) -> f32 {
//     let base: u32 = 0b0_01111110_00000000000000000000000;
//     let large_n = (n as u32) << 15;
//     let f32_bits = base | large_n;
//     let m = f32::from_bits(f32_bits);
//     2.0 * (m - 0.5)
// }

struct CPU {
    current_operation: u16,
    registers: [u8; 2],
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        // loop {
            let opcode = self.read_opcode();

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        // }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2],
    };

    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);

    println!("5 + 10 = {}", cpu.registers[0]);


    // let opcode: u16 = 0x71E4;

    // let c = (opcode & 0xF000) >> 12;
    // let x = (opcode & 0x0F00) >> 8;
    // let y = (opcode & 0x00F0) >> 4;
    // let d = (opcode & 0x000F) >> 0;

    // assert_eq!(c, 0x7);
    // assert_eq!(x, 0x1);
    // assert_eq!(y, 0xE);
    // assert_eq!(d, 0x4);

    // let nnn = opcode & 0x0FFF;
    // let kk = opcode & 0x00FF;

    // assert_eq!(nnn, 0x1E4);
    // assert_eq!(kk, 0xE4);
}