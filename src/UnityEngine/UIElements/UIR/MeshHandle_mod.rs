#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshHandle {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    >,
    pub allocVerts: crate::UnityEngine::UIElements::UIR::Alloc,
    pub allocIndices: crate::UnityEngine::UIElements::UIR::Alloc,
    pub triangleCount: u32,
    pub allocPage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::Page>,
    pub allocTime: u32,
    pub updateAllocID: u32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::MeshHandle =>
    "UnityEngine.UIElements.UIR"."MeshHandle"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::MeshHandle {
    type Target = quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::UIR::MeshHandle>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::MeshHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
impl crate::UnityEngine::UIElements::UIR::MeshHandle {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::MeshHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
