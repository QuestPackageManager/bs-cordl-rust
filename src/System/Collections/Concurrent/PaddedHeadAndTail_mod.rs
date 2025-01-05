#[cfg(feature = "System+Collections+Concurrent+PaddedHeadAndTail")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PaddedHeadAndTail {
    padding: quest_hook::libil2cpp::ValueTypePadding<260usize>,
}
#[cfg(feature = "System+Collections+Concurrent+PaddedHeadAndTail")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Concurrent::PaddedHeadAndTail =>
    "System.Collections.Concurrent"."PaddedHeadAndTail"
);
#[cfg(feature = "System+Collections+Concurrent+PaddedHeadAndTail")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Collections::Concurrent::PaddedHeadAndTail {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Collections+Concurrent+PaddedHeadAndTail")]
impl crate::System::Collections::Concurrent::PaddedHeadAndTail {}
