#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
pub mod ch5 {
    use std::collections::HashMap;
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;
    use anyhow::Result;
    use rand::Rng;
    use thiserror::Error;
    use std::thread;
    use std::sync::{Arc, Mutex};
    use std::thread::JoinHandle;

    #[derive(Clone, Copy)]
    struct N {
        n: usize
    }

    #[derive(Debug, Error)]
    #[error("Please supply a value n between 8 and 48 which is a multiple of 8")]
    struct ErrorInRangeAndMultipleOf8;

    impl N {
        fn new(n: usize) -> Result<Self, ErrorInRangeAndMultipleOf8>{
            if n < 8 || n > 48 || n % 8 != 0 {
                return Err(ErrorInRangeAndMultipleOf8);
            }

            Ok(Self { n })
        }
    }

    fn sha_512_n(input: &str, n: N) -> Vec<u8> {
        let mut hasher = Sha512::new();

        let mut res: [u8; 64] = [0;64];

        hasher.input_str(input);
        hasher.result(&mut res);

        res[0..(n.n / 8)].to_vec()
    }

    pub fn ch_5_3(n: usize) -> Result<()>{
        let n =  N::new(n)?;

        let results_map = HashMap::new();
        let mutex = Arc::new(Mutex::new(results_map));

        let mut threads = vec![];

        for j in 0..5 {
            let mutex_clone = mutex.clone();

            let t = thread::spawn(move || {
                loop {
                    let i: u32 = rand::thread_rng().gen();

                    let input = format!("{}", i);

                    let hash = sha_512_n(&input, n);

                    let mut m = mutex_clone.lock().unwrap();

                    let res = m.insert(hash.clone(), input.clone());
                    drop(m);

                    match res {
                        Some(x) => {
                            if input != x {
                                println!("Thread {}: ({}, {}) {:?}", j, input, x, hash);
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            });
            threads.push(t);
        }

        for t in threads {
            t.join().unwrap();
        }

        Ok(())
    }

    #[cfg(test)]
    mod tests5 {
        use super::*;
        #[test]
        fn ch_5_3() {
            let input = "input";
            let blah = sha_512_n(
                &input,
                N {n: 8}
            );

            println!("here {:?}", blah);
        }

    }
}

pub mod ch6 {

}