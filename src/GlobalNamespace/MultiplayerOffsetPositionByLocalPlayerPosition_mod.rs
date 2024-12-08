#[cfg(feature = "MultiplayerOffsetPositionByLocalPlayerPosition")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerOffsetPositionByLocalPlayerPosition {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _multiplayerPlayersManager: *mut MultiplayerPlayersManager,
    pub _positionOffset: crate::UnityEngine::Vector3,
    pub _rotationOffset: crate::UnityEngine::Quaternion,
    pub _lastParentPosition: crate::UnityEngine::Vector3,
    pub _lastParentRotation: crate::UnityEngine::Quaternion,
}
#[cfg(feature = "MultiplayerOffsetPositionByLocalPlayerPosition")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerOffsetPositionByLocalPlayerPosition => ""
    ."MultiplayerOffsetPositionByLocalPlayerPosition"
);
#[cfg(feature = "MultiplayerOffsetPositionByLocalPlayerPosition")]
impl std::ops::Deref for MultiplayerOffsetPositionByLocalPlayerPosition {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerOffsetPositionByLocalPlayerPosition")]
impl std::ops::DerefMut for MultiplayerOffsetPositionByLocalPlayerPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerOffsetPositionByLocalPlayerPosition")]
impl MultiplayerOffsetPositionByLocalPlayerPosition {
    pub fn UpdatePositionAndRotationIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePositionAndRotationIfNeeded", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetEnabled(
        &mut self,
        isEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEnabled", (isEnabled))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerOffsetPositionByLocalPlayerPosition")]
impl quest_hook::libil2cpp::ObjectType
for MultiplayerOffsetPositionByLocalPlayerPosition {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
