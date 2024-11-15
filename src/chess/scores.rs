// Source: https://www.chessprogramming.org/Simplified_Evaluation_Function

pub const ENDGAME_THRESHOLD: u32 = 1500 + 20000; // 15 pawns + king

pub const PAWN_SCORES: [i16; 64] = [
    100, 100, 100, 100, 100, 100, 100, 100, 150, 150, 150, 150, 150, 150, 150, 150, 110, 110, 120,
    130, 130, 120, 110, 110, 105, 105, 110, 125, 125, 110, 105, 105, 100, 100, 100, 120, 120, 100,
    100, 100, 105, 95, 90, 100, 100, 90, 95, 105, 105, 110, 110, 80, 80, 110, 110, 105, 100, 100,
    100, 100, 100, 100, 100, 100,
];

pub const KNIGHT_SCORES: [i16; 64] = [
    270, 280, 290, 290, 290, 290, 280, 270, 280, 300, 320, 320, 320, 320, 300, 280, 290, 320, 330,
    335, 335, 330, 320, 290, 290, 325, 335, 340, 340, 335, 325, 290, 290, 320, 335, 340, 340, 335,
    320, 290, 290, 325, 330, 335, 335, 330, 325, 290, 280, 300, 320, 325, 325, 320, 300, 280, 270,
    280, 290, 290, 290, 290, 280, 270,
];

pub const BISHOP_SCORES: [i16; 64] = [
    310, 320, 320, 320, 320, 320, 320, 310, 320, 330, 330, 330, 330, 330, 330, 320, 320, 330, 335,
    340, 340, 335, 330, 320, 320, 335, 335, 340, 340, 335, 335, 320, 320, 330, 340, 340, 340, 340,
    330, 320, 320, 340, 340, 340, 340, 340, 340, 320, 320, 335, 330, 330, 330, 330, 335, 320, 310,
    320, 320, 320, 320, 320, 320, 310,
];

pub const ROOK_SCORES: [i16; 64] = [
    500, 500, 500, 500, 500, 500, 500, 500, 505, 510, 510, 510, 510, 510, 510, 505, 495, 500, 500,
    500, 500, 500, 500, 495, 495, 500, 500, 500, 500, 500, 500, 495, 495, 500, 500, 500, 500, 500,
    500, 495, 495, 500, 500, 500, 500, 500, 500, 495, 495, 500, 500, 500, 500, 500, 500, 495, 500,
    500, 500, 505, 505, 500, 500, 500,
];
pub const QUEEN_SCORES: [i16; 64] = [
    880, 890, 890, 895, 895, 890, 890, 880, 890, 900, 900, 900, 900, 900, 900, 890, 890, 900, 905,
    905, 905, 905, 900, 890, 895, 900, 905, 905, 905, 905, 900, 895, 900, 900, 905, 905, 905, 905,
    900, 895, 890, 905, 905, 905, 905, 905, 900, 890, 890, 900, 905, 900, 900, 900, 900, 890, 880,
    890, 890, 895, 895, 890, 890, 880,
];
pub const KING_SCORES_MIDDLE: [i16; 64] = [
    19970, 19960, 19960, 19950, 19950, 19960, 19960, 19970, 19970, 19960, 19960, 19950, 19950,
    19960, 19960, 19970, 19970, 19960, 19960, 19950, 19950, 19960, 19960, 19970, 19970, 19960,
    19960, 19950, 19950, 19960, 19960, 19970, 19980, 19970, 19970, 19960, 19960, 19970, 19970,
    19980, 19990, 19980, 19980, 19980, 19980, 19980, 19980, 19990, 20020, 20020, 20000, 20000,
    20000, 20000, 20020, 20020, 20020, 20030, 20010, 20000, 20000, 20010, 20030, 20020,
];

pub const KING_SCORES_END: [i16; 64] = [
    19950, 19960, 19970, 19980, 19980, 19970, 19960, 19950, 19970, 19980, 19990, 20000, 20000,
    19990, 19980, 19970, 19970, 19990, 20020, 20030, 20030, 20020, 19990, 19970, 19970, 19990,
    20030, 20040, 20040, 20030, 19990, 19970, 19970, 19990, 20030, 20040, 20040, 20030, 19990,
    19970, 19970, 19990, 20020, 20030, 20030, 20020, 19990, 19970, 19970, 19970, 20000, 20000,
    20000, 20000, 19970, 19970, 19950, 19970, 19970, 19970, 19970, 19970, 19970, 19950,
];