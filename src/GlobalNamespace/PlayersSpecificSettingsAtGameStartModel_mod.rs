#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayersSpecificSettingsAtGameStartModel {
    __cordl_parent: crate::System::Object,
    pub _localPlayerSpecificSettings_k__BackingField: *mut PlayerSpecificSettingsNetSerializable,
    pub _playersAtGameStartNetSerializable_k__BackingField: *mut PlayerSpecificSettingsAtStartNetSerializable,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
}
#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayersSpecificSettingsAtGameStartModel => ""
    ."PlayersSpecificSettingsAtGameStartModel"
);
#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
impl std::ops::Deref for PlayersSpecificSettingsAtGameStartModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
impl std::ops::DerefMut for PlayersSpecificSettingsAtGameStartModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
impl PlayersSpecificSettingsAtGameStartModel {
    #[cfg(feature = "PlayersSpecificSettingsAtGameStartModel+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel___c__DisplayClass13_0;
    pub fn set_localPlayerSpecificSettings(
        &mut self,
        value: *mut PlayerSpecificSettingsNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_localPlayerSpecificSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        localPlayerSpecificSettings: *mut PlayerSpecificSettingsNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSessionManager, localPlayerSpecificSettings))?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayerSpecificSettingsForUserId(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerSpecificSettingsNetSerializable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerSpecificSettingsNetSerializable = __cordl_object
            .invoke("GetPlayerSpecificSettingsForUserId", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn get_playersAtGameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut IConnectedPlayer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut IConnectedPlayer,
        > = __cordl_object.invoke("get_playersAtGameStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localPlayerSpecificSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerSpecificSettingsNetSerializable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerSpecificSettingsNetSerializable = __cordl_object
            .invoke("get_localPlayerSpecificSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn SaveFromNetSerializable(
        &mut self,
        playersAtGameStartNetSerializable: *mut PlayerSpecificSettingsAtStartNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveFromNetSerializable", (playersAtGameStartNetSerializable))?;
        Ok(__cordl_ret)
    }
    pub fn get_playersAtGameStartNetSerializable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut PlayerSpecificSettingsAtStartNetSerializable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerSpecificSettingsAtStartNetSerializable = __cordl_object
            .invoke("get_playersAtGameStartNetSerializable", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_playersAtGameStartNetSerializable(
        &mut self,
        value: *mut PlayerSpecificSettingsAtStartNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playersAtGameStartNetSerializable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        localPlayerSpecificSettings: *mut PlayerSpecificSettingsNetSerializable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (multiplayerSessionManager, localPlayerSpecificSettings),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayersSpecificSettingsAtGameStartModel")]
impl quest_hook::libil2cpp::ObjectType for PlayersSpecificSettingsAtGameStartModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
