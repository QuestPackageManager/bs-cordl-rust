#[cfg(feature = "ILobbyPlayersDataModel")]
#[repr(C)]
#[derive(Debug)]
pub struct ILobbyPlayersDataModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ILobbyPlayersDataModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ILobbyPlayersDataModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ILobbyPlayersDataModel";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl std::ops::Deref for crate::GlobalNamespace::ILobbyPlayersDataModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::ILobbyPlayersDataModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl crate::GlobalNamespace::ILobbyPlayersDataModel {
    pub fn Activate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Activate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearLocalPlayerBeatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLocalPlayerBeatmapLevel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearLocalPlayerGameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLocalPlayerGameplayModifiers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearRecommendations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearRecommendations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Deactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deactivate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestKickPlayer(
        &mut self,
        kickedUserId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RequestKickPlayer", (kickedUserId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerBeatmapLevel(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerBeatmapLevel", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerGameplayModifiers(
        &mut self,
        modifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerGameplayModifiers", (modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerIsActive(
        &mut self,
        isActive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerIsActive", (isActive))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerIsInLobby(
        &mut self,
        isInLobby: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerIsInLobby", (isInLobby))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLocalPlayerIsReady(
        &mut self,
        isReady: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLocalPlayerIsReady", (isReady))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_localUserId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_localUserId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_partyOwnerId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_partyOwnerId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ILobbyPlayersDataModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
        >,
    >,
> for crate::GlobalNamespace::ILobbyPlayersDataModel {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
        >,
    >,
> for crate::GlobalNamespace::ILobbyPlayersDataModel {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl AsRef<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
        >,
    >,
> for crate::GlobalNamespace::ILobbyPlayersDataModel {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl AsMut<
    crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
        >,
    >,
> for crate::GlobalNamespace::ILobbyPlayersDataModel {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl AsRef<
    crate::System::Collections::Generic::IReadOnlyDictionary_2<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
    >,
> for crate::GlobalNamespace::ILobbyPlayersDataModel {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IReadOnlyDictionary_2<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl AsMut<
    crate::System::Collections::Generic::IReadOnlyDictionary_2<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
    >,
> for crate::GlobalNamespace::ILobbyPlayersDataModel {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyDictionary_2<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILobbyPlayerData>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::GlobalNamespace::ILobbyPlayersDataModel {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ILobbyPlayersDataModel")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::GlobalNamespace::ILobbyPlayersDataModel {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
