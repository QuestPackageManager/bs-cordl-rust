#[cfg(feature = "Zenject+ScopeTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeTypes {
    Singleton = 2i32,
    Transient = 1i32,
    Unset = 0i32,
}
#[cfg(feature = "Zenject+ScopeTypes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ScopeTypes => "Zenject"."ScopeTypes"
);
