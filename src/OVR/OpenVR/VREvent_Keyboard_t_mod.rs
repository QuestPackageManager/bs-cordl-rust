#[cfg(feature = "OVR+OpenVR+VREvent_Keyboard_t")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VREvent_Keyboard_t {
    pub cNewInput0: u8,
    pub cNewInput1: u8,
    pub cNewInput2: u8,
    pub cNewInput3: u8,
    pub cNewInput4: u8,
    pub cNewInput5: u8,
    pub cNewInput6: u8,
    pub cNewInput7: u8,
    pub uUserValue: u64,
}
#[cfg(feature = "OVR+OpenVR+VREvent_Keyboard_t")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::VREvent_Keyboard_t => "OVR.OpenVR"
    ."VREvent_Keyboard_t"
);
#[cfg(feature = "OVR+OpenVR+VREvent_Keyboard_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::VREvent_Keyboard_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VREvent_Keyboard_t")]
impl crate::OVR::OpenVR::VREvent_Keyboard_t {
    pub fn get_cNewInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_cNewInput",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
