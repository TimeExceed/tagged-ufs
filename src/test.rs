use super::*;
use crate::raw::test::Oracle;
use quickcheck_macros::*;

#[quickcheck]
fn add_connect_query(adds: Vec<u8>, connects: Vec<(u8, u8)>, queries: Vec<u8>) {
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

    for x in queries.into_iter() {
        let trial_set = trial.find(&x);
        let oracle_set = oracle.find(&x);

        assert_eq!(trial_set.is_none(), oracle_set.is_none());
        if let (Some(trial_set), Some(mut oracle_set)) = (trial_set, oracle_set) {
            let mut trial_set: Vec<_> = trial_set.iter().copied().collect();
            trial_set.sort();
            oracle_set.sort();
            assert_eq!(trial_set, oracle_set);
        }
    }
}
