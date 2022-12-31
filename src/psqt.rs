// SPDX-License-Identifier: GPL-3.0-or-later

use std;

use bitboard::*;
use types::*;

const _BONUS: [[[Score; 4]; 8]; 6] = [
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

const PSQ: [[Score; 64]; 16] = [
    [
        Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), ],
    [
        Score(15728811), Score(15728811), Score(15728811), Score(15728811), Score(15728811), Score(15728811), Score(15728811), Score(15728811), Score(16187552), Score(15466673), Score(16253106), Score(15597742), Score(15597742), Score(16253106), Score(15466673), Score(16187552), Score(15466649), Score(15401129), Score(16056510), Score(15990979), Score(15990979), Score(16056510), Score(15401129), Score(15466649), Score(15925402), Score(15925410), Score(15204543), Score(15532238), Score(15532238), Score(15204543), Score(15925410), Score(15925402), Score(16253093), Score(16318640), Score(16187566), Score(15335616), Score(15335616), Score(16187566), Score(16318640), Score(16253093), Score(16253093), Score(15401123), Score(15859877), Score(15990953), Score(15990953), Score(15859877), Score(15401123), Score(16253093), Score(15925415), Score(15139007), Score(15794339), Score(16908455), Score(16908455), Score(15794339), Score(15139007), Score(15925415), Score(15728811), Score(15728811), Score(15728811), Score(15728811), Score(15728811), Score(15728811), Score(15728811), Score(15728811), ],
    [
        Score(48693851), Score(50201244), Score(52560556), Score(54657715), Score(54657715), Score(52560556), Score(50201244), Score(48693851), Score(51053225), Score(52036305), Score(54461159), Score(56165106), Score(56165106), Score(54461159), Score(52036305), Score(51053225), Score(52298421), Score(53019366), Score(55116540), Score(57410309), Score(57410309), Score(55116540), Score(53019366), Score(52298421), Score(52888291), Score(53936910), Score(55968551), Score(58065707), Score(58065707), Score(55968551), Score(53936910), Score(52888291), Score(52560610), Score(53936908), Score(55771938), Score(58196782), Score(58196782), Score(55771938), Score(53936908), Score(52560610), Score(52036337), Score(53084961), Score(55116596), Score(57344829), Score(57344829), Score(55116596), Score(53084961), Score(52036337), Score(51315389), Score(52298473), Score(54002433), Score(56427274), Score(56427274), Score(54002433), Score(52298473), Score(51315389), Score(48431673), Score(49742521), Score(52298450), Score(54723295), Score(54723295), Score(52298450), Score(49742521), Score(48431673), ],
    [
        Score(54592270), Score(56361773), Score(55968545), Score(57148184), Score(57148184), Score(55968545), Score(56361773), Score(54592270), Score(56165158), Score(57803598), Score(57475910), Score(58655547), Score(58655547), Score(57475910), Score(57803598), Score(56165158), Score(56886065), Score(58393429), Score(58196815), Score(59441989), Score(59441989), Score(58196815), Score(58393429), Score(56886065), Score(56689455), Score(58196822), Score(58065743), Score(59441988), Score(59441988), Score(58065743), Score(58196822), Score(56689455), Score(56689455), Score(58131285), Score(57934666), Score(59310915), Score(59310915), Score(57934666), Score(58131285), Score(56689455), Score(56820521), Score(58262346), Score(58393414), Score(59245372), Score(59245372), Score(58393414), Score(58262346), Score(56820521), Score(56165155), Score(57738059), Score(57606976), Score(58786616), Score(58786616), Score(57606976), Score(57738059), Score(56165155), Score(54788887), Score(56296239), Score(56034087), Score(57279261), Score(57279261), Score(56034087), Score(56296239), Score(54788887), ],
    [
        Score(89982185), Score(89982194), Score(89982194), Score(89982201), Score(89982201), Score(89982194), Score(89982194), Score(89982185), Score(89982189), Score(89982202), Score(89982207), Score(89982210), Score(89982210), Score(89982207), Score(89982202), Score(89982189), Score(89982189), Score(89982201), Score(89982206), Score(89982212), Score(89982212), Score(89982206), Score(89982201), Score(89982189), Score(89982188), Score(89982204), Score(89982209), Score(89982212), Score(89982212), Score(89982209), Score(89982204), Score(89982188), Score(89982188), Score(89982203), Score(89982210), Score(89982211), Score(89982211), Score(89982210), Score(89982203), Score(89982188), Score(89982189), Score(89982203), Score(89982210), Score(89982212), Score(89982212), Score(89982210), Score(89982203), Score(89982189), Score(89982198), Score(89982214), Score(89982218), Score(89982222), Score(89982222), Score(89982218), Score(89982214), Score(89982198), Score(89982187), Score(89982195), Score(89982199), Score(89982205), Score(89982205), Score(89982199), Score(89982195), Score(89982187), ],
    [
        Score(168757726), Score(169740762), Score(170658267), Score(171510237), Score(171510237), Score(170658267), Score(169740762), Score(168757726), Score(169740762), Score(171444708), Score(172034535), Score(173083110), Score(173083110), Score(172034535), Score(171444708), Score(169740762), Score(170854876), Score(172296676), Score(172886503), Score(173738471), Score(173738471), Score(172886503), Score(172296676), Score(170854876), Score(171510237), Score(173083110), Score(174000616), Score(174655973), Score(174655973), Score(174000616), Score(173083110), Score(171510237), Score(171641307), Score(173083111), Score(174066150), Score(174787045), Score(174787045), Score(174066150), Score(173083111), Score(171641307), Score(170789340), Score(172362212), Score(172755430), Score(173607400), Score(173607400), Score(172755430), Score(172362212), Score(170789340), Score(169806300), Score(171444709), Score(172034533), Score(173017572), Score(173017572), Score(172034533), Score(171444709), Score(169806300), Score(168561117), Score(169806298), Score(170592733), Score(171444702), Score(171444702), Score(170592733), Score(169806298), Score(168561117), ],
    [
        Score(267), Score(3146048), Score(4915470), Score(5505219), Score(5505219), Score(4915470), Score(3146048), Score(267), Score(2818312), Score(6029616), Score(9371886), Score(8650932), Score(8650932), Score(9371886), Score(6029616), Score(2818312), Score(5439688), Score(9044213), Score(10944688), Score(10813550), Score(10813550), Score(10944688), Score(9044213), Score(5439688), Score(6946993), Score(11075769), Score(11075732), Score(11731054), Score(11731054), Score(11075732), Score(11075769), Score(6946993), Score(7078037), Score(10682545), Score(13107315), Score(13303874), Score(13303874), Score(13107315), Score(10682545), Score(7078037), Score(6226038), Score(10158239), Score(11534420), Score(11403305), Score(11403305), Score(11534420), Score(10158239), Score(6226038), Score(3276887), Score(6488192), Score(7995455), Score(9109524), Score(9109524), Score(7995455), Score(6488192), Score(3276887), Score(589887), Score(3604568), Score(5242927), Score(5898240), Score(5898240), Score(5242927), Score(3604568), Score(589887), ],
    [
        Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), ],
    [
        Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), ],
    [
        Score(-15728811), Score(-15728811), Score(-15728811), Score(-15728811), Score(-15728811), Score(-15728811), Score(-15728811), Score(-15728811), Score(-15925415), Score(-15139007), Score(-15794339), Score(-16908455), Score(-16908455), Score(-15794339), Score(-15139007), Score(-15925415), Score(-16253093), Score(-15401123), Score(-15859877), Score(-15990953), Score(-15990953), Score(-15859877), Score(-15401123), Score(-16253093), Score(-16253093), Score(-16318640), Score(-16187566), Score(-15335616), Score(-15335616), Score(-16187566), Score(-16318640), Score(-16253093), Score(-15925402), Score(-15925410), Score(-15204543), Score(-15532238), Score(-15532238), Score(-15204543), Score(-15925410), Score(-15925402), Score(-15466649), Score(-15401129), Score(-16056510), Score(-15990979), Score(-15990979), Score(-16056510), Score(-15401129), Score(-15466649), Score(-16187552), Score(-15466673), Score(-16253106), Score(-15597742), Score(-15597742), Score(-16253106), Score(-15466673), Score(-16187552), Score(-15728811), Score(-15728811), Score(-15728811), Score(-15728811), Score(-15728811), Score(-15728811), Score(-15728811), Score(-15728811), ],
    [
        Score(-48431673), Score(-49742521), Score(-52298450), Score(-54723295), Score(-54723295), Score(-52298450), Score(-49742521), Score(-48431673), Score(-51315389), Score(-52298473), Score(-54002433), Score(-56427274), Score(-56427274), Score(-54002433), Score(-52298473), Score(-51315389), Score(-52036337), Score(-53084961), Score(-55116596), Score(-57344829), Score(-57344829), Score(-55116596), Score(-53084961), Score(-52036337), Score(-52560610), Score(-53936908), Score(-55771938), Score(-58196782), Score(-58196782), Score(-55771938), Score(-53936908), Score(-52560610), Score(-52888291), Score(-53936910), Score(-55968551), Score(-58065707), Score(-58065707), Score(-55968551), Score(-53936910), Score(-52888291), Score(-52298421), Score(-53019366), Score(-55116540), Score(-57410309), Score(-57410309), Score(-55116540), Score(-53019366), Score(-52298421), Score(-51053225), Score(-52036305), Score(-54461159), Score(-56165106), Score(-56165106), Score(-54461159), Score(-52036305), Score(-51053225), Score(-48693851), Score(-50201244), Score(-52560556), Score(-54657715), Score(-54657715), Score(-52560556), Score(-50201244), Score(-48693851), ],
    [
        Score(-54788887), Score(-56296239), Score(-56034087), Score(-57279261), Score(-57279261), Score(-56034087), Score(-56296239), Score(-54788887), Score(-56165155), Score(-57738059), Score(-57606976), Score(-58786616), Score(-58786616), Score(-57606976), Score(-57738059), Score(-56165155), Score(-56820521), Score(-58262346), Score(-58393414), Score(-59245372), Score(-59245372), Score(-58393414), Score(-58262346), Score(-56820521), Score(-56689455), Score(-58131285), Score(-57934666), Score(-59310915), Score(-59310915), Score(-57934666), Score(-58131285), Score(-56689455), Score(-56689455), Score(-58196822), Score(-58065743), Score(-59441988), Score(-59441988), Score(-58065743), Score(-58196822), Score(-56689455), Score(-56886065), Score(-58393429), Score(-58196815), Score(-59441989), Score(-59441989), Score(-58196815), Score(-58393429), Score(-56886065), Score(-56165158), Score(-57803598), Score(-57475910), Score(-58655547), Score(-58655547), Score(-57475910), Score(-57803598), Score(-56165158), Score(-54592270), Score(-56361773), Score(-55968545), Score(-57148184), Score(-57148184), Score(-55968545), Score(-56361773), Score(-54592270), ],
    [
        Score(-89982187), Score(-89982195), Score(-89982199), Score(-89982205), Score(-89982205), Score(-89982199), Score(-89982195), Score(-89982187), Score(-89982198), Score(-89982214), Score(-89982218), Score(-89982222), Score(-89982222), Score(-89982218), Score(-89982214), Score(-89982198), Score(-89982189), Score(-89982203), Score(-89982210), Score(-89982212), Score(-89982212), Score(-89982210), Score(-89982203), Score(-89982189), Score(-89982188), Score(-89982203), Score(-89982210), Score(-89982211), Score(-89982211), Score(-89982210), Score(-89982203), Score(-89982188), Score(-89982188), Score(-89982204), Score(-89982209), Score(-89982212), Score(-89982212), Score(-89982209), Score(-89982204), Score(-89982188), Score(-89982189), Score(-89982201), Score(-89982206), Score(-89982212), Score(-89982212), Score(-89982206), Score(-89982201), Score(-89982189), Score(-89982189), Score(-89982202), Score(-89982207), Score(-89982210), Score(-89982210), Score(-89982207), Score(-89982202), Score(-89982189), Score(-89982185), Score(-89982194), Score(-89982194), Score(-89982201), Score(-89982201), Score(-89982194), Score(-89982194), Score(-89982185), ],
    [
        Score(-168561117), Score(-169806298), Score(-170592733), Score(-171444702), Score(-171444702), Score(-170592733), Score(-169806298), Score(-168561117), Score(-169806300), Score(-171444709), Score(-172034533), Score(-173017572), Score(-173017572), Score(-172034533), Score(-171444709), Score(-169806300), Score(-170789340), Score(-172362212), Score(-172755430), Score(-173607400), Score(-173607400), Score(-172755430), Score(-172362212), Score(-170789340), Score(-171641307), Score(-173083111), Score(-174066150), Score(-174787045), Score(-174787045), Score(-174066150), Score(-173083111), Score(-171641307), Score(-171510237), Score(-173083110), Score(-174000616), Score(-174655973), Score(-174655973), Score(-174000616), Score(-173083110), Score(-171510237), Score(-170854876), Score(-172296676), Score(-172886503), Score(-173738471), Score(-173738471), Score(-172886503), Score(-172296676), Score(-170854876), Score(-169740762), Score(-171444708), Score(-172034535), Score(-173083110), Score(-173083110), Score(-172034535), Score(-171444708), Score(-169740762), Score(-168757726), Score(-169740762), Score(-170658267), Score(-171510237), Score(-171510237), Score(-170658267), Score(-169740762), Score(-168757726), ],
    [
        Score(-589887), Score(-3604568), Score(-5242927), Score(-5898240), Score(-5898240), Score(-5242927), Score(-3604568), Score(-589887), Score(-3276887), Score(-6488192), Score(-7995455), Score(-9109524), Score(-9109524), Score(-7995455), Score(-6488192), Score(-3276887), Score(-6226038), Score(-10158239), Score(-11534420), Score(-11403305), Score(-11403305), Score(-11534420), Score(-10158239), Score(-6226038), Score(-7078037), Score(-10682545), Score(-13107315), Score(-13303874), Score(-13303874), Score(-13107315), Score(-10682545), Score(-7078037), Score(-6946993), Score(-11075769), Score(-11075732), Score(-11731054), Score(-11731054), Score(-11075732), Score(-11075769), Score(-6946993), Score(-5439688), Score(-9044213), Score(-10944688), Score(-10813550), Score(-10813550), Score(-10944688), Score(-9044213), Score(-5439688), Score(-2818312), Score(-6029616), Score(-9371886), Score(-8650932), Score(-8650932), Score(-9371886), Score(-6029616), Score(-2818312), Score(-267), Score(-3146048), Score(-4915470), Score(-5505219), Score(-5505219), Score(-4915470), Score(-3146048), Score(-267), ],
    [
        Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), Score(0), ],
]
;

pub fn psq(pc: Piece, s: Square) -> Score {
    PSQ[pc.0 as usize][s.0 as usize]
}

pub fn _generate_scores() {
    let mut temp_psq = [[Score(0); 64]; 16];

    for i in 1..7 {
        let pc = Piece(i);
        let v = Score::new(piece_value(MG, pc).0, piece_value(EG, pc).0);

        for s in ALL_SQUARES {
            let f = std::cmp::min(s.file(), FILE_H - s.file());
            temp_psq[pc.0 as usize][s.0 as usize] = v + _BONUS[(pc.0 - 1) as usize][s.rank() as
                usize][f as usize];
            temp_psq[(!pc).0 as usize][(!s).0 as usize] = -temp_psq[pc.0 as usize][s.0 as usize];
        }
    }

    println!("[");
    for scores in temp_psq {
        println!("[");

        for score in scores {
            print!("Score({}),", score.0);
        }
        println!("],");
    }
    println!("]");
}
