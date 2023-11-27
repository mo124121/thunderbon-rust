use std::{cmp, collections::BinaryHeap, time::Instant};

use rand::{rngs::StdRng, Rng, SeedableRng};

const H: usize = 30;
const W: usize = 30;
const END_TERN: u32 = 100;
const moves: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Default, Clone, PartialEq, Eq)]
struct Coord {
    h: i32,
    w: i32,
}

#[derive(Clone, PartialEq, Eq)]
struct MazeState {
    points: Vec<Vec<i32>>,
    turns: u32,
    game_score: i32,
    evaluated_score: i32,
    character: Coord,
    first_action: (i32, i32),
}
impl MazeState {
    fn new(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let character = Coord {
            w: rng.gen_range(0..W) as i32,
            h: rng.gen_range(0..H) as i32,
        };
        let mut points = vec![vec![0; W]; H];
        for h in 0..H {
            for w in 0..W {
                points[h][w] = rng.gen_range(0..10);
            }
        }
        MazeState {
            points,
            turns: 0,
            game_score: 0,
            evaluated_score: 0,
            character,
            first_action: (0, 0),
        }
    }

    fn is_done(&self) -> bool {
        self.turns == END_TERN
    }

    fn evaluate_score(&mut self) {
        self.evaluated_score = self.game_score;
    }

    fn advance(&mut self, _move: (i32, i32)) {
        self.character.w += _move.0;
        self.character.h += _move.1;
        self.game_score += self.points[self.character.h as usize][self.character.w as usize];
        self.points[self.character.h as usize][self.character.w as usize] = 0;
        self.turns += 1;
    }
    fn generate_legal_actions(&self) -> Vec<(i32, i32)> {
        let mut actions: Vec<(i32, i32)> = Vec::new();
        for (dw, dh) in moves {
            let nw = self.character.w + dw;
            let nh = self.character.h + dh;
            if 0 <= nw && nw < W as i32 && 0 <= nh && nh < H as i32 {
                actions.push((dw, dh));
            }
        }
        actions
    }
}
impl Ord for MazeState {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}
impl PartialOrd for MazeState {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::fmt::Display for MazeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "turns:\t{}", self.turns)?;
        writeln!(f, "score:\t{}", self.game_score)?;
        for h in 0..H {
            for w in 0..W {
                if self.character.w == w as i32 && self.character.h == h as i32 {
                    write!(f, "@")?;
                } else if self.points[h][w] > 0 {
                    write!(f, "{}", self.points[h][w])?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

struct TimeKeeper {
    started_time: Instant,
    time_threshold_milseconds: u128,
}
impl TimeKeeper {
    fn new(time_threshold_milseconds: u128) -> Self {
        TimeKeeper {
            started_time: Instant::now(),
            time_threshold_milseconds,
        }
    }
    fn is_time_over(&self) -> bool {
        self.started_time.elapsed().as_millis() >= self.time_threshold_milseconds
    }
}

fn random_action(state: &MazeState) -> (i32, i32) {
    let actions = state.generate_legal_actions();
    actions[rand::thread_rng().gen_range(0..actions.len())]
}

fn greedy_action(state: &MazeState) -> (i32, i32) {
    let actions = state.generate_legal_actions();
    let mut best_score: i32 = -1;
    let mut best_action = (-1, -1);
    for action in actions {
        let mut now_state = (*state).clone();
        now_state.advance(action);
        now_state.evaluate_score();
        if now_state.evaluated_score > best_score {
            best_score = now_state.evaluated_score;
            best_action = action;
        }
    }
    best_action
}

fn beam_search_action(state: &MazeState, beam_width: usize, beam_depth: usize) -> (i32, i32) {
    let mut now_beam = BinaryHeap::new();
    let mut best_state = state.clone();
    now_beam.push(state.clone());
    for t in 0..beam_depth {
        let mut next_beam = BinaryHeap::new();
        for _ in 0..beam_width {
            match now_beam.pop() {
                Some(now_state) => {
                    let legal_actions = now_state.generate_legal_actions();
                    for action in legal_actions {
                        let mut next_state = now_state.clone();
                        next_state.advance(action);
                        next_state.evaluate_score();
                        if t == 0 {
                            next_state.first_action = action;
                        }
                        next_beam.push(next_state);
                    }
                }
                None => break,
            }
        }
        now_beam = next_beam;
        best_state = now_beam.pop().unwrap();
        if best_state.is_done() {
            break;
        }
        now_beam.push(best_state.clone());
    }

    best_state.first_action
}

fn beam_search_with_time_threshold(
    state: &MazeState,
    beam_width: usize,
    time_threshold_milseconds: u128,
) -> (i32, i32) {
    let mut time_keeper = TimeKeeper::new(time_threshold_milseconds);

    let mut now_beam = BinaryHeap::new();
    let mut best_state = state.clone();
    now_beam.push(state.clone());
    for t in 0.. {
        let mut next_beam = BinaryHeap::new();
        for _ in 0..beam_width {
            if time_keeper.is_time_over() {
                if best_state.first_action == (0, 0) {
                    panic!()
                }
                return best_state.first_action;
            }
            match now_beam.pop() {
                Some(now_state) => {
                    let legal_actions = now_state.generate_legal_actions();
                    for action in legal_actions {
                        let mut next_state = now_state.clone();
                        next_state.advance(action);
                        next_state.evaluate_score();
                        if t == 0 {
                            next_state.first_action = action;
                        }
                        next_beam.push(next_state);
                    }
                }
                None => break,
            }
        }
        now_beam = next_beam;
        best_state = now_beam.pop().unwrap();
        if best_state.is_done() {
            break;
        }
        now_beam.push(best_state.clone());
    }

    best_state.first_action
}

fn play_game(seed: u64) {
    let mut state = MazeState::new(seed);
    println!("{}", &state);

    while !state.is_done() {
        state.advance(greedy_action(&state));
        println!("{}", &state);
    }
}

fn test_ai_score(game_number: usize) {
    let mut rng = StdRng::seed_from_u64(0);
    let mut score_cum = 0i32;
    for i in 0..game_number {
        eprintln!("{}", i);
        let mut state = MazeState::new(rng.gen());
        while !state.is_done() {
            // state.advance(beam_search_action(&state, 2, END_TERN as usize));
            state.advance(beam_search_with_time_threshold(&state, 10, 10));
        }
        score_cum += state.game_score;
    }
    println!("{}", score_cum as f32 / game_number as f32);
}

fn main() {
    println!("start game");
    // play_game(121321);
    println!("start evaluate");
    test_ai_score(100);
}
