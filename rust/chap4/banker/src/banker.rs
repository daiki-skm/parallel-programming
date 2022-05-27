use std::sync::{Arc, Mutex};

struct Resource<const NRES: usize, const NTH: usize> {
    available: [usize; NRES],
    allocation: [[usize; NRES]; NTH],
    max: [[usize; NRES]; NTH],
}

impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
    fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Resource {
            available,
            allocation: [[0; NRES]; NTH],
            max,
        }
    }

    fn is_safe(&self) -> bool {
        let mut finish = [false; NTH];
        let mut work = self.available.clone();

        loop {
            let mut found = false;
            let mut num_true = 0;
            for (i, alc) in self.allocation.iter().enumerate() {
                if finish[i] {
                    num_true += 1;
                    continue;
                }

                let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
                let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
                if is_avail {
                    found = true;
                    finish[i] = true;
                    for (w, a) in work.iter_mut().zip(alc) {
                        *w += *a;
                    }
                    break;
                }
            }

            if num_true == NTH {
                return true;
            }

            if !found {
                break;
            }
        }

        false
    }

    fn take(&mut self, id: usize, resource: usize) -> bool {
        if id >= NTH || resource >= NRES || self.available[resource] == 0 {
            return false;
        }

        self.allocation[id][resource] += 1;
        self.available[resource] -= 1;

        if self.is_safe() {
            true
        } else {
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
            false
        }
    }

    fn release(&mut self, id: usize, resource: usize) {
        if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
            return;
        }

        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
    }
}

#[derive(Clone)]
pub struct Banker<const NRES: usize, const NTH: usize> {
    resource: Arc<Mutex<Resource<NRES, NTH>>>,
}

impl<const NRES: usize, const NTH: usize> Banker<NRES, NTH> {
    pub fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Banker {
            resource: Arc::new(Mutex::new(Resource::new(available, max))),
        }
    }

    pub fn take(&self, id: usize, resource: usize) -> bool {
        let mut r = self.resource.lock().unwrap();
        r.take(id, resource)
    }

    pub fn release(&self, id: usize, release: usize) {
        let mut r = self.resource.lock().unwrap();
        r.release(id, release);
    }
}