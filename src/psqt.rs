// SPDX-License-Identifier: GPL-3.0-or-later

use std;

use bitboard::*;
use types::*;

const BONUS: [[[Score; 4]; 8]; 6] = [
    [ // Pawn
        [Score::new(0, 0), Score::new(0, 0), Score::new(0, 0), Score::new(0, 0)],
        [Score::new(-11, 7), Score::new(6, -4), Score::new(7, 8), Score::new(3, -2)],
        [Score::new(-18, -4), Score::new(-2, -5), Score::new(19, 5), Score::new(24, 4)],
        [Score::new(-17, 3), Score::new(-9, 3), Score::new(20, -8), Score::new(35, -3)],
        [Score::new(-6, 8), Score::new(5, 9), Score::new(3, 7), Score::new(21, -6)],
        [Score::new(-6, 8), Score::new(-8, -5), Score::new(-6, 2), Score::new(-2, 4)],
        [Score::new(-4, 3), Score::new(20, -9), Score::new(-8, 1), Score::new(-4, 18)],
        [Score::new(0, 0), Score::new(0, 0), Score::new(0, 0), Score::new(0, 0)]
    ],
    [ // Knight
        [Score::new(-161, -105), Score::new(-96, -82), Score::new(-80, -46), Score::new(-73, -14)],
        [Score::new(-83, -69), Score::new(-43, -54), Score::new(-21, -17), Score::new(-10, 9)],
        [Score::new(-71, -50), Score::new(-22, -39), Score::new(0, -7), Score::new(9, 28)],
        [Score::new(-25, -41), Score::new(18, -25), Score::new(43, 6), Score::new(47, 38)],
        [Score::new(-26, -46), Score::new(16, -25), Score::new(38, 3), Score::new(50, 40)],
        [Score::new(-11, -54), Score::new(37, -38), Score::new(56, -7), Score::new(65, 27)],
        [Score::new(-63, -65), Score::new(-19, -50), Score::new(5, -24), Score::new(14, 13)],
        [Score::new(-195, -109), Score::new(-67, -89), Score::new(-42, -50), Score::new(-29, -13)]
    ],
    [ // Bishop
        [Score::new(-44, -58), Score::new(-13, -31), Score::new(-25, -37), Score::new(-34, -19)],
        [Score::new(-20, -34), Score::new(20, -9), Score::new(12, -14), Score::new(1, 4)],
        [Score::new(-9, -23), Score::new(27, 0), Score::new(21, -3), Score::new(11, 16)],
        [Score::new(-11, -26), Score::new(28, -3), Score::new(21, -5), Score::new(10, 16)],
        [Score::new(-11, -26), Score::new(27, -4), Score::new(16, -7), Score::new(9, 14)],
        [Score::new(-17, -24), Score::new(16, -2), Score::new(12, 0), Score::new(2, 13)],
        [Score::new(-23, -34), Score::new(17, -10), Score::new(6, -12), Score::new(-2, 6)],
        [Score::new(-35, -55), Score::new(-11, -32), Score::new(-19, -36), Score::new(-29, -17)]
    ],
    [ // Rook
        [Score::new(-25, 0), Score::new(-16, 0), Score::new(-16, 0), Score::new(-9, 0)],
        [Score::new(-21, 0), Score::new(-8, 0), Score::new(-3, 0), Score::new(0, 0)],
        [Score::new(-21, 0), Score::new(-9, 0), Score::new(-4, 0), Score::new(2, 0)],
        [Score::new(-22, 0), Score::new(-6, 0), Score::new(-1, 0), Score::new(2, 0)],
        [Score::new(-22, 0), Score::new(-7, 0), Score::new(0, 0), Score::new(1, 0)],
        [Score::new(-21, 0), Score::new(-7, 0), Score::new(0, 0), Score::new(2, 0)],
        [Score::new(-12, 0), Score::new(4, 0), Score::new(8, 0), Score::new(12, 0)],
        [Score::new(-23, 0), Score::new(-15, 0), Score::new(-11, 0), Score::new(-5, 0)]
    ],
    [ // Queen
        [Score::new(0, -71), Score::new(-4, -56), Score::new(-3, -42), Score::new(-1, -29)],
        [Score::new(-4, -56), Score::new(6, -30), Score::new(9, -21), Score::new(8, -5)],
        [Score::new(-2, -39), Score::new(6, -17), Score::new(9, -8), Score::new(9, 5)],
        [Score::new(-1, -29), Score::new(8, -5), Score::new(10, 9), Score::new(7, 19)],
        [Score::new(-3, -27), Score::new(9, -5), Score::new(8, 10), Score::new(7, 21)],
        [Score::new(-2, -40), Score::new(6, -16), Score::new(8, -10), Score::new(10, 3)],
        [Score::new(-2, -55), Score::new(7, -30), Score::new(7, -21), Score::new(6, -6)],
        [Score::new(-1, -74), Score::new(-4, -55), Score::new(-1, -43), Score::new(0, -30)]
    ],
    [ // King
        [Score::new(267, 0), Score::new(320, 48), Score::new(270, 75), Score::new(195, 84)],
        [Score::new(264, 43), Score::new(304, 92), Score::new(238, 143), Score::new(180, 132)],
        [Score::new(200, 83), Score::new(245, 138), Score::new(176, 167), Score::new(110, 165)],
        [Score::new(177, 106), Score::new(185, 169), Score::new(148, 169), Score::new(110, 179)],
        [Score::new(149, 108), Score::new(177, 163), Score::new(115, 200), Score::new(66, 203)],
        [Score::new(118, 95), Score::new(159, 155), Score::new(84, 176), Score::new(41, 174)],
        [Score::new(87, 50), Score::new(128, 99), Score::new(63, 122), Score::new(20, 139)],
        [Score::new(63, 9), Score::new(88, 55), Score::new(47, 80), Score::new(0, 90)]
    ]
];

static mut PSQ: [[Score; 64]; 16] = [[Score(0); 64]; 16];

pub fn psq(pc: Piece, s: Square) -> Score {
    unsafe { PSQ[pc.0 as usize][s.0 as usize] }
}

pub fn init() {
    unsafe {
        for i in 1..7 {
            let pc = Piece(i);
            let v = Score::new(piece_value(MG, pc).0, piece_value(EG, pc).0);

            for s in ALL_SQUARES {
                let f = std::cmp::min(s.file(), FILE_H - s.file());
                PSQ[pc.0 as usize][s.0 as usize] = v
                    + BONUS[(pc.0 - 1) as usize][s.rank() as usize][f as usize];
                PSQ[(!pc).0 as usize][(!s).0 as usize] =
                    -PSQ[pc.0 as usize][s.0 as usize];
            }
        }
    }
}
