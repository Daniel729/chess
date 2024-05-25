use super::*;

// Documentation for perft: https://www.chessprogramming.org/Perft_Results

/// Performance Test
fn perft(game: &mut ChessGame, depth: u8) -> usize {
    let mut moves = ArrayVec::new();
    game.get_moves(&mut moves, true);

    if depth == 1 {
        return moves.len();
    }

    let mut count = 0;

    for _move in moves.iter() {
        let _move = *_move;
        game.push(_move);
        count += perft(game, depth - 1);
        game.pop(_move);
    }

    count
}

#[test]
fn perft1_startpos() {
    let mut game = ChessGame::new();
    assert_eq!(perft(&mut game, 1), 20);
}

#[test]
fn perft2_startpos() {
    let mut game = ChessGame::new();
    assert_eq!(perft(&mut game, 2), 400);
}

#[test]
fn perft3_startpos() {
    let mut game = ChessGame::new();
    assert_eq!(perft(&mut game, 3), 8902);
}

#[test]
fn perft4_startpos() {
    let mut game = ChessGame::new();
    assert_eq!(perft(&mut game, 4), 197281);
}

#[test]
fn perft5_startpos() {
    let mut game = ChessGame::new();
    assert_eq!(perft(&mut game, 5), 4865609);
}

#[test]
fn perft1_kiwipete() {
    let mut game =
        ChessGame::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
            .unwrap();
    assert_eq!(perft(&mut game, 1), 48);
}

#[test]
fn perft2_kiwipete() {
    let mut game =
        ChessGame::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
            .unwrap();
    assert_eq!(perft(&mut game, 2), 2039);
}

#[test]
fn perft3_kiwipete() {
    let mut game =
        ChessGame::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
            .unwrap();
    assert_eq!(perft(&mut game, 3), 97862);
}

#[test]
fn perft4_kiwipete() {
    let mut game =
        ChessGame::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -")
            .unwrap();
    assert_eq!(perft(&mut game, 4), 4085603);
}

#[test]
fn perft1_position_3() {
    let mut game = ChessGame::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ").unwrap();
    assert_eq!(perft(&mut game, 1), 14);
}

#[test]
fn perft2_position_3() {
    let mut game = ChessGame::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ").unwrap();
    assert_eq!(perft(&mut game, 2), 191);
}

#[test]
fn perft3_position_3() {
    let mut game = ChessGame::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ").unwrap();
    assert_eq!(perft(&mut game, 3), 2812);
}

#[test]
fn perft4_position_3() {
    let mut game = ChessGame::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ").unwrap();
    assert_eq!(perft(&mut game, 4), 43238);
}

#[test]
fn perft5_position_3() {
    let mut game = ChessGame::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ").unwrap();
    assert_eq!(perft(&mut game, 5), 674624);
}

#[test]
fn perft1_position_5() {
    let mut game =
        ChessGame::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    assert_eq!(perft(&mut game, 1), 44);
}

#[test]
fn perft2_position_5() {
    let mut game =
        ChessGame::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    assert_eq!(perft(&mut game, 2), 1486);
}

#[test]
fn perft3_position_5() {
    let mut game =
        ChessGame::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    assert_eq!(perft(&mut game, 3), 62379);
}

#[test]
fn perft4_position_5() {
    let mut game =
        ChessGame::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();
    assert_eq!(perft(&mut game, 4), 2103487);
}