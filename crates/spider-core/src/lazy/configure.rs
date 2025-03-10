use static_assertions::assert_impl_all;

assert_impl_all!(ConfigureOnCreate<String>: Send, Sync);


/// A lazy type that gets configured on creation
pub struct ConfigureOnCreate<T> {
    state: State<T>,
}

impl<T> ConfigureOnCreate<T> {
    /// Create a new `ConfigureOnCreate` object
    pub fn new<F>(f: F) -> Self
        where F: FnOnce() -> T + Send + Sync + 'static
    {
        Self {
            state: State::Init {
                lazy: Box::new(f),
                configurations: vec![],
            }
        }
    }

    /// Initializes and gets the inner object, running all configurations of necessary
    pub fn init_and_get(&mut self) -> &mut T {
        self.state.get_or_init()
    }

    /// Gets the inner data if possible
    pub fn get(&self) -> Option<&T> {
        self.state.get()
    }

    /// Gets the inner data as a mutable reference if possible
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.state.get_mut()
    }
    /// Configures this inner object, if possible
    pub fn configure<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T) + Send + Sync + 'static,
    {
        match &mut self.state {
            State::Poisoned => {
                panic!("can not configured poisoned ConfigureOnCreate")
            }
            State::Init { configurations, .. } => {
                configurations.push(Box::new(f));
            }
            State::Available(a) => f(a),
        }
    }

    /// Checks if the data in this is initialized
    pub fn is_init(&self) -> bool {
        matches!(self.state, State::Available(_))
    }
}

enum State<T> {
    Poisoned,
    Init {
        lazy: Box<dyn FnOnce() -> T + Send + Sync>,
        configurations: Vec<Box<dyn FnOnce(&mut T) + Send + Sync>>,
    },
    Available(T),
}

impl<T> State<T> {
    fn get_or_init(&mut self) -> &mut T {
        self.init();
        match self {
            State::Available(v) => v,
            _ => panic!("Failed to initialize value"),
        }
    }
    fn init(&mut self) {
        match self {
            State::Init { .. } => {
                let State::Init {
                    lazy,
                    configurations,
                } = std::mem::replace(self, State::Poisoned)
                else {
                    unreachable!()
                };

                let mut t = lazy();
                for configuration in configurations {
                    configuration(&mut t);
                }
                *self = State::Available(t);
            }
            State::Available(_) => {}
            State::Poisoned => {
                panic!("trying to configure panicked ConfigurationOnCreate")
            }
        }
    }

    fn get(&self) -> Option<&T> {
        match self {
            State::Available(v) => Some(v),
            _ => None,
        }
    }

    fn get_mut(&mut self) -> Option<&mut T> {
        match self {
            State::Available(v) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lazy::configure::ConfigureOnCreate;

    #[test]
    fn test_configure_on_create() {
        let mut c = ConfigureOnCreate::new(String::default);
        assert_eq!(c.get(), None);
        c.configure(|s| {
            s.push_str("Hello, world!");
        });
        assert_eq!(c.get(), None);

        let s = c.init_and_get().clone();
        assert_eq!(s, "Hello, world!");
    }
}