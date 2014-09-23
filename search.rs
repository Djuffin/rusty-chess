use types::*;
use eval::{SimpleEvaluator, Evaluator, INFINITY, Score}; 
use std::cmp::{max,min};

pub struct Search {
    evaluator: Box<Evaluator + 'static>,
    position: Position,
    top_line: Line
}

struct Window {
    alpha: Score, //min score that whites can already count on
    beta: Score   //max score that blacks can already count on
}

struct Line {
    move: Move,
    score: Score,
    children: Vec<Line>
}

impl Search {
    pub fn new(pos: &Position) -> Search {
        Search {
            evaluator: box SimpleEvaluator::new(),
            position: *pos,
            top_line: Line {
                move:NullMove,
                score: 0,
                children: Vec::with_capacity(0)
            } 
        }
    }


    pub fn top_moves(&self, n:uint) -> Vec<Move> {
        let mut result:Vec<Move> = Vec::with_capacity(n);
        for i in range(0, min(n, self.top_line.children.len())) {
            result.push(self.top_line.children[i].move);
        }
        result
    }

    pub fn calculate_lines(&mut self, depth:uint) {
        let window = Window { alpha: -INFINITY, beta:INFINITY };
        self.top_line.score = alphabeta(&*self.evaluator, &mut self.position, &mut self.top_line, window, depth);
    }

}

fn alphabeta(evaluator: &Evaluator, pos: &Position, line:&mut Line, win:Window, depth:uint) -> Score {
    let very_bad_score = if pos.next_to_move == White { -INFINITY } else { INFINITY };
    if line.children.is_empty() {
        let mut moves = pos.gen_moves();
        let (size, _) = moves.size_hint();
        line.children.reserve(size);
        for move in moves {
            line.children.push(Line {
                move: move,
                score: very_bad_score,
                children: Vec::with_capacity(0)
            });   
        }           
    }

    let mut window = Window { alpha: win.alpha, beta:win.beta };
    for child in line.children.iter_mut() {
        let mut new_pos = *pos;
        new_pos.apply_move(&line.move);   
        
        let score = if depth == 0 {
            evaluator.eval(&new_pos)
        } else {
            alphabeta(evaluator, &new_pos, child, window, depth - 1)
        }; 
        
        child.score = score;  
        if pos.next_to_move == White {
            window.alpha = max(window.alpha, score);
        } else {
            window.beta = min(window.beta, score);
        }
        if window.beta <= window.alpha {
            break;
        }                         
    }

    let result = if line.children.len() > 0 {
        if pos.next_to_move == White {
            line.children.sort_by(|a, b| b.score.cmp(&a.score));
            window.alpha
        } else {
            line.children.sort_by(|a, b| a.score.cmp(&b.score));
            window.beta
        }            
    } else {
        if pos.is_check() {
            //no moves available and check - checkmate
            very_bad_score
        } else {
            //no moves, but no check - stalemate
            //draw!
            0
        }
    };
    result
}


pub fn search(pos: &Position, depth: uint) -> Option<Move> {
    let mut search = Search::new(pos);
    search.calculate_lines(depth);
    let moves = search.top_moves(3);
    if moves.len() > 0 {
        Some(moves[0])
    } else {
        None
    }

}

/*
fn internal_search(p: &Position, eval: &Evaluator, depth: uint) -> (Option<Move>, i32) {
    let color = p.next_to_move;
    let mut best_move:Option<Move> = None;
    let mut best_score = if color == White { -INFINITY  } else { INFINITY };
    for move in p.gen_moves() {
        let mut p1 = *p;
        p1.apply_move(&move);
        let score = if depth == 0 { 
            eval.eval(&p1)
        } else {
            let (_, score) = internal_search(&p1, eval, depth - 1);
            score
        };
        if (color == White && score >= best_score) ||
           (color == Black && score <= best_score) {
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
} */