use rand::prelude::*;

pub struct Picker {
    len: usize,
    sum: i32,
    maxes: Vec<i32>,
}

impl Picker {
    pub fn new(w: Vec<i32>) -> Self {
        let len = w.len();
        let sum = w.iter().sum::<i32>();

        let mut running_total = 0;
        let mut maxes = Vec::new();
        for i in 0..len {
            running_total += w[i];

            maxes.push(running_total);
        }

        Picker {
            len: len,
            sum: sum,
            maxes: maxes,
        }
    }

    pub fn pick_index(&self) -> i32 {
        let rand = rand::thread_rng().gen_range(0, self.sum);

        for i in 0..self.len {
            if rand < self.maxes[i] {
                return i as i32;
            }
        }

        -1
    }
}
