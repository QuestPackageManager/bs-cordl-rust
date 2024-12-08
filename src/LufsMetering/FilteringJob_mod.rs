#[cfg(feature = "LufsMetering+FilteringJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FilteringJob {
    pub inputData: crate::Unity::Collections::NativeArray_1<f32>,
    pub coefficients: crate::LufsMetering::FilterCoefficients,
    pub outputData: crate::Unity::Collections::NativeArray_1<f32>,
}
#[cfg(feature = "LufsMetering+FilteringJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LufsMetering::FilteringJob => "LufsMetering"
    ."FilteringJob"
);
#[cfg(feature = "LufsMetering+FilteringJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::LufsMetering::FilteringJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LufsMetering+FilteringJob")]
impl crate::LufsMetering::FilteringJob {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        inputData: crate::Unity::Collections::NativeArray_1<f32>,
        outputData: crate::Unity::Collections::NativeArray_1<f32>,
        coefficients: crate::LufsMetering::FilterCoefficients,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (inputData, outputData, coefficients),
        )?;
        Ok(__cordl_ret)
    }
}
