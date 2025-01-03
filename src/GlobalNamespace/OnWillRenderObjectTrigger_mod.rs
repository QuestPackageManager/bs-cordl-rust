#[cfg(feature = "OnWillRenderObjectTrigger")]
#[repr(C)]
#[derive(Debug)]
pub struct OnWillRenderObjectTrigger {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _overrideShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub _renderQueue: i32,
    pub _material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub _meshFilter: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshFilter>,
    pub _meshRenderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::MeshRenderer>,
}
#[cfg(feature = "OnWillRenderObjectTrigger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OnWillRenderObjectTrigger => ""
    ."OnWillRenderObjectTrigger"
);
#[cfg(feature = "OnWillRenderObjectTrigger")]
impl std::ops::Deref for crate::GlobalNamespace::OnWillRenderObjectTrigger {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OnWillRenderObjectTrigger")]
impl std::ops::DerefMut for crate::GlobalNamespace::OnWillRenderObjectTrigger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OnWillRenderObjectTrigger")]
impl crate::GlobalNamespace::OnWillRenderObjectTrigger {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "OnWillRenderObjectTrigger")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OnWillRenderObjectTrigger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
