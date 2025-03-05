pub static EFFECTIVE: [&[i16]; 10] = [
    &[],           //none
    &[],           //unarmed
    &[],           //sword
    &[7],          //acid
    &[1, 2],       //thirster
    &[2, 4],       //silverfang
    &[3, 5],       //wail
    &[5, 9],       //cleaver
    &[3, 9],       //chilltouch
    &[2, 4, 6, 8], //sunray
];

pub static INEFFECTIVE: [&[i16]; 10] = [
    &[],                       //none
    &[2, 3, 4, 5, 6, 7, 8, 9], //unarmed
    &[6, 7, 8, 9],             //sword
    &[3, 6],                   //acid
    &[2, 4, 6, 8],             //thirster
    &[5, 7, 9],                //silverfang
    &[2, 4, 6, 8],             //wail
    &[6],                      //cleaver
    &[2, 4, 6, 8],             //chilltouch
    &[3, 5, 7, 9],             //sunray
];
