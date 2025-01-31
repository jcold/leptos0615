use std::{ops::Deref, rc::Rc};

use crate::{runtime::with_root_owner, RwSignal, SignalDispose};

/// A `RwSignal` that is owned by an `Rc`.
#[derive(Debug, Clone)]
pub struct RcRwSignal<T: 'static> {
    inner: RwSignal<T>,
    #[allow(unused)]
    rc: Rc<AutoDispose>,
}

// impl<T: 'static> RcRwSignal<T> {
//     pub(crate) fn try_dispose(&self) -> bool {
//         if Rc::strong_count(&self.rc) == 1 {
//             self.inner.dispose();
//             true
//         } else {
//             false
//         }
//     }
// }

impl<T> RcRwSignal<T> {
    /// Create a new `RcRwSignal` that is owned by an `Rc`.
    pub fn new(value: T) -> Self {
        let val = with_root_owner(|| RwSignal::new(value));
        tracing::debug!("Creating RcRwSignal: {:?}", val);
        let auto_dispose = AutoDispose::new(Box::new(move || {
            tracing::debug!("Disposing of RcRwSignal: {:?}", val);
            val.dispose()
        }));
        Self {
            inner: val,
            rc: Rc::new(auto_dispose),
        }
    }

    /// Get the number of strong references to the `RcRwSignal`.
    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.rc)
    }
}

impl<T> Deref for RcRwSignal<T> {
    type Target = RwSignal<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

struct AutoDispose(Box<dyn Fn()>);

impl std::fmt::Debug for AutoDispose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutoDispose").finish()
    }
}

impl AutoDispose {
    pub fn new(handle: Box<dyn Fn()>) -> Self {
        Self(handle)
    }
}

impl Drop for AutoDispose {
    fn drop(&mut self) {
        self.0()
    }
}
