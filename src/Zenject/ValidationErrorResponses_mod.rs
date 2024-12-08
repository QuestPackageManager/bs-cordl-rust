#[cfg(feature = "Zenject+ValidationErrorResponses")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationErrorResponses {
    Log = 0i32,
    Throw = 1i32,
}
#[cfg(feature = "Zenject+ValidationErrorResponses")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ValidationErrorResponses => "Zenject"
    ."ValidationErrorResponses"
);
