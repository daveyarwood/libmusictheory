use regex::Regex;

pub struct Note {
    letter: char,
    // a positive or negative number representing number of semitones up/down
    accidentals: isize,
    octave: isize
}

fn semitones_from_c(letter: char) -> isize {
    match letter {
        'c' => 0,
        'd' => 2,
        'e' => 4,
        'f' => 5,
        'g' => 7,
        'a' => 9,
        'b' => 11,
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
                    ltr = letter.to_lowercase().chars().next().unwrap();
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
    pub fn from_str(s: &str) -> Result<Note, String> {
        parse_note_components(s).map_err(|msg| match msg {
            None      => "Invalid note format".to_string(),
            Some(msg) => format!("Invalid note format: {} ({})", s, msg)
        })
    }

    pub fn number(&self) -> isize {
        let base_note = semitones_from_c(self.letter) + (self.octave * 12) + 12;
        base_note + self.accidentals
    }
}
