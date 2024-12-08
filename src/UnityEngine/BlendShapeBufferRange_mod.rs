#[cfg(feature = "UnityEngine+BlendShapeBufferRange")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BlendShapeBufferRange {
    pub m_StartIndex: u32,
    pub m_EndIndex: u32,
}
#[cfg(feature = "UnityEngine+BlendShapeBufferRange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::BlendShapeBufferRange =>
    "UnityEngine"."BlendShapeBufferRange"
);
#[cfg(feature = "UnityEngine+BlendShapeBufferRange")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::BlendShapeBufferRange {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+BlendShapeBufferRange")]
impl crate::UnityEngine::BlendShapeBufferRange {
    pub fn set_endIndex(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_endIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_startIndex(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_startIndex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
