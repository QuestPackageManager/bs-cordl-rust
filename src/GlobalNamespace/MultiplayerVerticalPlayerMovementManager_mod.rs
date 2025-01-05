#[cfg(feature = "MultiplayerVerticalPlayerMovementManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerVerticalPlayerMovementManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _movementRange: f32,
    pub _duelMovementRange: f32,
    pub _maxMoveSpeedMetersPerSecond: f32,
    pub _accelerationMetersPerSecondSquared: f32,
    pub _decelerationMetersPerSecondSquared: f32,
    pub _minScoreDifference: f32,
    pub _multiplayerPlayersManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerPlayersManager,
    >,
    pub _scoreProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerScoreProvider,
    >,
    pub _layoutProvider: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerLayoutProvider,
    >,
    pub _multiplayerController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerController,
    >,
    pub _reusablePlayersList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
            >,
        >,
    >,
    pub _currentSpeedsDictionary: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerConnectedPlayerFacade,
            >,
            f32,
        >,
    >,
    pub _lastFrameBaseScore: f32,
}
#[cfg(feature = "MultiplayerVerticalPlayerMovementManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerVerticalPlayerMovementManager => ""
    ."MultiplayerVerticalPlayerMovementManager"
);
#[cfg(feature = "MultiplayerVerticalPlayerMovementManager")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerVerticalPlayerMovementManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerVerticalPlayerMovementManager")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerVerticalPlayerMovementManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerVerticalPlayerMovementManager")]
impl crate::GlobalNamespace::MultiplayerVerticalPlayerMovementManager {
    pub fn HandleStateChanged(
        &mut self,
        state: crate::GlobalNamespace::MultiplayerController_State,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleStateChanged", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
#[cfg(feature = "MultiplayerVerticalPlayerMovementManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerVerticalPlayerMovementManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
