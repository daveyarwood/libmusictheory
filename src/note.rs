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
