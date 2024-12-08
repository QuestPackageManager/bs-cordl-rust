#[cfg(feature = "MultiplayerConnectedPlayerSpectatingSpot")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerSpectatingSpot {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _connectedPlayer: *mut IConnectedPlayer,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub _spectatingSpotManager: *mut MultiplayerSpectatingSpotManager,
    pub hasBeenRemovedEvent: *mut crate::System::Action_1<
        *mut IMultiplayerSpectatingSpot,
    >,
    pub isObservedChangedEvent: *mut crate::System::Action_1<bool>,
    pub _observable_k__BackingField: *mut IMultiplayerObservable,
    pub _playerFailed: bool,
}
#[cfg(feature = "MultiplayerConnectedPlayerSpectatingSpot")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerConnectedPlayerSpectatingSpot => ""
    ."MultiplayerConnectedPlayerSpectatingSpot"
);
#[cfg(feature = "MultiplayerConnectedPlayerSpectatingSpot")]
impl std::ops::Deref for MultiplayerConnectedPlayerSpectatingSpot {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerSpectatingSpot")]
impl std::ops::DerefMut for MultiplayerConnectedPlayerSpectatingSpot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerSpectatingSpot")]
impl MultiplayerConnectedPlayerSpectatingSpot {
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
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetIsObserved(
        &mut self,
        isObserved: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsObserved", (isObserved))?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_hasBeenRemovedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IMultiplayerSpectatingSpot>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_hasBeenRemovedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn IMultiplayerSpectatingSpot_get_transform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("IMultiplayerSpectatingSpot.get_transform", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_isObservedChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_isObservedChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ReloadBasedOnPlayerCurrentState(
        &mut self,
        connectedPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadBasedOnPlayerCurrentState", (connectedPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerStateChanged(
        &mut self,
        connectedPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerStateChanged", (connectedPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerDisconnected(
        &mut self,
        connectedPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerDisconnected", (connectedPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn get_spotName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_spotName", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_isObservedChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_isObservedChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_observable(
        &mut self,
        value: *mut IMultiplayerObservable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_observable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_isMain(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isMain", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_hasBeenRemovedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut IMultiplayerSpectatingSpot>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_hasBeenRemovedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_observable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IMultiplayerObservable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IMultiplayerObservable = __cordl_object
            .invoke("get_observable", ())?;
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
#[cfg(feature = "MultiplayerConnectedPlayerSpectatingSpot")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerConnectedPlayerSpectatingSpot {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
