#[cfg(feature = "MultiplayerPlayerResultsData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerPlayerResultsData {
    __cordl_parent: crate::System::Object,
    pub connectedPlayer: *mut IConnectedPlayer,
    pub multiplayerLevelCompletionResults: *mut MultiplayerLevelCompletionResults,
    pub badge: *mut MultiplayerBadgeAwardData,
}
#[cfg(feature = "MultiplayerPlayerResultsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerPlayerResultsData => ""
    ."MultiplayerPlayerResultsData"
);
#[cfg(feature = "MultiplayerPlayerResultsData")]
impl std::ops::Deref for MultiplayerPlayerResultsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayerResultsData")]
impl std::ops::DerefMut for MultiplayerPlayerResultsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlayerResultsData")]
impl MultiplayerPlayerResultsData {
    pub fn CompareTo(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        connectedPlayer: *mut IConnectedPlayer,
        multiplayerLevelCompletionResults: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connectedPlayer, multiplayerLevelCompletionResults))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        connectedPlayer: *mut IConnectedPlayer,
        multiplayerLevelCompletionResults: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connectedPlayer, multiplayerLevelCompletionResults))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerPlayerResultsData")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerPlayerResultsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}