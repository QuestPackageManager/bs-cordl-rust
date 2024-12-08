#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Door")]
#[repr(C)]
#[derive(Debug)]
pub struct Door {
    __cordl_parent: crate::UnityEngine::ProBuilder::Shapes::Shape,
    pub m_DoorHeight: f32,
    pub m_LegWidth: f32,
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Door")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Shapes::Door =>
    "UnityEngine.ProBuilder.Shapes"."Door"
);
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Door")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Shapes::Door {
    type Target = crate::UnityEngine::ProBuilder::Shapes::Shape;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Door")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Shapes::Door {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Door")]
impl crate::UnityEngine::ProBuilder::Shapes::Door {
    pub fn CopyShape(
        &mut self,
        shape: *mut crate::UnityEngine::ProBuilder::Shapes::Shape,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyShape", (shape))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RebuildMesh(
        &mut self,
        mesh: *mut crate::UnityEngine::ProBuilder::ProBuilderMesh,
        _cordl_size: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("RebuildMesh", (mesh, _cordl_size, rotation))?;
        Ok(__cordl_ret)
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
#[cfg(feature = "UnityEngine+ProBuilder+Shapes+Door")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Shapes::Door {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
