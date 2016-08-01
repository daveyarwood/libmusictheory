use regex::Regex;
use std::iter::repeat;

pub struct Note {
    letter: char,
    // a positive or negative number representing number of semitones up/down
    accidentals: isize,
    octave: isize
}

fn semitones_from_c(letter: char) -> isize {
    match letter {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _   => panic!(format!("{} isn't a note", letter))
    }
}


fn parse_note_components(s: &str) -> Result<Note, Option<&str>> {
    let re = Regex::new(r#"([A-Ga-g])([#b]*)(-?\d+)"#).unwrap();
    match re.captures(s) {
        None => return Err(None),
        Some(caps) => {
            let ltr:  char;
            let mut accs: isize = 0;
            let oct:  isize;

            match caps.at(1) {
                None         => { return Err(Some("missing letter")); },
                Some(letter) => {
                    ltr = letter.to_uppercase().chars().next().unwrap();
                }
            }

            for c in caps.at(2).unwrap().chars() {
                match c {
                    'b' => { accs -= 1; },
                    '#' => { accs += 1; },
                    _   => { panic!("regex doesn't work lol"); }
                }
            }

            match caps.at(3) {
                None         => { return Err(Some("missing octave")); },
                Some(octave) => { oct = octave.parse::<isize>().unwrap(); }
            }

            return Ok(Note { letter: ltr, accidentals: accs, octave: oct });
        }
    }
}

impl Note {
    pub fn from_string(s: &str) -> Result<Note, String> {
        parse_note_components(s).map_err(|msg| match msg {
            None      => "Invalid note format".to_string(),
            Some(msg) => format!("Invalid note format: {} ({})", s, msg)
        })
    }

    pub fn to_string(&self) -> String {
        format!("{}{}{}", self.letter, self.accidentals_string(), self.octave)
    }

    pub fn number(&self) -> isize {
        let base_note = semitones_from_c(self.letter) + (self.octave * 12) + 12;
        base_note + self.accidentals
    }

    pub fn number_plus_interval(number: isize, interval: Interval) -> isize {
        number + interval.semitones()
    }

    pub fn spell(note_number: isize, letter: char) -> Result<Note, String> {
        assert!(note_number >= 0, "Note number must be >= 0.");

        let letter = letter.to_uppercase().next().unwrap();

        let base_note = semitones_from_c(letter);

        let mut closest    = base_note;
        let mut octave     = -1;
        let mut difference = note_number - closest;

        while difference > 6 {
            closest += 12;
            octave += 1;
            difference = note_number - closest;
        }

        Ok(Note { letter: letter, accidentals: difference, octave: octave })
    }

    fn accidentals_string(&self) -> String {
        if self.accidentals.is_positive() {
            repeat("#").take(self.accidentals as usize).collect::<String>()
        } else if self.accidentals.is_negative() {
            repeat("b").take(-self.accidentals as usize).collect::<String>()
        } else {
            "".to_string()
        }
    }
}

#[derive(Copy, Clone)]
enum Quality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished
}

pub struct Interval {
    quality: Quality,
    number: isize
}

fn interval_number_supported(n: isize) -> bool {
    1 <= n && n <= 15
}


fn parse_interval_components(s: &str) -> Result<Interval, Option<String>> {
    let re = Regex::new(r#"([PMmAd])(\d+)"#).unwrap();
    match re.captures(s) {
        None => return Err(None),
        Some(caps) => {
            let qual: Quality;
            let num:  isize;

            match caps.at(1) {
                None      => {
                    return Err(Some("missing quality".to_string()));
                },
                Some("P") => { qual = Quality::Perfect; },
                Some("M") => { qual = Quality::Major; },
                Some("m") => { qual = Quality::Minor; },
                Some("A") => { qual = Quality::Augmented; },
                Some("d") => { qual = Quality::Diminished; },
                Some(_)   => { panic!("regex doesn't work lol"); }
            }

            match caps.at(2) {
                None         => {
                    return Err(Some("missing number".to_string()));
                },
                Some(s) => {
                    match s.parse::<isize>() {
                        Err(_) => {
                            return Err(Some("error parsing number".to_string()));
                        },
                        Ok(n) if interval_number_supported(n) => { num = n; },
                        Ok(n) => {
                            let msg = format!(
                                "unsupported interval number: {}", n)
                                .to_string();
                            return Err(Some(msg));
                        }
                    }
                }
            }

            return Ok(Interval { quality: qual, number: num });
        }
    }
}

impl Interval {
    pub fn from_string(s: &str) -> Result<Interval, String> {
        parse_interval_components(s).map_err(|msg| match msg {
            None      => "Invalid interval format".to_string(),
            Some(msg) => format!("Invalid interval format: {} ({})", s, msg)
        })
    }

    pub fn to_string(self) -> String {
        let q = match self.quality {
            Quality::Perfect    => "P",
            Quality::Major      => "M",
            Quality::Minor      => "m",
            Quality::Augmented  => "A",
            Quality::Diminished => "d"
        };

        format!("{}{}", q, self.number)
    }

    pub fn semitones(self) -> isize {
        match (self.quality, self.number) {
            (Quality::Perfect, 1)     => 0,
            (Quality::Minor, 2)       => 1,
            (Quality::Major, 2)       => 2,
            (Quality::Diminished, 3)  => 2,
            (Quality::Minor, 3)       => 3,
            (Quality::Major, 3)       => 4,
            (Quality::Augmented, 3)   => 5,
            (Quality::Perfect, 4)     => 5,
            (Quality::Augmented, 4)   => 6,
            (Quality::Diminished, 5)  => 6,
            (Quality::Perfect, 5)     => 7,
            (Quality::Minor, 6)       => 8,
            (Quality::Major, 6)       => 9,
            (Quality::Minor, 7)       => 10,
            (Quality::Major, 7)       => 11,
            (Quality::Perfect, 8)     => 12,
            (Quality::Minor, 9)       => 13,
            (Quality::Major, 9)       => 14,
            (Quality::Minor, 10)      => 15,
            (Quality::Major, 10)      => 16,
            (Quality::Perfect, 11)    => 17,
            (Quality::Augmented, 11)  => 18,
            (Quality::Diminished, 12) => 18,
            (Quality::Perfect, 12)    => 19,
            (Quality::Minor, 13)      => 20,
            (Quality::Major, 13)      => 21,
            (Quality::Minor, 14)      => 22,
            (Quality::Major, 14)      => 23,
            (Quality::Perfect, 15)    => 24,
            (_, _)                    => {
                let msg = format!("Unsupported interval: {}", self.to_string());
                panic!(msg);
            }
        }
    }
}

