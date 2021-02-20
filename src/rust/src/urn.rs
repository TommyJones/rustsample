use rand::distributions::{Distribution, Uniform};


pub struct Urn {
    pub cats: Vec<u64>, // categories of alias table
    pub v: Vec<f64>, // alias probabilities
}

impl Urn {
    // makes a new alias based on probabilities
    pub fn new(probs: &std::vec::Vec<f64>) -> Urn {
        // TODO: check input and fail gracefully
        // needs to be longer than one value
        // needs to not contain any NaN values

        // initialize some useful values
        let n: u64 = probs.len() as u64;

        let a: f64 = 1.0 / (n as f64);

        // get a normalized probability vector
        let norm: f64 = probs.iter().sum();

        let mut pp = Vec::new();

        for j in 0..n as usize {
            pp.push(probs[j] / norm);
        }

        // initialize alias "table" (actually 2 vectors)
        let mut cats = Vec::new(); // alias category index
        let mut v = Vec::new(); // alias probability

        for j in 0..n {
            cats.push(j);
            v.push((j as f64 + 1.0) * a);
        }

        // allocate "table" (actually 2 vectors)
        // do n-2 times, k itself is irrelevant
        for _k in 0..(n - 1) {
            // find indices of smallest and largest probs
            let minval = pp
                .iter()
                .cloned()
                .fold(0./0., f64::min);

            let maxval = pp
                .iter()
                .cloned()
                .fold(0./0., f64::max);

            let i = pp
                .iter()
                .position(|&x| x == minval)
                .unwrap();

            let j = pp
                .iter()
                .position(|&x| x == maxval)
                .unwrap();

            // assign and shuffle
            cats[i] = j as u64;
            v[i] = i as f64 * a + pp[i];

            pp[j] = pp[j] - (a - pp[i]);
            pp[i] = a;
        }

        let result = Urn { cats: cats, v: v };

        return result;
    }

    // samples from the existing alias
    pub fn sample(self, n: u64) -> Vec<u64> {
        // get number of categories
        let k: u64 = self.cats.len() as u64;

        // sample loop. parallelize this if you will...
        let mut result = Vec::new();

        let mut rng = rand::thread_rng();
        let die = Uniform::from(0.0..=1.0);

        for _i in 0..n {
            let u: f64 = die.sample(&mut rng);

            let j: usize = (u * k as f64).floor() as usize;

            if u < self.v[j] {
                result.push(j as u64);
            } else {
                result.push(self.cats[j]);
            }
        }

        return result;
    }
}
