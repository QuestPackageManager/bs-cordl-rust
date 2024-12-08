#[cfg(feature = "UnityEngine+GraphicsBufferHandle")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GraphicsBufferHandle {
    pub value: u32,
}
#[cfg(feature = "UnityEngine+GraphicsBufferHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GraphicsBufferHandle =>
    "UnityEngine"."GraphicsBufferHandle"
);
#[cfg(feature = "UnityEngine+GraphicsBufferHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::GraphicsBufferHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+GraphicsBufferHandle")]
impl crate::UnityEngine::GraphicsBufferHandle {
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_GraphicsBufferHandle1(
        &mut self,
        other: crate::UnityEngine::GraphicsBufferHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
}
