
#[derive(Debug)]
enum Error {
    BrokenDie,
    GameEnded,
    InvalidArgument(String),
}

trait ToError<T,E> {
    fn check(self, error: E) -> Result<T,E>;
}

impl<T,E> ToError<T,E> for Option<T> {
    fn check(self, error: E) -> Result<T,E> {
        self.ok_or(error)
    }
}

impl<T,E,_E> ToError<T,E> for Result<T,_E> {
    fn check(self, error: E) -> Result<T,E> {
        self.ok().check(error)
    }
}

trait TakeExactly<'a, I> where I: Iterator {
    fn take_exactly(&'a mut self, n: usize) -> TakeExactlyIterator<'a, I>;
}

struct TakeExactlyIterator<'a, I> where I: Iterator {
    iter: &'a mut I,
    done: usize,
    take: usize,
}

impl<'a, I, T> Iterator for TakeExactlyIterator<'a, I> where I: Iterator<Item=T> {
    type Item = Option<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let done = self.done;
        if done >= self.take {
            None
        } else {
            self.done += 1;
            Some(self.iter.next())
        }
    }
}

impl<'a, I> TakeExactly<'a, I> for I where I: Iterator {
    fn take_exactly(&'a mut self, n: usize) -> TakeExactlyIterator<'a, I> {
        TakeExactlyIterator{iter:self, done:0, take:n}
    }
}

#[derive(Copy,Clone,Debug)]
struct Player {
    score: usize,
    field: usize,
}

struct DiracGame<I> where I: Iterator<Item=usize> {
    round: usize,
    players: Vec<Player>,
    die: Option<I>,
    max: usize,
}

impl<I> DiracGame<I> where I: Iterator<Item=usize> {

    fn has_ended(&self) -> bool {
        self.die.is_none()
    }

    fn next_player(&self) -> Player {
        self.players[self.round % self.players.len()]
    }

    fn play_round(&mut self) -> Result<&Self,Error> {
        let score = match &mut self.die {
            None => Err(Error::GameEnded),
            Some(die) => die.take_exactly(3).sum::<Option<usize>>()
                .check(Error::BrokenDie),
        }?;
        let round = self.round;
        let index = round % self.players.len();
        let player = &mut self.players[index];
        player.field = ((player.field + score - 1) % 10) + 1;
        player.score += player.field;
        self.round += 1;
        if player.score >= self.max {
            self.die = None;
        }
        Ok(self)
    }

    fn new(starting_positions: &Vec<usize>, die: I, max: usize) -> Self {
        let players = starting_positions.into_iter().map(|&p| Player{score:0, field:p}).collect();
        DiracGame{die:Some(die), players, round:0, max}
    }

}

fn quantum_game(
    p1: &Player,
    p2: &Player,
) -> (usize, usize) {
    if p2.score >= 21 {
        (0, 1)
    } else {
        [(3,1), (4,3), (5,6), (6,7), (7,6), (8,3), (9,1)]
            .iter()
            .map(|(score, count)| {
                let field = ((p1.field + score - 1) % 10) + 1;
                let score = p1.score + field;
                let (w1, w2) = quantum_game(p2, &Player{field, score});
                (w2 * count, w1 * count)
            })
            .reduce(|(w1, w2), (u1, u2)| (w1 + u1, w2 + u2))
            .unwrap()
    }
}

fn main_or_error() -> Result<(),Error> {
    let positions: Result<Vec<usize>,Error> = std::env::args()
        .skip(1)
        .map(|arg| arg.parse().check(Error::InvalidArgument(arg)))
        .collect();
    let positions = positions?;
    let mut game = DiracGame::new(&positions, 1.., 1000);
    while !game.has_ended() {
        game.play_round()?;
    }

    println!("checksum: {}",  game.next_player().score * game.round * 3);

    println!("part 2: {:?}", quantum_game(
        &Player{field:positions[0], score:0},
        &Player{field:positions[1], score:0}));

    Ok(())
}

fn main() {
    if let Err(e) = main_or_error() {
        println!("error: {:?}", e);
    }
}


#[test]
fn example_part1() {
    let mut d = DiracGame::new(&vec![4,8], 1.., 1000);
    for _ in 0..330 {
        let r = d.play_round();
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(!r.has_ended());
    }
    assert!(d.play_round().unwrap().has_ended());
    assert_eq!(d.round, 331);
    assert_eq!(d.players[1].score, 745);
}

#[test]
fn example_part2() {
    assert_eq!(
        quantum_game(
            &Player{field:4,score:0},
            &Player{field:8,score:0},
        ), (444356092776315, 341960390180808));
}

