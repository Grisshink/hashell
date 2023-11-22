use std::collections::vec_deque::VecDeque;

mod consts;
use crate::consts::*;

type EncodedHash = String;

pub struct Hashell {
    out_len: u32,
}

impl Hashell {
    pub fn new(hash_length: u32) -> Self {
        Hashell {
            out_len: hash_length,
        }
    }

    pub fn digest(&self, input: &str) -> String {
        if input.len() == 0 {
            panic!("Function cannot be computed for empty strings!");
        }

        self.hash_number(self.hash_encode(input))
    }

    fn hash_encode(&self, inp: &str) -> EncodedHash {
        let mut next_iter = inp.chars().cycle().skip(2);

        inp.chars()
            .enumerate()
            .fold(String::new(), |acc, (i, letter)| {
                let letter_number = find_in_alphabet(letter).expect("Unknown symbols in input!");
                let letter_next =
                    find_in_alphabet(next_iter.next().unwrap()).expect("Unknown symbols in input!");

                acc + ((letter_number
                    + (BIG_PRIME
                        * ((ALF_PRIME + 17 * (i as u32 + 1)) % (i as u32 + 1 + letter_next))))
                    % (MAX_CODE - 1))
                    .to_string()
                    .as_str()
            })
    }

    fn hash_number(&self, inp: EncodedHash) -> String {
        let mut hash_history = VecDeque::new();
        //let mut test = vec![];
        let mut hash = String::with_capacity(self.out_len as usize);
        let parsed_inp = inp.parse::<f64>().unwrap();

        let mut pair_left = inp.chars().cycle().skip(2).step_by(6);
        let mut pair_right = inp.chars().cycle().skip(1).step_by(6);

        for i in 0..(inp.len() as u32 + 1) / 2 {
            let val_left = pair_left.next().unwrap().to_digit(10).unwrap();
            let val_right = pair_right.next().unwrap().to_digit(10).unwrap();

            let pair_val = val_left * 10 + val_right;

            hash_history.push_back(
                ((pair_val
                    + (PRIMELIST
                        .get((val_left as usize).wrapping_sub(1))
                        .unwrap_or(&0)
                        * i
                        * 3))
                    % ALF_PRIME)
                    + 1,
            );
        }

        for _ in 0..((self.out_len + 2000) as f64 / hash_history.len() as f64)
            .log2()
            .ceil() as u32
        //for _ in 0..(((self.out_len + 2000) as f64 / hash_history.len() as f64).log10() / 0.3010299956639812).ceil() as u32
        {
            let mut h = vec_get(&hash_history, (hash_history.len() + 1) / 2 - 1);

            for _ in 0..hash_history.len() {
                h = 1 + ((h + BIG_PRIME) % hash_history.len() as u32);

                let hash_val = vec_get(&hash_history, h as usize - 1);

                let v = (primelist_get(((h % (10 + self.out_len)) + 19) as usize) * hash_val)
                    % inp.len() as u32;

                let inp_val = inp
                    .chars()
                    .skip(v as usize)
                    .next()
                    .unwrap_or('0')
                    .to_digit(10)
                    .unwrap() as f64;

                let prime_val = primelist_get(h as usize % PRIMELIST.len());

                let calc_val = CALC_VARS
                    .get(((hash_val % 31) + (h % 10) * 31) as usize)
                    .unwrap_or(&0.0);

                let final_val = 1
                    + ((inp_val
                        + (hash_val as f64
                            + (vec_get(
                                &hash_history,
                                (prime_val + hash_val) as usize % hash_history.len(),
                            ) as f64
                                + calc_val)))
                        % 241.0) as u32;

                hash_history.push_back(final_val);
            }
        }

        let mut summ = self.out_len;
        for _ in 0..hash_history.len() - (self.out_len as usize + 1000) {
            summ = (summ + hash_history.pop_front().unwrap_or(0)) % 3011;
        }

        for _ in 0..hash_history.len() - (self.out_len as usize + 500) {
            let counter = ((PRIMELIST.last().unwrap() % 11) + summ) as usize % hash_history.len();
            summ = (summ + hash_history.remove(counter).unwrap_or(0)) % 3011;
        }

        let mut h = vec_get(&hash_history, (hash_history.len() + 1) / 2 - 1);
        for _ in 0..25 + vec_get(&hash_history, 0) % 23 {
            for _ in 0..hash_history.len() {
                h = 1 + ((h + 1) % hash_history.len() as u32);

                let hash_val = vec_get(&hash_history, h as usize - 1);

                let inp_val = inp
                    .chars()
                    .skip((h as f64 % parsed_inp) as usize)
                    .next()
                    .unwrap_or('0')
                    .to_digit(10)
                    .unwrap();

                let comb_val =
                    (inp_val + hash_val + ((h - 1) % hash_history.len() as u32) * 17) as f64;

                let vec_val = vec_get(
                    &hash_history,
                    (primelist_get(h as usize % PRIMELIST.len()) * hash_val) as usize
                        % hash_history.len(),
                ) as f64;

                let exp_val = EXP_VARS
                    .get((((h + summ) % 13) + (hash_val % 10) * 13) as usize)
                    .unwrap();

                let final_val = 1 + ((comb_val + (vec_val + exp_val)) % 1013.0) as u32;

                *hash_history.get_mut(h as usize - 1).unwrap() = final_val;
            }
            hash_history.pop_front();
            if vec_get(&hash_history, 0) % 2 == 1 {
                hash_history.remove(12);
            }
        }

        for _ in 0..10 {
            hash_history.insert(
                hash_history.len() - (5 + self.out_len as usize % 11) - 1,
                *hash_history.front().unwrap(),
            );
            hash_history.pop_front();
            hash_history.push_back(vec_get(&hash_history, self.out_len as usize - 1));
            hash_history.remove(self.out_len as usize - 1);
        }

        for _ in 0..self.out_len {
            let inp_val = inp
                .chars()
                .skip(
                    (vec_get(
                        &hash_history,
                        (hash_history.len() as f64 / 50.0).round() as usize,
                    ) as f64
                        % parsed_inp) as usize,
                )
                .next()
                .unwrap_or('0')
                .to_digit(10)
                .unwrap();

            let hash_range = hash.len()..=hash.len();
            if vec_get(&hash_history, 0) % 2 == inp_val % 2 {
                hash.push_str(((vec_get(&hash_history, 0) as f64 / 10.0).round() as u32 % 10).to_string().as_str());
                let hash_letter = hash[hash_range].parse().expect("parsed slice");
                for _ in 0..hash_letter {
                    hash_history.insert(hash_history.len() - 4, vec_get(&hash_history, 3));
                    hash_history.remove(3);
                }
            } else {
                hash.push_str((vec_get(&hash_history, 0) % 10).to_string().as_str());
                let hash_letter = hash[hash_range].parse().expect("parsed slice");
                for _ in 0..4 {
                    hash_history.push_back(vec_get(&hash_history, hash_letter));
                    hash_history.remove(hash_letter);
                }
            }

            hash_history.insert(hash_history.len() - 11, vec_get(&hash_history, 0));
            hash_history.remove(0);
        }

        hash
    }
}

#[inline]
fn vec_get(vec: &VecDeque<u32>, idx: usize) -> u32 {
    *vec.get(idx).unwrap_or(&0)
}

#[inline]
fn primelist_get(idx: usize) -> u32 {
    *PRIMELIST.get(idx).unwrap_or(&0)
}

fn find_in_alphabet(char: char) -> Option<u32> {
    let mut accum = 0;
    ALPHABET.chars().find(|&v| {
        accum += 1;
        v == char
    })?;

    Some(accum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabet() {
        assert_eq!(find_in_alphabet('д'), Some(73));
        assert_eq!(find_in_alphabet('4'), Some(6));
        assert_eq!(find_in_alphabet('e'), Some(47));
        assert_eq!(find_in_alphabet('\\'), Some(19));
        assert_eq!(find_in_alphabet('"'), Some(40));
        assert_eq!(find_in_alphabet(' '), Some(1));
        assert_eq!(find_in_alphabet('Ы'), None);
    }

    #[test]
    fn hash_encode_test() {
        let hashell = Hashell::new(0);

        assert_eq!(
            hashell.hash_encode("486486313653298"),
            "18369657183482421359888518530247712270482".to_owned()
        );
        assert_eq!(
            hashell.hash_encode("fie30a9fhafh9wjnsp0"),
            "46528728377671058223115703991284538790667241351769825356".to_owned()
        );
        assert_eq!(
            hashell.hash_encode("-020-4i032230fo"),
            "13615946119430140565153618124041860052116".to_owned()
        );
        assert_eq!(
            hashell.hash_encode("  wwpoj3ropawjo"),
            "1901789549586247029260330852971955663701411".to_owned()
        );
        assert_eq!(hashell.hash_encode("]]30r30]iir3]0]r30]-r0i30wqqrw30-j"), "319065824141868362089442283047729756599494547735686119071365173902419132421299123360831486430882".to_owned());
        assert_eq!(
            hashell.hash_encode("394334512383894"),
            "6424712464477419125298535182144776974187".to_owned()
        );
        assert_eq!(
            hashell.hash_encode("8778484646941145632546"),
            "1030468187655416589395067601419622987737083453663365541716".to_owned()
        );

        assert_eq!(hashell.hash_encode("0"), "2".to_owned());
    }

    #[test]
    fn hash_number_test() {
        let hashell = Hashell::new(16);

        assert_eq!(
            "0555279339417795".to_owned(),
            hashell.digest("3483488345939349")
        );

        assert_eq!(
            "3973351530153453".to_owned(),
            hashell.digest("0")
        );

        assert_eq!(
            "1095160231297737".to_owned(),
            hashell.digest("1")
        );

        let hashell = Hashell::new(15);

        assert_eq!(
            "755539785697931".to_owned(),
            hashell.digest("394334512383894")
        );

        assert_eq!(
            "933097009979939".to_owned(),
            hashell.digest("8778484646941145632546")
        );

        assert_eq!(
            "736348119714255".to_owned(),
            hashell.digest("asrgaeфыкрфкр4984612")
        );

        let hashell = Hashell::new(120);

        assert_eq!(
            "678779357945397196395199965043199010191399199043079718719201731813323999879713959292058944979749150493093286654673351543".to_owned(),
            hashell.digest("394334512383894")
        );

        assert_eq!(
            "914937769319477494315979533594219379210994159831194102531133189333595613942051627930572350359741979903311599949173916579".to_owned(),
            hashell.digest("8778484646941145632546")
        );

        assert_eq!(
            "943891315585135305317517730943583572837303418114493569641982101018331964979535915753136415018093539879929973199912995333".to_owned(),
            hashell.digest("asrgaeфыкрфкр4984612")
        );
    }

    #[test]
    #[should_panic]
    fn hash_empty_string() {
        let hasher = Hashell::new(10);
        hasher.digest("");
    }
}
