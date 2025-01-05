#[cfg(feature = "SelectMultiplayerLobbyDestination")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectMultiplayerLobbyDestination {
    __cordl_parent: crate::GlobalNamespace::MenuDestination,
    pub lobbySecret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub lobbyCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "SelectMultiplayerLobbyDestination")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SelectMultiplayerLobbyDestination => ""
    ."SelectMultiplayerLobbyDestination"
);
#[cfg(feature = "SelectMultiplayerLobbyDestination")]
impl std::ops::Deref for crate::GlobalNamespace::SelectMultiplayerLobbyDestination {
    type Target = crate::GlobalNamespace::MenuDestination;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectMultiplayerLobbyDestination")]
impl std::ops::DerefMut for crate::GlobalNamespace::SelectMultiplayerLobbyDestination {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectMultiplayerLobbyDestination")]
impl crate::GlobalNamespace::SelectMultiplayerLobbyDestination {
    pub fn New_Il2CppString0(
        lobbySecret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lobbyCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lobbySecret, lobbyCode))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString1(
        lobbyCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lobbyCode))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        lobbySecret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lobbyCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lobbySecret, lobbyCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        lobbyCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lobbyCode))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SelectMultiplayerLobbyDestination")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SelectMultiplayerLobbyDestination {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
