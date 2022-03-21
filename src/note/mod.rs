use crate::{pitch::Pitch};
use core::fmt::{self, Debug};

mod accidental;
pub use accidental::Accidental;

mod letter;
pub use letter::Letter;

mod pitch_note;
pub use pitch_note::PitchNote;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    pub letter: Letter,
    pub accidental: Accidental,
}

impl Note {
    pub const fn new(letter: Letter, accidental: Accidental) -> Self {
        Self { letter, accidental }
    }

    pub const fn natural(letter: Letter) -> Self {
        Self::new(letter, Accidental::Natrual)
    }

    pub const fn flat(letter: Letter) -> Self {
        Self::new(letter, Accidental::Flat)
    }

    pub const fn sharp(letter: Letter) -> Self {
        Self::new(letter, Accidental::Sharp)
    }

    pub const fn from_sharp(pitch: Pitch) -> Self {
        match pitch {
            Pitch::C => Self::natural(Letter::C),
            Pitch::C_SHARP => Self::sharp(Letter::C),
            Pitch::D => Self::natural(Letter::D),
            Pitch::D_SHARP => Self::sharp(Letter::D),
            Pitch::E => Self::natural(Letter::E),
            Pitch::F => Self::natural(Letter::F),
            Pitch::F_SHARP => Self::sharp(Letter::F),
            Pitch::G => Self::natural(Letter::G),
            Pitch::G_SHARP => Self::sharp(Letter::G),
            Pitch::A => Self::natural(Letter::A),
            Pitch::A_SHARP => Self::sharp(Letter::A),
            Pitch::B => Self::natural(Letter::B),
            _ => unreachable!(),
        }
    }

    pub const fn from_flat(pitch: Pitch) -> Self {
        match pitch {
            Pitch::C => Self::natural(Letter::C),
            Pitch::C_SHARP => Self::flat(Letter::D),
            Pitch::D => Self::natural(Letter::D),
            Pitch::D_SHARP => Self::flat(Letter::E),
            Pitch::E => Self::natural(Letter::E),
            Pitch::F => Self::natural(Letter::F),
            Pitch::F_SHARP => Self::flat(Letter::G),
            Pitch::G => Self::natural(Letter::G),
            Pitch::G_SHARP => Self::flat(Letter::A),
            Pitch::A => Self::natural(Letter::A),
            Pitch::A_SHARP => Self::flat(Letter::B),
            Pitch::B => Self::natural(Letter::B),
            _ => unreachable!(),
        }
    }

    /// Returns the enharmonic note for `self` in flat notation.
    ///
    /// # Examples
    ///
    /// Convert a `Note` in sharp notation to flats
    /// ```
    /// use music::note::{Letter, Note};
    ///
    /// let note = Note::sharp(Letter::G);
    /// assert_eq!(note.into_flat(), Note::flat(Letter::A))
    /// ```
    ///
    /// Find a natural enharmonic note
    /// ```
    /// use music::note::{Letter, Note};
    ///
    /// let note = Note::flat(Letter::F);
    /// assert_eq!(note.into_flat(), Note::natural(Letter::E))
    /// ```
    pub const fn into_flat(self) -> Self {
        Self::from_flat(Pitch::from_note(self))
    }

    /// Returns the enharmonic note for `self` in sharp notation.
    ///
    /// # Examples
    ///
    /// Convert a `Note` in flat notation to sharps
    /// ```
    /// use music::note::{Letter, Note};
    ///
    /// let note = Note::flat(Letter::D);
    /// assert_eq!(note.into_sharp(), Note::sharp(Letter::C))
    /// ```
    ///
    /// Find a natural enharmonic note
    /// ```
    /// use music::note::{Letter, Note};
    ///
    /// let note = Note::sharp(Letter::B);
    /// assert_eq!(note.into_sharp(), Note::natural(Letter::C))
    /// ```
    pub const fn into_sharp(self) -> Self {
        Self::from_sharp(Pitch::from_note(self))
    }

    /// Returns `true` if the `self` is enharmonically equivalent to `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// use music::note::{Letter, Note};
    ///
    /// let note = Note::flat(Letter::D);
    /// assert!(note.is_enharmonic(Note::sharp(Letter::C)))
    /// ```
    ///
    /// This function will also return true if the notes are the same.
    /// ```
    /// use music::note::{Letter, Note};
    ///
    /// let note = Note::natural(Letter::C);
    /// assert!(note.is_enharmonic(note))
    /// ```
    pub const fn is_enharmonic(self, other: Self) -> bool {
        Pitch::from_note(self).into_byte() == Pitch::from_note(other).into_byte()
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let accidental = match self.accidental {
            Accidental::Natrual => "",
            Accidental::Sharp => "#",
            Accidental::DoubleSharp => "##",
            Accidental::Flat => "b",
            Accidental::DoubleFlat => "bb",
        };
        write!(f, "{}{}", self.letter, accidental)
    }
}
