#[cfg(feature = "MultiplayerLobbyCenterStageManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLobbyCenterStageManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _centerObjectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _centerStageScreenController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::CenterStageScreenController,
    >,
    pub _lobbyStateDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ILobbyStateDataModel,
    >,
    pub _innerCircleRadius: f32,
    pub _minOuterCircleRadius: f32,
}
#[cfg(feature = "MultiplayerLobbyCenterStageManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLobbyCenterStageManager => ""
    ."MultiplayerLobbyCenterStageManager"
);
#[cfg(feature = "MultiplayerLobbyCenterStageManager")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLobbyCenterStageManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyCenterStageManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerLobbyCenterStageManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLobbyCenterStageManager")]
impl crate::GlobalNamespace::MultiplayerLobbyCenterStageManager {
    pub fn ActivateCenterStageManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateCenterStageManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DeactivateCenterStageManager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateCenterStageManager", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        innerCircleRadius: f32,
        minOuterCircleRadius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (innerCircleRadius, minOuterCircleRadius))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RecalculateCenterPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateCenterPosition", ())?;
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
#[cfg(feature = "MultiplayerLobbyCenterStageManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLobbyCenterStageManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
