//! The private observer cannot outlive an unwind or leak into parallel tests.

use super::*;
use std::sync::{Arc, Barrier};

#[test]
fn checkout_observer_panics_releases_scope_and_guards_before_explicit_retry() {
    let dir = fixture();
    let repo = ProjectRepo::open(dir.path()).unwrap();
    let before = snapshot(dir.path());

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _scope = observer::install(|phase| {
            assert_ne!(phase, Phase::Guarded, "owned test-only observer panic");
        });
        repo.checkout(REFERENCE, actor())
    }));

    assert!(result.is_err(), "observer must actually unwind inside checkout");
    assert_eq!(snapshot(dir.path()), before);
    assert_unlocked(dir.path());
    // A surviving callback would panic again; an uncleared slot would reject
    // the new installation. Exercise both installation and the production path.
    let seen = Rc::new(RefCell::new(Vec::new()));
    let recorded = seen.clone();
    let scope = observer::install(move |phase| recorded.borrow_mut().push(phase));
    repo.checkout(REFERENCE, actor()).unwrap();
    drop(scope);
    assert_eq!(seen.borrow().iter().filter(|phase| **phase == Phase::HeadPublished).count(), 1);
    assert_target(dir.path());
}

#[test]
fn checkout_parallel_observers_record_only_their_own_thread() {
    let barrier = Arc::new(Barrier::new(2));
    let fixtures = [fixture(), fixture()];
    let threads: Vec<_> = fixtures
        .into_iter()
        .map(|dir| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let repo = ProjectRepo::open(dir.path());
                let seen = Rc::new(RefCell::new(Vec::new()));
                let recorded = seen.clone();
                let scope = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    observer::install(move |phase| recorded.borrow_mut().push(phase))
                }));
                barrier.wait();
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let _scope = scope.unwrap();
                    repo.unwrap().checkout(REFERENCE, actor())
                }));
                // Even setup/checkout panics release the peer before asserting.
                barrier.wait();
                result.unwrap().unwrap();
                for phase in [Phase::Observed, Phase::Guarded, Phase::WorktreeUpdated, Phase::HeadPublished] {
                    assert_eq!(seen.borrow().iter().filter(|observed| **observed == phase).count(), 1);
                }
                assert_target(dir.path());
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}
