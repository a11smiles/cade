//! Private test-only observations; never compiled into a normal library build.

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Phase {
    Observed,
    Guarded,
    SdkProgress,
    SdkReturned { vetoed: bool, succeeded: bool },
    SdkFailed { code: git2::ErrorCode, class: git2::ErrorClass },
    WorktreeUpdated,
    HeadPublished,
}

struct Observer {
    callback: Box<dyn FnMut(Phase)>,
}

thread_local! {
    static OBSERVER: RefCell<Option<Observer>> = const { RefCell::new(None) };
}

// Rc's marker makes the guard !Send/!Sync: only its installing test thread can
// drop it. A panic in a callback unwinds its borrow before this guard is dropped.
pub(super) struct Scope {
    _thread: PhantomData<Rc<()>>,
}

pub(super) fn install(callback: impl FnMut(Phase) + 'static) -> Scope {
    OBSERVER.with(|slot| {
        let mut slot = slot.borrow_mut();
        assert!(slot.is_none(), "checkout observer scopes must not overlap on one thread");
        *slot = Some(Observer {
            callback: Box::new(callback),
        });
    });
    Scope { _thread: PhantomData }
}

pub(super) fn visit(phase: Phase) {
    OBSERVER.with(|slot| {
        if let Some(observer) = slot.borrow_mut().as_mut() {
            (observer.callback)(phase);
        }
    });
}

impl Drop for Scope {
    fn drop(&mut self) {
        let observer = OBSERVER.with(|slot| slot.borrow_mut().take());
        drop(observer);
    }
}
