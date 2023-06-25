pub struct RailFence(u32);

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence(rails)
    }

    pub fn encode(&self, text: &str) -> String {
        let n = self.0 as usize;
        let l = text.len();

        let chars = text.chars();
        let mut output = vec![vec![' '; l]; n];

        let mut v = 1_isize;
        let mut i = 0_isize;

        for (h, c) in chars.enumerate() {
            output[i as usize][h] = c;

            if h > 0 && (i == 0 || (i + 1) % n as isize == 0) {
                v = -v;
            }

            i += v;
        }

        String::from_iter(output.iter().map(String::from_iter)).replace(' ', "")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let n = self.0 as usize;
        let l = cipher.len();

        let chars: Vec<char> = cipher.chars().collect();
        let mut zigzag = vec![vec![' '; l]; n];

        let mut v = -1_isize;
        let mut i = 0_isize;

        for h in 0..l {
            zigzag[i as usize][h] = '*';

            if i == 0 {
                v = 1;
            }

            if (i + 1) % n as isize == 0 {
                v = -1;
            }

            i += v;
        }

        let mut index = 0;

        for i in 0..n {
            for j in 0..l {
                if zigzag[i][j] == '*' && index < l {
                    zigzag[i][j] = chars[index];
                    index += 1;
                }
            }
        }

        let mut output = String::new();

        let mut v = 1_isize;
        let mut i = 0_isize;

        for h in 0..l {
            output.push(zigzag[i as usize][h]);

            if h > 0 && (i == 0 || (i + 1) % n as isize == 0) {
                v = -v;
            }

            i += v;
        }

        output
    }
}

// fn determine_split_factor(l: usize, n: usize) -> usize {
//     let f = 2 * n - 1;

//     let mut l = l;
//     while l % f != 0 {
//         l += 1;
//     }
//     l / f
// }
