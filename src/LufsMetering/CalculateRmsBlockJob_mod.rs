#[cfg(feature = "LufsMetering+CalculateRmsBlockJob")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CalculateRmsBlockJob {
    pub inputData: crate::Unity::Collections::NativeArray_1<f32>,
    pub outputData: f32,
}
#[cfg(feature = "LufsMetering+CalculateRmsBlockJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LufsMetering::CalculateRmsBlockJob =>
    "LufsMetering"."CalculateRmsBlockJob"
);
#[cfg(feature = "LufsMetering+CalculateRmsBlockJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LufsMetering::CalculateRmsBlockJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LufsMetering+CalculateRmsBlockJob")]
impl crate::LufsMetering::CalculateRmsBlockJob {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LufsMetering+CalculateRmsBlockJob")]
impl AsRef<crate::Unity::Jobs::IJob> for crate::LufsMetering::CalculateRmsBlockJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "LufsMetering+CalculateRmsBlockJob")]
impl AsMut<crate::Unity::Jobs::IJob> for crate::LufsMetering::CalculateRmsBlockJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
