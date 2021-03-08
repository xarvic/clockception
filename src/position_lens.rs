use druid::Lens;
use std::f64::consts::PI;

pub struct PositionalLens(usize);

impl PositionalLens {
    pub fn new(x: u8, y: u8) -> Self {
        PositionalLens((x + 5 * y) as usize)
    }
    pub fn get(&self, value: u8) -> (f64, f64) {
        let (x, y) = match value {
            0 => POS_0[self.0],
            1 => POS_1[self.0],
            2 => POS_2[self.0],
            3 => POS_3[self.0],
            4 => POS_4[self.0],
            5 => POS_5[self.0],
            6 => POS_6[self.0],
            7 => POS_7[self.0],
            8 => POS_8[self.0],
            9 => POS_9[self.0],
            255 => POS_IDLE[self.0],
            _ => (0.0, 0.0)
        };

        const RATIO: f64 = PI / 2.0;

        (x * RATIO, y * RATIO)
    }
}

impl Lens<u8, (f64, f64)> for PositionalLens {
    fn with<V, F: FnOnce(&(f64, f64)) -> V>(&self, data: &u8, f: F) -> V {
        let inner = self.get(*data);
        f(&inner)
    }

    fn with_mut<V, F: FnOnce(&mut (f64, f64)) -> V>(&self, data: &mut u8, f: F) -> V {
        let mut inner = self.get(*data);
        f(&mut inner)
    }
}

const ID: (f64, f64) = (1.5, 1.5); // IDLE
const HO: (f64, f64) = (0.0, 2.0); // HORIZONTAL
const VE: (f64, f64) = (1.0, 3.0); // VERTICAL
const LT: (f64, f64) = (2.0, 3.0); // LEFT-TOP
const LB: (f64, f64) = (2.0, 1.0); // LEFT-BOTTOM
const RT: (f64, f64) = (0.0, 3.0); // RIGHT-TOP
const RB: (f64, f64) = (0.0, 1.0); // RIGHT-BOTTOM

const POS_IDLE: [(f64, f64); 30] = [
    ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID,
    ID, ID, ID, ID, ID,
];

const POS_0: [(f64, f64); 30] = [
    RB, HO, HO, HO, LB,
    VE, RB, HO, LB, VE,
    VE, VE, ID, VE, VE,
    VE, VE, ID, VE, VE,
    VE, RT, HO, LT, VE,
    RT, HO, HO, HO, LT,
];

const POS_1: [(f64, f64); 30] = [
    ID, RB, HO, LB, ID,
    ID, RT, LB, VE, ID,
    ID, ID, VE, VE, ID,
    ID, ID, VE, VE, ID,
    ID, ID, VE, VE, ID,
    ID, ID, RT, LT, ID,
];

const POS_2: [(f64, f64); 30] = [
    RB, HO, HO, HO, LB,
    RT, HO, HO, LB, VE,
    RB, HO, HO, LT, VE,
    VE, RB, HO, HO, LT,
    VE, RT, HO, HO, LB,
    RT, HO, HO, HO, LT,
];

const POS_3: [(f64, f64); 30] = [
    RB, HO, HO, HO, LB,
    RT, HO, HO, LB, VE,
    RB, HO, HO, LT, VE,
    RT, HO, HO, LB, VE,
    RB, HO, HO, LT, VE,
    RT, HO, HO, HO, LT,
];

const POS_4: [(f64, f64); 30] = [
    RB, LB, ID, ID, ID,
    VE, VE, ID, ID, ID,
    VE, VE, RB, LB, ID,
    VE, RT, LT, RT, LB,
    RT, HO, LB, RB, LT,
    ID, ID, RT, LT, ID,
];

const POS_5: [(f64, f64); 30] = [
    RB, HO, HO, HO, LB,
    VE, RB, HO, HO, LT,
    VE, RT, HO, HO, LB,
    RT, HO, HO, LB, VE,
    RB, HO, HO, LT, VE,
    RT, HO, HO, HO, LT,
];

const POS_6: [(f64, f64); 30] = [
    RB, HO, HO, HO, LB,
    VE, RB, HO, HO, LT,
    VE, RT, HO, HO, LB,
    VE, RB, HO, LB, VE,
    VE, RT, HO, LT, VE,
    RT, HO, HO, HO, LT,
];

const POS_7: [(f64, f64); 30] = [
    RB, HO, HO, HO, LB,
    RT, HO, HO, (2.0, 1.5), (3.0, 1.5),
    ID, ID, (1.0, 3.5), (1.0, 3.5), ID,
    ID, ID, VE, VE, ID,
    ID, ID, VE, VE, ID,
    ID, ID, RT, LT, ID,
];

const POS_8: [(f64, f64); 30] = [
    RB, HO, HO, HO, LB,
    VE, RB, HO, LB, VE,
    VE, RT, HO, LT, VE,
    VE, RB, HO, LB, VE,
    VE, RT, HO, LT, VE,
    RT, HO, HO, HO, LT,
];

const POS_9: [(f64, f64); 30] = [
    RB, HO, HO, HO, LB,
    VE, RB, HO, LB, VE,
    VE, RT, HO, LT, VE,
    RT, HO, HO, LB, VE,
    RB, HO, HO, LT, VE,
    RT, HO, HO, HO, LT,
];
