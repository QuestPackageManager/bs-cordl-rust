#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerSpecificSettingsAtStartNetSerializable {
    __cordl_parent: crate::System::Object,
    pub _activePlayerSpecificSettingsAtGameStart_k__BackingField: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut PlayerSpecificSettingsNetSerializable,
    >,
    pub _activePlayersAtGameStart: *mut crate::System::Collections::Generic::List_1<
        *mut IConnectedPlayer,
    >,
}
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayerSpecificSettingsAtStartNetSerializable => ""
    ."PlayerSpecificSettingsAtStartNetSerializable"
);
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
impl std::ops::Deref for PlayerSpecificSettingsAtStartNetSerializable {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
impl std::ops::DerefMut for PlayerSpecificSettingsAtStartNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
impl PlayerSpecificSettingsAtStartNetSerializable {
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
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_List_1_1(
        &mut self,
        activePlayerSpecificSettingsAtGameStart: *mut crate::System::Collections::Generic::List_1<
            *mut PlayerSpecificSettingsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (activePlayerSpecificSettingsAtGameStart))?;
        Ok(__cordl_ret)
    }
    pub fn set_activePlayerSpecificSettingsAtGameStart(
        &mut self,
        value: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut PlayerSpecificSettingsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_activePlayerSpecificSettingsAtGameStart", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_activePlayerSpecificSettingsAtGameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut PlayerSpecificSettingsNetSerializable,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut PlayerSpecificSettingsNetSerializable,
        > = __cordl_object.invoke("get_activePlayerSpecificSettingsAtGameStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_activePlayersAtGameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut IConnectedPlayer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut IConnectedPlayer,
        > = __cordl_object.invoke("get_activePlayersAtGameStart", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_List_1_1(
        activePlayerSpecificSettingsAtGameStart: *mut crate::System::Collections::Generic::List_1<
            *mut PlayerSpecificSettingsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (activePlayerSpecificSettingsAtGameStart))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerSpecificSettingsAtStartNetSerializable")]
impl quest_hook::libil2cpp::ObjectType for PlayerSpecificSettingsAtStartNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
