// ███████╗██████╗  █████╗ ██╗    ██╗███╗   ██╗███████╗
// ██╔════╝██╔══██╗██╔══██╗██║    ██║████╗  ██║██╔════╝
// ███████╗██████╔╝███████║██║ █╗ ██║██╔██╗ ██║███████╗
// ╚════██║██╔═══╝ ██╔══██║██║███╗██║██║╚██╗██║╚════██║
// ███████║██║     ██║  ██║╚███╔███╔╝██║ ╚████║███████║
// ╚══════╝╚═╝     ╚═╝  ╚═╝ ╚══╝╚══╝ ╚═╝  ╚═══╝╚══════╝
//
// 1 bat
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
    &[1, 3, 4, 6, 9],          //deep
    &[1, 2, 3, 4, 6],          //shallow
    &[1, 2, 3, 4, 6, 8],       //swamp
    &[1, 2, 3, 6],             //plain
    &[1, 2, 3, 4, 5, 6, 8],    //forest
    &[1, 2, 3, 4, 5, 6, 8],    //darkforest
    &[1, 2, 3, 4, 5, 6, 7],    //hill
    &[1, 2, 4, 5, 6, 7, 8, 9], //mountain
    &[1, 2, 4, 5, 6, 7, 8, 9], //clouds
    &[1, 2, 4, 6, 7, 8, 9],    //peak
    &[1, 6, 7, 9],             //lava
];

pub const SPAWN_ALLOWED: [[bool; 11]; 10] = {
    let mut spawns_allowed = [[false; 11]; 10];
    let mut i = 0;

    while i < 11 {
        let spawns = SPAWNS[i];
        let mut j = 0;
        while j < spawns.len() {
            spawns_allowed[spawns[j]][i] = true;

            j += 1;
        }

        i += 1;
    }

    spawns_allowed
};
