#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalShaderPassNames")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct DecalShaderPassNames {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalShaderPassNames")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::DecalShaderPassNames
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DecalShaderPassNames";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalShaderPassNames")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::DecalShaderPassNames {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalShaderPassNames")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::DecalShaderPassNames {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalShaderPassNames")]
impl crate::UnityEngine::Rendering::Universal::DecalShaderPassNames {
    pub const DBufferMesh: &'static str = "DBufferMesh";
    pub const DBufferProjector: &'static str = "DBufferProjector";
    pub const DBufferProjectorVFX: &'static str = "DBufferProjectorVFX";
    pub const DecalGBufferMesh: &'static str = "DecalGBufferMesh";
    pub const DecalGBufferProjector: &'static str = "DecalGBufferProjector";
    pub const DecalGBufferProjectorVFX: &'static str = "DecalGBufferProjectorVFX";
    pub const DecalMeshForwardEmissive: &'static str = "DecalMeshForwardEmissive";
    pub const DecalPreview: &'static str = "DecalPreview";
    pub const DecalProjectorForwardEmissive: &'static str = "DecalProjectorForwardEmissive";
    pub const DecalProjectorForwardEmissiveVFX: &'static str = "DecalProjectorForwardEmissiveVFX";
    pub const DecalScreenSpaceMesh: &'static str = "DecalScreenSpaceMesh";
    pub const DecalScreenSpaceProjector: &'static str = "DecalScreenSpaceProjector";
    pub const DecalScreenSpaceProjectorVFX: &'static str = "DecalScreenSpaceProjectorVFX";
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalShaderPassNames")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::DecalShaderPassNames
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
