#[cfg(feature = "System+Threading+Tasks+VoidTaskResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VoidTaskResult {}
#[cfg(feature = "System+Threading+Tasks+VoidTaskResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::VoidTaskResult =>
    "System.Threading.Tasks"."VoidTaskResult"
);
#[cfg(feature = "System+Threading+Tasks+VoidTaskResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::Tasks::VoidTaskResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+Tasks+VoidTaskResult")]
impl crate::System::Threading::Tasks::VoidTaskResult {}
