#[cfg(feature = "UnityEngine+ProBuilder+MeshHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshHandle {
    __cordl_parent: crate::System::Object,
    pub m_Transform: *mut crate::UnityEngine::Transform,
    pub m_Mesh: *mut crate::UnityEngine::Mesh,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::MeshHandle =>
    "UnityEngine.ProBuilder"."MeshHandle"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandle")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshHandle {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandle")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandle")]
impl crate::UnityEngine::ProBuilder::MeshHandle {
    pub fn DrawMeshNow(
        &mut self,
        submeshIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawMeshNow", (submeshIndex))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        transform: *mut crate::UnityEngine::Transform,
        mesh: *mut crate::UnityEngine::Mesh,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (transform, mesh))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        transform: *mut crate::UnityEngine::Transform,
        mesh: *mut crate::UnityEngine::Mesh,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (transform, mesh))?;
        Ok(__cordl_ret)
    }
    pub fn get_mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Mesh> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Mesh = __cordl_object
            .invoke("get_mesh", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandle")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::MeshHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
