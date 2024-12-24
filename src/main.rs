use clap::{Parser, ValueEnum};
use wordle_solver::Guesser;

const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser, Debug)]
#[clap(author, version, about,long_about=None)]
struct Args {
    #[clap(short, long, value_enum)]
    implementation: Implementation,

    #[clap(short, long)]
    max: Option<usize>,
}

#[derive(ValueEnum, Debug, Clone)]
enum Implementation {
    Naive,
    Allocs,
    VecRem,
}

fn main() {
    let args = Args::parse();

    match args.implementation {
        Implementation::Naive => play(wordle_solver::algorithms::Naive::new, args.max),
        Implementation::Allocs => play(wordle_solver::algorithms::Allocs::new, args.max),
        Implementation::VecRem => play(wordle_solver::algorithms::VecRem::new, args.max),
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>)
where
    G: Guesser,
{
    let w = wordle_solver::Wordle::new();
    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = (mk)();
        if let Some(score) = w.play(answer, guesser) {
            println!("guessed '{}' in {}", answer, score);
        } else {
            eprintln!("failed to guess");
        }
    }
}
