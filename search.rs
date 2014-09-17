use types::*;
use eval::{SimpleEvaluator, Evaluator}; 

pub fn search(pos: &Position, depth: uint) -> Option<Move> {
    let (move,_) = internal_search(pos, &SimpleEvaluator::new(), depth);
    move
}


fn internal_search(p: &Position, eval: &Evaluator, depth: uint) -> (Option<Move>, i32) {
    let color = p.next_to_move;
    let mut best_move:Option<Move> = None;
    let mut best_score = if color == White { ::std::i32::MIN  } else { ::std::i32::MAX };
    for move in p.gen_moves() {
        let mut p1 = *p;
        p1.apply_move(&move);
        let score = if depth == 0 { 
            eval.eval(&p1)
        } else {
            let (_, score) = internal_search(&p1, eval, depth - 1);
            score
        };
        if (color == White && score > best_score) ||
           (color == Black && score < best_score) {
            best_score = score;
            best_move = Some (move);
        }
    }
    if best_move.is_none() && !p.is_check() {
        //no moves, but no check - stalemate
        //draw!
        best_score = 0;
    }
    (best_move, best_score)
}