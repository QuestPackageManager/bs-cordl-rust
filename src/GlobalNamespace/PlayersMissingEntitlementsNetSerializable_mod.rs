#[cfg(feature = "PlayersMissingEntitlementsNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayersMissingEntitlementsNetSerializable {
    __cordl_parent: crate::GlobalNamespace::PoolableSerializable,
    pub _playersWithoutEntitlements: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "PlayersMissingEntitlementsNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayersMissingEntitlementsNetSerializable => ""
    ."PlayersMissingEntitlementsNetSerializable"
);
#[cfg(feature = "PlayersMissingEntitlementsNetSerializable")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable {
    type Target = crate::GlobalNamespace::PoolableSerializable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayersMissingEntitlementsNetSerializable")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayersMissingEntitlementsNetSerializable")]
impl crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable {
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
        playersWithoutEntitlements: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable = __cordl_object
            .invoke("Init", (playersWithoutEntitlements))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_playersWithoutEntitlements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_playersWithoutEntitlements", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlayersMissingEntitlementsNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayersMissingEntitlementsNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
