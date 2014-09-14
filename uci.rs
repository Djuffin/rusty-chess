//Implementation of Universal Chess Interface (UCI)
//http://wbec-ridderkerk.nl/html/UCIProtocol.html
use types::{Kind, Color, Move, Position, Board, Square};
use fen::{parse_fen, render_fen};


struct UciMove {
    from:Square,
    to:Square,
    promotion:Option<Kind>
}

enum SearchOption {
    MovetimeMsc(uint),
    Infinity
}

enum Command {
    CmdUci,
    CmdIsReady,
    CmdUciNewGame,
    CmdPosition (Position, Vec<UciMove>),
    CmdGo (SearchOption),
    CmdStop,
    CmdQuit
}

enum Response {
    RspId (String, String),
    RspUciOk,
    RspReadyOk,
    RspBestMove (Move),
    RspInfo (String),
    RspOption (String) 
}

pub fn parse_command(line: &str) -> Option<Command> {
    if line.starts_with("uci") {
        return Some(CmdUci);
    } 
    if line.starts_with("isready") {
        return Some(CmdIsReady);
    }
    if line.starts_with("ucinewgame") {
        return Some(CmdUciNewGame);
    }
    if line.starts_with("stop") {
        return Some(CmdStop);
    }
    if line.starts_with("quit") {
        return Some(CmdQuit);
    }
    if line.starts_with("position") {
        let position_index = match line.find(' ') {
            Some(index) => index + 1,
            None => return None
        };
        let position_str = line.slice_from(position_index);
        let position = if position_str.starts_with("startpos") {
            //initial position
            parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
        } else {
            match parse_fen(position_str) {
                Ok(p) => p,
                Err(e) => {
                    //TODO: log something
                    return None;
                }
            }
        };
        let moves_index = match line.find_str("moves ") {
            Some(index) => index + "moves ".len(),
            None => 0
        };
        let moves:Vec<UciMove> = if moves_index > 0 && moves_index < line.len() {
            parse_moves(line.slice_from(moves_index))
        } else {
            Vec::new()
        };
        return Some(CmdPosition (position, moves))
    }
    unimplemented!();
} 

fn parse_moves(input: &str) -> Vec<UciMove> {
    Vec::<UciMove>::new()
}
