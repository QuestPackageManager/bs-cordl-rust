#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct OptionalAvatarDataPacket {
    __cordl_parent: crate::System::Object,
    pub dataType: u32,
    pub data: *mut ByteArrayNetSerializable,
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::OptionalAvatarDataPacket
    => "BeatSaber.AvatarCore"."OptionalAvatarDataPacket"
);
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
impl crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket {
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
    pub fn Init_OptionalAvatarData0(
        &mut self,
        optionalAvatarData: crate::BeatSaber::AvatarCore::OptionalAvatarData,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket = __cordl_object
            .invoke("Init", (optionalAvatarData))?;
        Ok(__cordl_ret)
    }
    pub fn Init_u32_Il2CppArray_i32_1(
        &mut self,
        dataType: u32,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket = __cordl_object
            .invoke("Init", (dataType, data, length))?;
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
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}