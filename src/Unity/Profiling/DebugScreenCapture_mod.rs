#[cfg(feature = "Unity+Profiling+DebugScreenCapture")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DebugScreenCapture {
    pub _RawImageDataReference_k__BackingField: crate::Unity::Collections::NativeArray_1<
        u8,
    >,
    pub _ImageFormat_k__BackingField: crate::UnityEngine::TextureFormat,
    pub _Width_k__BackingField: i32,
    pub _Height_k__BackingField: i32,
}
#[cfg(feature = "Unity+Profiling+DebugScreenCapture")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::DebugScreenCapture =>
    "Unity.Profiling"."DebugScreenCapture"
);
#[cfg(feature = "Unity+Profiling+DebugScreenCapture")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::DebugScreenCapture {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+DebugScreenCapture")]
impl crate::Unity::Profiling::DebugScreenCapture {
    pub fn set_Height(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Height",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_ImageFormat(
        &mut self,
        value: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ImageFormat",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_RawImageDataReference(
        &mut self,
        value: crate::Unity::Collections::NativeArray_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_RawImageDataReference",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_Width(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Width",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
