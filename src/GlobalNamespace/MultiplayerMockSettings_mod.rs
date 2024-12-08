#[cfg(feature = "MultiplayerMockSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerMockSettings {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _isEnabled: bool,
    pub _localPlayer: *mut crate::GlobalNamespace::MockPlayerSettings,
    pub _otherPlayers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::MockPlayerSettings,
    >,
    pub _quickplayServer: *mut crate::GlobalNamespace::MockServerSettings,
    pub _multiplayerStatusData: *mut crate::GlobalNamespace::MultiplayerStatusData,
    pub _quickPlaySetupData: *mut crate::GlobalNamespace::QuickPlaySetupData,
}
#[cfg(feature = "MultiplayerMockSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerMockSettings => ""
    ."MultiplayerMockSettings"
);
#[cfg(feature = "MultiplayerMockSettings")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerMockSettings {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerMockSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerMockSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerMockSettings")]
impl crate::GlobalNamespace::MultiplayerMockSettings {
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
    pub fn get_isEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localPlayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::MockPlayerSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MockPlayerSettings = __cordl_object
            .invoke("get_localPlayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multiplayerStatusData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MultiplayerStatusData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MultiplayerStatusData = __cordl_object
            .invoke("get_multiplayerStatusData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_otherPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::MockPlayerSettings,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::MockPlayerSettings,
        > = __cordl_object.invoke("get_otherPlayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_quickPlaySetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::QuickPlaySetupData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::QuickPlaySetupData = __cordl_object
            .invoke("get_quickPlaySetupData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_quickplayServer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::MockServerSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MockServerSettings = __cordl_object
            .invoke("get_quickplayServer", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_isEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_multiplayerStatusData(
        &mut self,
        value: *mut crate::GlobalNamespace::MultiplayerStatusData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_multiplayerStatusData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_quickPlaySetupData(
        &mut self,
        value: *mut crate::GlobalNamespace::QuickPlaySetupData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_quickPlaySetupData", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerMockSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerMockSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
