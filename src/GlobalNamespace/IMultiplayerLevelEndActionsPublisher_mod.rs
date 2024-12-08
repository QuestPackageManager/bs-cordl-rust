#[cfg(feature = "IMultiplayerLevelEndActionsPublisher")]
#[repr(C)]
#[derive(Debug)]
pub struct IMultiplayerLevelEndActionsPublisher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IMultiplayerLevelEndActionsPublisher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IMultiplayerLevelEndActionsPublisher => ""
    ."IMultiplayerLevelEndActionsPublisher"
);
#[cfg(feature = "IMultiplayerLevelEndActionsPublisher")]
impl std::ops::Deref for IMultiplayerLevelEndActionsPublisher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IMultiplayerLevelEndActionsPublisher")]
impl std::ops::DerefMut for IMultiplayerLevelEndActionsPublisher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IMultiplayerLevelEndActionsPublisher")]
impl IMultiplayerLevelEndActionsPublisher {
    pub fn add_playerNetworkDidFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerNetworkDidFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerNetworkDidFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerNetworkDidFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IMultiplayerLevelEndActionsPublisher")]
impl quest_hook::libil2cpp::ObjectType for IMultiplayerLevelEndActionsPublisher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
