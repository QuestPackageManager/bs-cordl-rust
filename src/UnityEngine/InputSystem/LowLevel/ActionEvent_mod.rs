#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+ActionEvent+_m_ValueData_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ActionEvent__m_ValueData_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+ActionEvent+_m_ValueData_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::ActionEvent__m_ValueData_e__FixedBuffer =>
    "UnityEngine.InputSystem.LowLevel"."ActionEvent/<m_ValueData>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+ActionEvent+_m_ValueData_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::ActionEvent__m_ValueData_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+ActionEvent+_m_ValueData_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::LowLevel::ActionEvent__m_ValueData_e__FixedBuffer {}
