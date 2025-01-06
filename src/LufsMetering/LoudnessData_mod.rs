#[cfg(feature = "LufsMetering+LoudnessData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoudnessData {
    pub lufs: f32,
    pub blockDuration: f32,
    pub momentaryValues: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
}
#[cfg(feature = "LufsMetering+LoudnessData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LufsMetering::LoudnessData => "LufsMetering"
    ."LoudnessData"
);
#[cfg(feature = "LufsMetering+LoudnessData")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::LufsMetering::LoudnessData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LufsMetering+LoudnessData")]
impl crate::LufsMetering::LoudnessData {
    pub fn _ctor(
        &mut self,
        lufs: f32,
        momentaryValues: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<f32>,
        >,
        blockDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (lufs, momentaryValues, blockDuration),
        )?;
        Ok(__cordl_ret.into())
    }
}
