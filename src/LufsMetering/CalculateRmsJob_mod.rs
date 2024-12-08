#[cfg(feature = "LufsMetering+CalculateRmsJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CalculateRmsJob {
    pub inputData: crate::Unity::Collections::NativeArray_1<f32>,
    pub outputData: crate::Unity::Collections::NativeArray_1<f32>,
    pub step: f32,
    pub timeGate: f32,
    pub rate: i32,
}
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LufsMetering::CalculateRmsJob => "LufsMetering"
    ."CalculateRmsJob"
);
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LufsMetering::CalculateRmsJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LufsMetering+CalculateRmsJob")]
impl crate::LufsMetering::CalculateRmsJob {
    pub fn Execute(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (i),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        inputData: crate::Unity::Collections::NativeArray_1<f32>,
        outputData: crate::Unity::Collections::NativeArray_1<f32>,
        step: f32,
        timeGate: f32,
        rate: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (inputData, outputData, step, timeGate, rate),
        )?;
        Ok(__cordl_ret)
    }
}
