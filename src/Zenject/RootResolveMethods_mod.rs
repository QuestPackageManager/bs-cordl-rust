#[cfg(feature = "Zenject+RootResolveMethods")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RootResolveMethods {
    #[default]
    All = 1i32,
    NonLazyOnly = 0i32,
}
#[cfg(feature = "Zenject+RootResolveMethods")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::RootResolveMethods => "Zenject"
    ."RootResolveMethods"
);
