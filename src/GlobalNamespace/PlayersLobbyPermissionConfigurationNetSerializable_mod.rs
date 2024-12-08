#[cfg(feature = "PlayersLobbyPermissionConfigurationNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayersLobbyPermissionConfigurationNetSerializable {
    __cordl_parent: PoolableSerializable,
    pub _playersPermission: *mut crate::System::Collections::Generic::List_1<
        *mut PlayerLobbyPermissionConfigurationNetSerializable,
    >,
}
#[cfg(feature = "PlayersLobbyPermissionConfigurationNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayersLobbyPermissionConfigurationNetSerializable => ""
    ."PlayersLobbyPermissionConfigurationNetSerializable"
);
#[cfg(feature = "PlayersLobbyPermissionConfigurationNetSerializable")]
impl std::ops::Deref for PlayersLobbyPermissionConfigurationNetSerializable {
    type Target = PoolableSerializable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayersLobbyPermissionConfigurationNetSerializable")]
impl std::ops::DerefMut for PlayersLobbyPermissionConfigurationNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayersLobbyPermissionConfigurationNetSerializable")]
impl PlayersLobbyPermissionConfigurationNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        playersPermission: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut PlayerLobbyPermissionConfigurationNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut PlayersLobbyPermissionConfigurationNetSerializable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayersLobbyPermissionConfigurationNetSerializable = __cordl_object
            .invoke("Init", (playersPermission))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret)
    }
    pub fn Retain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Retain", ())?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
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
    pub fn get_playersPermission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut PlayerLobbyPermissionConfigurationNetSerializable,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut PlayerLobbyPermissionConfigurationNetSerializable,
        > = __cordl_object.invoke("get_playersPermission", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlayersLobbyPermissionConfigurationNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for PlayersLobbyPermissionConfigurationNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}