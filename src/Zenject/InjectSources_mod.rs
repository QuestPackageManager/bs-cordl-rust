#[cfg(feature = "Zenject+InjectSources")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InjectSources {
    #[default]
    Any = 0i32,
    AnyParent = 3i32,
    Local = 1i32,
    Parent = 2i32,
}
#[cfg(feature = "Zenject+InjectSources")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InjectSources => "Zenject"
    ."InjectSources"
);
