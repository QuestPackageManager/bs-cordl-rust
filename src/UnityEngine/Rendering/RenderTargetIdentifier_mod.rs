#[cfg(feature = "UnityEngine+Rendering+RenderTargetIdentifier")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Texture2(
        &mut self,
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (tex),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_BuiltinRenderTextureType0(
        _cordl_type: crate::UnityEngine::Rendering::BuiltinRenderTextureType,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderTargetIdentifier,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::RenderTargetIdentifier = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Texture2(
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderTargetIdentifier,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::RenderTargetIdentifier = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (tex))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderTargetIdentifier,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::RenderTargetIdentifier = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (nameID))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderTargetIdentifier")]
impl AsRef<
    crate::System::IEquatable_1<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
> for crate::UnityEngine::Rendering::RenderTargetIdentifier {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::RenderTargetIdentifier,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderTargetIdentifier")]
impl AsMut<
    crate::System::IEquatable_1<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
> for crate::UnityEngine::Rendering::RenderTargetIdentifier {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::RenderTargetIdentifier,
    > {
        todo!()
    }
}
