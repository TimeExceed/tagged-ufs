use super::*;
use quickcheck_macros::*;

#[quickcheck]
fn add_connect_query(adds: Vec<u8>, connects: Vec<(u8, u8)>, queries: Vec<(u8, u8)>) {
    let mut trial = UnionFindSets::new();
    let mut oracle = Oracle::new();

    for x in adds.into_iter() {
        let trial_res = trial.make_set(x, ());
        let oracle_res = oracle.make_set(x);
        assert_eq!(trial_res.is_ok(), oracle_res.is_ok());
    }

    for (x, y) in connects.into_iter() {
        match (trial.unite(&x, &y), oracle.unite(x, y)) {
            (Err(_), Err(_)) | (Ok(true), Ok(true)) | (Ok(false), Ok(false)) => (),
            (trial_res, oracle_res) => {
                panic!(
                    "differences:\
                    \n  oracle result: {:?}\
                    \n  trial result: {:?}",
                    trial_res, oracle_res,
                );
            }
        }
    }

    for (x, y) in queries.into_iter() {
        let trial_set_x = trial.find(&x);
        let trial_set_y = trial.find(&y);
        let oralce_set_x = oracle.find(&x);
        let oracle_set_y = oracle.find(&y);

        assert_eq!(trial_set_x.is_none(), oralce_set_x.is_none());
        assert_eq!(trial_set_y.is_none(), oracle_set_y.is_none());
        if let (Some(trial_set_x), Some(trial_set_y)) = (trial_set_x, trial_set_y) {
            let oracle_set_x = oralce_set_x.unwrap();
            let oracle_set_y = oracle_set_y.unwrap();
            assert_eq!(trial_set_x == trial_set_y, oracle_set_x == oracle_set_y);
        }
    }
}

pub(crate) struct Oracle {
    sets: Vec<Vec<u8>>,
}

impl Oracle {
    pub(crate) fn new() -> Self {
        Self { sets: vec![] }
    }

    pub(crate) fn make_set(&mut self, key: u8) -> anyhow::Result<()> {
        for xs in self.sets.iter() {
            if xs.contains(&key) {
                anyhow::bail!("duplicated key");
            }
        }
        self.sets.push(vec![key]);
        Ok(())
    }

    pub(crate) fn unite(&mut self, key1: u8, key2: u8) -> anyhow::Result<bool> {
        let mut key1_set = self.pop(key1)?;
        if key1_set.contains(&key2) {
            self.sets.push(key1_set);
            return Ok(false);
        }
        let mut key2_set = match self.pop(key2) {
            Ok(x) => x,
            Err(err) => {
                self.sets.push(key1_set);
                return Err(err);
            }
        };
        assert!(!key2_set.contains(&key1));
        key1_set.append(&mut key2_set);
        self.sets.push(key1_set);
        Ok(true)
    }

    pub(crate) fn find(&self, key: &u8) -> Option<Vec<u8>> {
        for xs in self.sets.iter() {
            if xs.contains(key) {
                return Some(xs.clone());
            }
        }
        None
    }

    fn pop(&mut self, key: u8) -> anyhow::Result<Vec<u8>> {
        let key_index = (|| {
            for (i, xs) in self.sets.iter().enumerate() {
                if xs.contains(&key) {
                    return Some(i);
                }
            }
            None
        })();
        if let Some(key_index) = key_index {
            Ok(self.sets.swap_remove(key_index))
        } else {
            anyhow::bail!("Cannot find the set.");
        }
    }
}
