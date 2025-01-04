#[cfg(feature = "System+Net+WebExceptionInternalStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WebExceptionInternalStatus {
    #[default]
    Isolated = 3i32,
    Recoverable = 2i32,
    RequestFatal = 0i32,
    ServicePointFatal = 1i32,
}
#[cfg(feature = "System+Net+WebExceptionInternalStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebExceptionInternalStatus =>
    "System.Net"."WebExceptionInternalStatus"
);
