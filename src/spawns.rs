// ███████╗██████╗  █████╗ ██╗    ██╗███╗   ██╗███████╗
// ██╔════╝██╔══██╗██╔══██╗██║    ██║████╗  ██║██╔════╝
// ███████╗██████╔╝███████║██║ █╗ ██║██╔██╗ ██║███████╗
// ╚════██║██╔═══╝ ██╔══██║██║███╗██║██║╚██╗██║╚════██║
// ███████║██║     ██║  ██║╚███╔███╔╝██║ ╚████║███████║
// ╚══════╝╚═╝     ╚═╝  ╚═╝ ╚══╝╚══╝ ╚═╝  ╚═══╝╚══════╝
//
// -1 gloamling
// 0 bat
// 1 wolf
// 2 boney
// 3 saurian
// 4 vampire
// 5 dweomer
// 6 banshee
// 7 goyle
// 8 lich
// 9 dragon

// 0 deep
// 1 shallow
// 2 swamp
// 3 plain
// 4 forest
// 5 darkforest
// 6 hill
// 7 mountain
// 8 clouds
// 9 peak
// 10 lava

pub static SPAWNS: [&[usize]; 11] = [
    &[3, 4, 6, 9],             //deep
    &[2, 3, 4, 6],             //shallow
    &[1, 2, 3, 4, 6, 8],       //swamp
    &[1, 2, 3, 6],             //plain
    &[1, 2, 3, 4, 5, 6, 8],    //forest
    &[1, 2, 3, 4, 5, 6, 8],    //darkforest
    &[1, 2, 3, 4, 5, 6, 7],    //hill
    &[1, 2, 4, 5, 6, 7, 8, 9], //mountain
    &[1, 2, 4, 5, 6, 7, 8, 9], //clouds
    &[1, 2, 4, 6, 7, 8, 9],    //peak
    &[6, 7, 9],                //lava
];

pub static NO_FOLLOW: [&[usize]; 11] = [
    &[], //deep
    &[], //shallow
    &[], //swamp
    &[], //plain
    &[], //forest
    &[], //darkforest
    &[], //hill
    &[], //mountain
    &[], //clouds
    &[], //peak
    &[], //lava
];
