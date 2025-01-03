#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSpecificSettingsAtStartNetSerializable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _activePlayerSpecificSettingsAtGameStart_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
        >,
    >,
    pub _activePlayersAtGameStart: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    >,
}
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable => ""
    ."PlayerSpecificSettingsAtStartNetSerializable"
);
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
impl crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_List_1_1(
        activePlayerSpecificSettingsAtGameStart: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (activePlayerSpecificSettingsAtGameStart))?;
        Ok(__cordl_object.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_List_1_1(
        &mut self,
        activePlayerSpecificSettingsAtGameStart: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (activePlayerSpecificSettingsAtGameStart))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activePlayerSpecificSettingsAtGameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
            >,
        > = __cordl_object.invoke("get_activePlayerSpecificSettingsAtGameStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activePlayersAtGameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::IConnectedPlayer,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::IConnectedPlayer,
            >,
        > = __cordl_object.invoke("get_activePlayersAtGameStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_activePlayerSpecificSettingsAtGameStart(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_activePlayerSpecificSettingsAtGameStart", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
