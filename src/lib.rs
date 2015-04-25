//   Copyright 2015 Colin Sherratt
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

extern crate pulse;

use std::thread;
use pulse::{Signal, TimeoutError};

/// Run f for at most max_ms, this function will panic if
/// f is still running.
pub fn timeout<F>(f: F, max_ms: u32) where F: FnOnce() + Send + 'static {
    let (mut signal, pulse) = Signal::new();

    let guard = thread::spawn(|| {
        f();
        pulse.pulse();
    });

    match signal.wait_timeout_ms(max_ms) {
        Err(TimeoutError::Timeout) => {
            panic!("Timed out");
        }
        _ => ()
    }

    guard.join().unwrap();
}

#[test]
fn timeout_no_timeout() {
    timeout(|| {}, 100);
}

#[test]
#[should_panic]
fn timeout_spin() {
    timeout(|| loop {}, 100);
}

#[test]
#[should_panic]
fn child_panics() {
    timeout(|| {
        panic!("oh no!")
    }, 100);
}