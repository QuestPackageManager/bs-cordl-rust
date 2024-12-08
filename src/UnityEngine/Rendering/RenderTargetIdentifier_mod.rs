#[cfg(feature = "UnityEngine+Rendering+RenderTargetIdentifier")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RenderTargetIdentifier {
    pub m_Type: crate::UnityEngine::Rendering::BuiltinRenderTextureType,
    pub m_NameID: i32,
    pub m_InstanceID: i32,
    pub m_BufferPointer: crate::System::IntPtr,
    pub m_MipLevel: i32,
    pub m_CubeFace: crate::UnityEngine::CubemapFace,
    pub m_DepthSlice: i32,
}
#[cfg(feature = "UnityEngine+Rendering+RenderTargetIdentifier")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::RenderTargetIdentifier
    => "UnityEngine.Rendering"."RenderTargetIdentifier"
);
#[cfg(feature = "UnityEngine+Rendering+RenderTargetIdentifier")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::RenderTargetIdentifier {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderTargetIdentifier")]
impl crate::UnityEngine::Rendering::RenderTargetIdentifier {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BuiltinRenderTextureType0(
        &mut self,
        _cordl_type: crate::UnityEngine::Rendering::BuiltinRenderTextureType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (nameID),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Texture2(
        &mut self,
        tex: *mut crate::UnityEngine::Texture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (tex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_RenderTargetIdentifier0(
        &mut self,
        rhs: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (rhs),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
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
}
