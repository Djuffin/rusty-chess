use fen::parse_fen;
use types::*;

fn pick_best_move(fen:&str, best_move_hint:&str) -> Option<String> {
    let pos = parse_fen(fen).unwrap();
    let moves = ::move_gen::MovesIterator::new(&pos);
    for mv in moves {
        let (piece, from_sq, to_sq) = match mv {
            OrdinaryMove (ref of) => {
                let piece = match of.kind {
                    Pawn   => "",
                    Knight => "N",
                    Bishop => "B",
                    Rook   => "R",
                    King   => "K",
                    Queen  => "Q"
                };
                (piece, of.from.to_string(), of.to.to_string())
            },
            _ => continue
        };
        if best_move_hint.starts_with(piece) && best_move_hint.contains(&to_sq) {
            return Some(from_sq + &to_sq);
        }
    }
    return None;
}

fn assert_bestmove(fen:&str, best_move_hint:&str, depth:i32) {
    use std::io::{Read, Write, Cursor};
    let best_mv = pick_best_move(fen, best_move_hint).unwrap_or("(move not found)".to_string());
    println!("testing: {}, best move {} ({})", fen, best_mv, best_move_hint);
    let mut input = Cursor::new(Vec::new());
    let mut output = Cursor::new(Vec::new());
    write!(input, "position fen {}\n", fen).unwrap();
    write!(input, "go depth {}\n", depth).unwrap();
    input.set_position(0);
    ::uci::UciEngine::new().main_loop(&mut input, &mut output);
    let mut result = String::new();
    output.set_position(0);
    output.read_to_string(&mut result).unwrap();
    println!("Engine output: {}", result);
    assert!(result.contains(&best_mv));
}

#[test]
fn search_test() {
    ::tables::init_tables();
    let depth = 6;

    assert_bestmove("1k1r4/pp1b1R2/3q2pp/4p3/2B5/4Q3/PPP2B2/2K5 b - - 0 1", "Qd1", depth);
    // assert_bestmove("r1b1r1k1/p1p3pp/2p2n2/2bp4/5P2/3BBQPq/PPPK3P/R4N1R b q - 0 1", "Bg4", depth);
}
