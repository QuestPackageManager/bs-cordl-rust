#[cfg(feature = "MoveAndRotateWithMainCamera")]
#[repr(C)]
#[derive(Debug)]
pub struct MoveAndRotateWithMainCamera {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _mainCamera: *mut crate::GlobalNamespace::MainCamera,
    pub _rotationOffset: crate::UnityEngine::Quaternion,
    pub _positionOffset: crate::UnityEngine::Vector3,
    pub _transform: *mut crate::UnityEngine::Transform,
}
#[cfg(feature = "MoveAndRotateWithMainCamera")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MoveAndRotateWithMainCamera =>
    ""."MoveAndRotateWithMainCamera"
);
#[cfg(feature = "MoveAndRotateWithMainCamera")]
impl std::ops::Deref for crate::GlobalNamespace::MoveAndRotateWithMainCamera {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MoveAndRotateWithMainCamera")]
impl std::ops::DerefMut for crate::GlobalNamespace::MoveAndRotateWithMainCamera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MoveAndRotateWithMainCamera")]
impl crate::GlobalNamespace::MoveAndRotateWithMainCamera {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "MoveAndRotateWithMainCamera")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MoveAndRotateWithMainCamera {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
