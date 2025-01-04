#[cfg(feature = "Internal+Runtime+Augments+AsyncStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AsyncStatus {
    #[default]
    Canceled = 2i32,
    Completed = 1i32,
    Error = 3i32,
    Started = 0i32,
}
#[cfg(feature = "Internal+Runtime+Augments+AsyncStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Internal::Runtime::Augments::AsyncStatus =>
    "Internal.Runtime.Augments"."AsyncStatus"
);
