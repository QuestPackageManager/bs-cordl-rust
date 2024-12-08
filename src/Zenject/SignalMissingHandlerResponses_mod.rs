#[cfg(feature = "Zenject+SignalMissingHandlerResponses")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalMissingHandlerResponses {
    Ignore = 0i32,
    Throw = 1i32,
    Warn = 2i32,
}
#[cfg(feature = "Zenject+SignalMissingHandlerResponses")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalMissingHandlerResponses =>
    "Zenject"."SignalMissingHandlerResponses"
);
