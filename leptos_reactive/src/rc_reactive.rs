use std::{ops::Deref, rc::Rc};

use crate::{
    create_signal, runtime::with_root_owner, Memo, ReadSignal, RwSignal,
    Signal, SignalDispose, StoredValue, WriteSignal,
};

/// 生成 Rc 包装的信号类型
macro_rules! rc_wrapper {
    ($name:ident, $inner_type:ident) => {
        /// A `$inner_type` that is owned by an `Rc`.
        #[derive(Debug, Clone)]
        pub struct $name<T: 'static> {
            inner: $inner_type<T>,
            #[allow(unused)]
            rc: Rc<AutoDispose>,
        }

        impl<T> $name<T> {
            /// Get the number of strong references.
            pub fn strong_count(&self) -> usize {
                Rc::strong_count(&self.rc)
            }
        }

        impl<T> Deref for $name<T> {
            type Target = $inner_type<T>;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
    };
}

// 使用宏生成各种 Rc 包装类型
rc_wrapper!(RcRwSignal, RwSignal);
impl<T: 'static> RcRwSignal<T> {
    /// 创建一个 Rc 包装的 RwSignal
    pub fn new(value: T) -> Self {
        let val = with_root_owner(|| RwSignal::new(value));
        Self {
            inner: val,
            rc: Rc::new(AutoDispose::new(val)),
        }
    }
}

rc_wrapper!(RcStoredValue, StoredValue);
impl<T: 'static> RcStoredValue<T> {
    /// 创建一个 Rc 包装的 StoredValue
    pub fn new(value: T) -> Self {
        let val = with_root_owner(|| StoredValue::new(value));
        Self {
            inner: val,
            rc: Rc::new(AutoDispose::new(val)),
        }
    }
}

rc_wrapper!(RcMemo, Memo);
impl<T: PartialEq + 'static> RcMemo<T> {
    /// 创建一个 Rc 包装的 Memo
    pub fn new(value: impl Fn(Option<&T>) -> T + 'static) -> Self {
        let val = with_root_owner(|| Memo::new(value));
        Self {
            inner: val,
            rc: Rc::new(AutoDispose::new(val)),
        }
    }
}

rc_wrapper!(RcSignal, Signal);
impl<T: PartialEq + 'static> RcSignal<T> {
    /// 创建一个 Rc 包装的 Signal
    pub fn new(value: impl Fn() -> T + 'static) -> Self {
        let val = with_root_owner(|| Signal::derive(value));
        Self {
            inner: val,
            rc: Rc::new(AutoDispose::new(val)),
        }
    }
}

rc_wrapper!(RcReadSignal, ReadSignal);
rc_wrapper!(RcWriteSignal, WriteSignal);

/// 创建一个 Rc 包装的 ReadSignal 和 WriteSignal
pub fn create_rc_signal<T: 'static>(
    value: T,
) -> (RcReadSignal<T>, RcWriteSignal<T>) {
    let (read, write) = with_root_owner(|| create_signal(value));
    let dispose = Rc::new(AutoDispose::new(read));
    (
        RcReadSignal {
            inner: read,
            rc: Rc::clone(&dispose),
        },
        RcWriteSignal {
            inner: write,
            rc: Rc::clone(&dispose),
        },
    )
}

struct AutoDispose(Box<dyn Fn()>);

impl std::fmt::Debug for AutoDispose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutoDispose").finish()
    }
}

impl AutoDispose {
    pub fn new<T: SignalDispose + Clone + 'static>(value: T) -> Self {
        Self(Box::new(move || value.clone().dispose()))
    }
}

impl Drop for AutoDispose {
    fn drop(&mut self) {
        self.0()
    }
}
