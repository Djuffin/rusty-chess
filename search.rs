use types::*;
use eval::{SimpleEvaluator, Evaluator, INFINITY, Score}; 
use std::cmp::{max,min};
use std::collections::HashMap;

type SearchCache = HashMap<u64, PositionInfo>;

struct SearchEngine {
    evaluator: Box<Evaluator + 'static>,
    search_cache: SearchCache
}

struct Search {
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

struct PositionInfo {
    depth: uint,
    score: Score,
    win: Window
}

impl SearchEngine {
    pub fn new() -> SearchEngine {
        SearchEngine {
            evaluator: box SimpleEvaluator::new(),
            search_cache: HashMap::new()
        }
    }
}

impl Search {
    pub fn new(pos: &Position) -> Search {
        Search {
            position: *pos,
            top_line: Line {
                move:NullMove,
                score: 0,
                children: Vec::with_capacity(0),
            },
        }
    }


    pub fn top_moves(&self, n:uint) -> Vec<Move> {
        let mut result:Vec<Move> = Vec::with_capacity(n);
        for i in range(0, min(n, self.top_line.children.len())) {
            result.push(self.top_line.children[i].move);
        }
        result
    }

    pub fn calculate_lines(&mut self, search_engine: &mut SearchEngine, depth:uint) {
        let window = Window { alpha: -INFINITY, beta:INFINITY };
        self.top_line.score = alphabeta(search_engine, &self.position, &mut self.top_line, window, depth);
    }

}

fn alphabeta(search_engine: &mut SearchEngine, pos: &Position, line: &mut Line, win: Window, depth: uint ) -> Score {
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
    } else {
        //Sort child moves in order of their decreasing benefit for the moving side. 
        //This sort is based on the score that was obtained on previous iterations.
        //It should help us to do more pruning, since we look through better moves first.
        if pos.next_to_move == White {
            line.children.sort_by(|a, b| b.score.cmp(&a.score));
        } else {
            line.children.sort_by(|a, b| a.score.cmp(&b.score));
        }        
    }

    let mut window = Window { alpha: win.alpha, beta:win.beta };
    for child in line.children.iter_mut() {
        let mut new_pos = *pos;
        new_pos.apply_move(&child.move);   
        let hash = ::hash::calc_position_hash(&new_pos);
        
        //let pi = search_engine.search_cache.find(&hash);
        //TODO: use pi value
        
        let score = if depth == 0 {
            search_engine.evaluator.eval(&new_pos)
        } else {
            alphabeta(search_engine, &new_pos, child, window, depth - 1)
        }; 

        search_engine.search_cache.insert(hash, PositionInfo { score:score, depth: depth, win: window });
        
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
            window.alpha
        } else {
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
    let mut search_engine = SearchEngine::new();
    for i in range(0, depth + 1) {
        search.calculate_lines(&mut search_engine, i);
    }
    let moves = search.top_moves(3);
    if moves.len() > 0 {
        Some(moves[0])
    } else {
        None
    }

}
