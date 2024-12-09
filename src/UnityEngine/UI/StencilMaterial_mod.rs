#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
#[repr(C)]
#[derive(Debug)]
pub struct StencilMaterial {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::StencilMaterial =>
    "UnityEngine.UI"."StencilMaterial"
);
#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
impl std::ops::Deref for crate::UnityEngine::UI::StencilMaterial {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
impl std::ops::DerefMut for crate::UnityEngine::UI::StencilMaterial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
impl crate::UnityEngine::UI::StencilMaterial {
    #[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
    pub type MatEntry = crate::UnityEngine::UI::StencilMaterial_MatEntry;
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::StencilMaterial {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct StencilMaterial_MatEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub baseMat: *mut crate::UnityEngine::Material,
    pub customMat: *mut crate::UnityEngine::Material,
    pub count: i32,
    pub stencilId: i32,
    pub operation: crate::UnityEngine::Rendering::StencilOp,
    pub compareFunction: crate::UnityEngine::Rendering::CompareFunction,
    pub readMask: i32,
    pub writeMask: i32,
    pub useAlphaClip: bool,
    pub colorMask: crate::UnityEngine::Rendering::ColorWriteMask,
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::StencilMaterial_MatEntry =>
    "UnityEngine.UI"."StencilMaterial/MatEntry"
);
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
impl std::ops::Deref for crate::UnityEngine::UI::StencilMaterial_MatEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
impl std::ops::DerefMut for crate::UnityEngine::UI::StencilMaterial_MatEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
impl crate::UnityEngine::UI::StencilMaterial_MatEntry {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::StencilMaterial_MatEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
