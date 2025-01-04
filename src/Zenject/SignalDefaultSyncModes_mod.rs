#[cfg(feature = "Zenject+SignalDefaultSyncModes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SignalDefaultSyncModes {
    #[default]
    Asynchronous = 1i32,
    Synchronous = 0i32,
}
#[cfg(feature = "Zenject+SignalDefaultSyncModes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalDefaultSyncModes => "Zenject"
    ."SignalDefaultSyncModes"
);
