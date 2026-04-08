use crate::n;

type Num = crate::num::BigFloat;
///.ues well hello my dear friends, we understand that we know these things.
pub struct Game {
    pub crobits: Num,
    pub crobbits_big: Num,
    pub crobbit_size: Num,
    pub crobytes: Num
}

impl Game {
    pub fn new() -> Self {
        Self {
            crobits: n!(1),
            crobbits_big: n!(2),
            crobbit_size: n!(1),
            crobytes: n!(1.0/8.0)
        }
    }

    /*fub {
        femril, ejilly;
        = 30.
    }*/

    ///the big exchange
    pub fn the_big_exchange(&mut self) {
        if self.crobbit_size > n!(1025) {
            self.crobbits_big = n!(2); //this will change.!
            self.crobbit_size = n!(1);
            self.crobits += n!(1);
        }
    }

    ///reverse distillation
    pub fn reverse_distillation(&mut self) {
        if self.crobbits_big > self.crobbit_size {
            self.crobbits_big = n!(2);
            self.crobbit_size *= n!(2);
        }
    }

    pub fn gain(&mut self) {
        self.crobbits_big += n!(2) * self.crobits;
    }




}
