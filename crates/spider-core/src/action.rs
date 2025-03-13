//! Generic trait [`Action`] as a way of performing some action against some object

use static_assertions::assert_obj_safe;

/// An action against some type
pub trait Action<T, R=()> {
    /// Executes this action against object T
    fn execute(&self, t: T) -> R;
}
assert_obj_safe!(Action<i32>);

pub type BoxAction<T, R=()> = Box<dyn Action<T, R>>;

impl<T, R, F> Action<T, R> for F
where F: Fn(T) -> R {
    fn execute(&self, t: T) -> R {
        self(t)
    }
}

/// Runs an `action` against `t`.
pub fn execute<T, R>(t: T, action: impl Action<T, R>) -> R {
    action.execute(t)
}

#[cfg(test)]
mod tests {
    use crate::action::execute;

    #[test]
    fn test_execute() {
        let pow = execute(2, |i| { i * i });
        assert_eq!(pow, 4);
    }
}