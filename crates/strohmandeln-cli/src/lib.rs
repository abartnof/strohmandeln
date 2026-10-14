//! `strohmandeln`: play a sitting against the computer in the terminal.
//!
//! ```text
//! strohmandeln [--lang en|de] [--seed N] [--touren 3|6] [--seat 0|1] [--level N]
//! ```
//!
//! English by default. Every string is in `text` (en and de).

pub mod render;
pub mod text;

use std::io::{self, BufRead, Write};
use strohmandeln_core::agents::{Player, RandomPlayer};
use strohmandeln_core::session::Sitting;
use strohmandeln_core::types::{Action, Seat, Settings};
use strohmandeln_core::view::{filter_event, view};
use text::{refusal, t, Lang};

#[derive(Clone, Debug)]
pub struct Options {
    pub lang: Lang,
    pub seed: u64,
    pub touren: u32,
    pub seat: Seat,
    /// The opponent's level; opaque until the ladder exists.
    pub level: u32,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            lang: Lang::En,
            seed: 1,
            touren: 3,
            seat: Seat::A,
            level: 1,
        }
    }
}

/// Reads the command line (without the program name).
pub fn parse_args(args: &[String]) -> Result<Options, String> {
    let mut o = Options::default();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut value = |name: &str| {
            it.next()
                .cloned()
                .ok_or_else(|| format!("{name} needs a value"))
        };
        match a.as_str() {
            "--lang" => {
                let v = value("--lang")?;
                o.lang = Lang::parse(&v).ok_or(format!("unknown language {v:?} (en or de)"))?;
            }
            "--seed" => {
                o.seed = value("--seed")?
                    .parse()
                    .map_err(|_| "--seed wants a number")?
            }
            "--touren" => {
                o.touren = value("--touren")?
                    .parse()
                    .map_err(|_| "--touren wants 3 or 6")?;
                if o.touren != 3 && o.touren != 6 {
                    return Err("--touren wants 3 or 6".into());
                }
            }
            "--seat" => {
                let n: u8 = value("--seat")?
                    .parse()
                    .map_err(|_| "--seat wants 0 or 1")?;
                o.seat = Seat::try_from(n)?;
            }
            "--level" => {
                o.level = value("--level")?
                    .parse()
                    .map_err(|_| "--level wants a number")?
            }
            "-h" | "--help" => return Err(t(o.lang, "usage", &[])),
            other => return Err(format!("unknown option {other:?}; try --help")),
        }
    }
    Ok(o)
}

/// The opponent for now: the core's random player.
fn opponent(o: &Options) -> Box<dyn Player> {
    Box::new(RandomPlayer::new(o.seed, o.seat.other()))
}

/// Which choice a line of input names: a number, a card as printed or as
/// coded, or `take` / `pass` / `y` / `n`.
fn pick(line: &str, choices: &[(Action, String)], lang: Lang) -> Option<usize> {
    let line = line.trim();
    if let Ok(n) = line.parse::<usize>() {
        return (1..=choices.len()).contains(&n).then(|| n - 1);
    }
    let want = line.to_lowercase();
    choices.iter().position(|(a, _)| match a {
        Action::Play { card, .. } => {
            card.code().to_lowercase() == want || text::card(lang, *card).to_lowercase() == want
        }
        Action::Take => want == "take",
        Action::Pass => want == "pass",
        Action::Extend { accept } => want == if *accept { "y" } else { "n" },
        Action::Announce { .. } => false,
    })
}

/// Plays a whole sitting, reading the human's choices from `input` and
/// writing the table to `out`. Returns the finished sitting; `None` if the
/// human quit (or the input ended) first.
pub fn run(
    o: &Options,
    input: &mut impl BufRead,
    out: &mut impl Write,
) -> io::Result<Option<Sitting>> {
    let lang = o.lang;
    let me = o.seat;
    let mut sitting = Sitting::new(o.seed, Settings::default(), o.touren)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let mut opp = opponent(o);
    let mut told = 0;
    let tell = |sitting: &Sitting, told: &mut usize, out: &mut dyn Write| -> io::Result<()> {
        let all = sitting.events();
        for e in &all[*told..] {
            if let Some(e) = filter_event(e, me) {
                if let Some(line) = render::narrate(lang, &e, me) {
                    writeln!(out, "{line}")?;
                }
            }
        }
        *told = all.len();
        Ok(())
    };
    tell(&sitting, &mut told, out)?;
    while let Some(seat) = sitting.to_move() {
        if seat != me {
            let v = view(&sitting, seat);
            let action = opp
                .choose(&v)
                .ok_or_else(|| io::Error::other("the opponent has no action to choose"))?;
            sitting
                .apply(seat, action)
                .map_err(|e| io::Error::other(refusal(lang, &e)))?;
            tell(&sitting, &mut told, out)?;
            continue;
        }
        let v = view(&sitting, me);
        let choices = render::choices(lang, &v);
        writeln!(out, "\n{}", render::table(lang, &v))?;
        for (i, (_, label)) in choices.iter().enumerate() {
            writeln!(out, "  {:>2}. {label}", i + 1)?;
        }
        loop {
            write!(out, "{}", t(lang, "choose", &[]))?;
            out.flush()?;
            let mut line = String::new();
            if input.read_line(&mut line)? == 0 || line.trim() == "q" {
                writeln!(out, "\n{}", t(lang, "quit", &[]))?;
                return Ok(None);
            }
            let Some(i) = pick(&line, &choices, lang) else {
                writeln!(out, "{}", t(lang, "bad-choice", &[]))?;
                continue;
            };
            match sitting.apply(me, choices[i].0.clone()) {
                Ok(_) => break,
                Err(r) => writeln!(out, "{}", refusal(lang, &r))?,
            }
        }
        tell(&sitting, &mut told, out)?;
    }
    writeln!(out, "\n{}", render::table(lang, &view(&sitting, me)))?;
    Ok(Some(sitting))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    #[test]
    fn arguments_default_to_english_and_three_touren() {
        let o = parse_args(&[]).unwrap();
        assert_eq!(o.lang, Lang::En);
        assert_eq!(o.touren, 3);
        let o = parse_args(&args("--lang de --seed 9 --touren 6 --seat 1")).unwrap();
        assert_eq!(
            (o.lang, o.seed, o.touren, o.seat),
            (Lang::De, 9, 6, Seat::B)
        );
    }

    #[test]
    fn bad_arguments_are_errors() {
        for bad in [
            "--lang fr",
            "--touren 4",
            "--seat 2",
            "--seed x",
            "--what",
            "--seed",
        ] {
            assert!(parse_args(&args(bad)).is_err(), "{bad}");
        }
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone, Default)]
    struct Shared(Rc<RefCell<Vec<u8>>>);
    impl Write for Shared {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            self.0.borrow_mut().extend_from_slice(b);
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    /// A player at the keyboard: answers the first choice, or declines the
    /// offer of an extension, judging by what was last printed.
    struct Bot {
        out: Shared,
        line: Vec<u8>,
    }
    impl io::Read for Bot {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            unreachable!("only read_line is used")
        }
    }
    impl BufRead for Bot {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            let text = String::from_utf8_lossy(&self.out.0.borrow()).into_owned();
            let tail = text.rsplit("\n\n").next().unwrap_or("");
            let extension = tail.contains("Settle now") || tail.contains("Jetzt abrechnen");
            self.line = if extension {
                b"n\n".to_vec()
            } else {
                b"1\n".to_vec()
            };
            Ok(&self.line)
        }
        fn consume(&mut self, _: usize) {}
    }

    fn play(lang: Lang, seat: Seat) -> (String, Sitting) {
        let o = Options {
            lang,
            seat,
            seed: 11,
            ..Options::default()
        };
        let out = Shared::default();
        let mut bot = Bot {
            out: out.clone(),
            line: Vec::new(),
        };
        let mut w = out.clone();
        let sitting = run(&o, &mut bot, &mut w).unwrap().expect("finished");
        let text = String::from_utf8(out.0.borrow().clone()).unwrap();
        (text, sitting)
    }

    #[test]
    fn a_whole_sitting_in_english_ends_with_a_settlement() {
        let (text, s) = play(Lang::En, Seat::A);
        assert!(s.is_over());
        assert!(text.contains("Your hand"));
        assert!(text.contains("Settlement"));
        assert!(!text.contains("Zählung"));
    }

    #[test]
    fn a_whole_sitting_in_german_uses_the_german_table() {
        let (text, s) = play(Lang::De, Seat::B);
        assert!(s.is_over());
        assert!(text.contains("Deine Hand"));
        assert!(text.contains("Abrechnung"));
        assert!(!text.contains("Your hand"));
    }

    #[test]
    fn quitting_and_end_of_input_stop_cleanly() {
        let o = Options::default();
        let mut out = Vec::new();
        assert!(run(&o, &mut io::Cursor::new(b"q\n".to_vec()), &mut out)
            .unwrap()
            .is_none());
        let mut out = Vec::new();
        assert!(run(&o, &mut io::Cursor::new(Vec::new()), &mut out)
            .unwrap()
            .is_none());
    }

    #[test]
    fn a_bad_choice_is_told_and_asked_again() {
        let o = Options::default();
        let mut out = Vec::new();
        run(&o, &mut io::Cursor::new(b"99\nxyz\nq\n".to_vec()), &mut out).unwrap();
        let text = String::from_utf8(out).unwrap();
        assert_eq!(text.matches("Not one of the choices.").count(), 2);
    }
}
