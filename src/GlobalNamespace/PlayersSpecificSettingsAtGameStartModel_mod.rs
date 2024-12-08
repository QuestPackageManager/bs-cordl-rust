#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayersSpecificSettingsAtGameStartModel {
    __cordl_parent: crate::System::Object,
    pub _localPlayerSpecificSettings_k__BackingField: *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
    pub _playersAtGameStartNetSerializable_k__BackingField: *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
}
#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel => ""
    ."PlayersSpecificSettingsAtGameStartModel"
);
#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
impl crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel {
    #[cfg(feature = "PlayersSpecificSettingsAtGameStartModel+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel___c__DisplayClass13_0;
    pub fn GetPlayerSpecificSettingsForUserId(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable = __cordl_object
            .invoke("GetPlayerSpecificSettingsForUserId", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
        localPlayerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (multiplayerSessionManager, localPlayerSpecificSettings),
            )?;
        Ok(__cordl_object)
    }
    pub fn SaveFromNetSerializable(
        &mut self,
        playersAtGameStartNetSerializable: *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveFromNetSerializable", (playersAtGameStartNetSerializable))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
        localPlayerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSessionManager, localPlayerSpecificSettings))?;
        Ok(__cordl_ret)
    }
    pub fn get_localPlayerSpecificSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable = __cordl_object
            .invoke("get_localPlayerSpecificSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playersAtGameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        > = __cordl_object.invoke("get_playersAtGameStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playersAtGameStartNetSerializable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable = __cordl_object
            .invoke("get_playersAtGameStartNetSerializable", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_localPlayerSpecificSettings(
        &mut self,
        value: *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_localPlayerSpecificSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playersAtGameStartNetSerializable(
        &mut self,
        value: *mut crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playersAtGameStartNetSerializable", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
