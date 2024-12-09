#[cfg(feature = "LIV+SDK+Unity+SDKShaders")]
#[repr(C)]
#[derive(Debug)]
pub struct SDKShaders {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LIV+SDK+Unity+SDKShaders")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKShaders => "LIV.SDK.Unity"
    ."SDKShaders"
);
#[cfg(feature = "LIV+SDK+Unity+SDKShaders")]
impl std::ops::Deref for crate::LIV::SDK::Unity::SDKShaders {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKShaders")]
impl std::ops::DerefMut for crate::LIV::SDK::Unity::SDKShaders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKShaders")]
impl crate::LIV::SDK::Unity::SDKShaders {
    pub const LIV_CLIP_PLANE_COMPLEX_DEBUG_SHADER: &'static str = "Hidden/LIV_ClipPlaneComplexDebug";
    pub const LIV_CLIP_PLANE_COMPLEX_SHADER: &'static str = "Hidden/LIV_ClipPlaneComplex";
    pub const LIV_CLIP_PLANE_SIMPLE_DEBUG_SHADER: &'static str = "Hidden/LIV_ClipPlaneSimpleDebug";
    pub const LIV_CLIP_PLANE_SIMPLE_SHADER: &'static str = "Hidden/LIV_ClipPlaneSimple";
    pub const LIV_COMBINE_ALPHA_SHADER: &'static str = "Hidden/LIV_CombineAlpha";
    pub const LIV_FORCE_FORWARD_RENDERING_SHADER: &'static str = "Hidden/LIV_ForceForwardRendering";
    pub const LIV_MR_BACKGROUND_KEYWORD: &'static str = "LIV_MR_BACKGROUND";
    pub const LIV_MR_FOREGROUND_KEYWORD: &'static str = "LIV_MR_FOREGROUND";
    pub const LIV_MR_KEYWORD: &'static str = "LIV_MR";
    pub const LIV_WRITE_OPAQUE_TO_ALPHA_SHADER: &'static str = "Hidden/LIV_WriteOpaqueToAlpha";
    pub const LIV_WRITE_SHADER: &'static str = "Hidden/LIV_Write";
}
#[cfg(feature = "LIV+SDK+Unity+SDKShaders")]
impl quest_hook::libil2cpp::ObjectType for crate::LIV::SDK::Unity::SDKShaders {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
