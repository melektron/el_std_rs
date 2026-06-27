/*
ELEKTRON © 2026 - now
Written by melektron
www.elektron.work
26.06.26, 12:18
All rights reserved.

This source code is licensed under the Apache-2.0 license found in the
LICENSE file in the root directory of this source tree.
*/

//! Additional methods for std::sync::poison::Mutex and similar to
//! intentfully unwrap lock results.

pub trait PoisonlessLock<T> {
    /// locks an [`std::sync::poison::Mutex`] and always returns 
    /// locked mutex guard (equivalent to `.lock().unwrap()`).
    /// Use of this method is preferred over `.lock().unwrap()` as it
    /// explicitly states that mutex poisoning should be treated as 
    /// unrecoverable, where as `.unwrap()` doesn't rule out plain
    /// laziness to handle the poison error.
    /// 
    /// # Panics
    /// 
    /// Panics if the mutex is poisoned. 
    fn poisonless_lock(&self) -> std::sync::MutexGuard<'_, T>;
}

impl<T> PoisonlessLock<T> for std::sync::Mutex<T> {
    fn poisonless_lock(&self) -> std::sync::MutexGuard<'_, T> {
        self.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread::sleep,
        time::{Duration, Instant},
    };

    use super::*;

    #[test]
    fn locking_works() {
        let guarded_string = Arc::new(Mutex::new("before".to_owned()));

        let before_thread = Instant::now();

        std::thread::spawn({
            let guarded_string = guarded_string.clone();
            move || {
                let mut string = guarded_string.poisonless_lock();
                sleep(Duration::from_millis(200));
                *string = "modified".to_owned();
            }
        });

        sleep(Duration::from_millis(100));
        // string should be modified by thread
        let string = guarded_string.poisonless_lock();
        assert_eq!(*string, "modified");
        // locking should take at least 200 ms because the thread holds it for that time
        let elapsed_millis = (Instant::now() - before_thread).as_millis();
        assert!(
            elapsed_millis >= 200,
            "mutex locked before released by thread"
        )
    }

    #[test]
    fn poisoning_works_with_normal_lock() {
        let guarded_string = Arc::new(Mutex::new("before".to_owned()));

        std::thread::spawn({
            let guarded_string = guarded_string.clone();
            #[allow(unreachable_code, unused_variables)]
            move || {
                let lock_result = guarded_string.lock();
                panic!("Simulated thread panic");
                if let Ok(mut string) = lock_result {
                    *string = "modified".to_owned();
                }
            }
        });

        sleep(Duration::from_millis(100));
        // string should be modified by thread
        let lock_result = guarded_string.lock();
        assert!(lock_result.is_err(), "mutex wasn't poisoned");
    }

    #[test]
    #[should_panic]
    fn panics_on_poisoning_with_poisonless() {
        let guarded_string = Arc::new(Mutex::new("before".to_owned()));

        std::thread::spawn({
            let guarded_string = guarded_string.clone();
            #[allow(unreachable_code, unused_variables)]
            move || {
                let lock_result = guarded_string.lock();
                panic!("Simulated thread panic");
                if let Ok(mut string) = lock_result {
                    *string = "modified".to_owned();
                }
            }
        });

        sleep(Duration::from_millis(100));
        // this should panic because the mutex was poisoned
        let _string = guarded_string.poisonless_lock();
    }
}
