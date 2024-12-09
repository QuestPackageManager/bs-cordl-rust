#[cfg(feature = "BGNet+Core+GameLift+PlayerSessionInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSessionInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub playerSessionId: *mut quest_hook::libil2cpp::Il2CppString,
    pub gameSessionId: *mut quest_hook::libil2cpp::Il2CppString,
    pub dnsName: *mut quest_hook::libil2cpp::Il2CppString,
    pub port: i32,
    pub beatmapLevelSelectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    pub gameplayServerConfiguration: crate::GlobalNamespace::GameplayServerConfiguration,
    pub privateGameSecret: *mut quest_hook::libil2cpp::Il2CppString,
    pub privateGameCode: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "BGNet+Core+GameLift+PlayerSessionInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGNet::Core::GameLift::PlayerSessionInfo =>
    "BGNet.Core.GameLift"."PlayerSessionInfo"
);
#[cfg(feature = "BGNet+Core+GameLift+PlayerSessionInfo")]
impl std::ops::Deref for crate::BGNet::Core::GameLift::PlayerSessionInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+PlayerSessionInfo")]
impl std::ops::DerefMut for crate::BGNet::Core::GameLift::PlayerSessionInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGNet+Core+GameLift+PlayerSessionInfo")]
impl crate::BGNet::Core::GameLift::PlayerSessionInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "BGNet+Core+GameLift+PlayerSessionInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGNet::Core::GameLift::PlayerSessionInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
