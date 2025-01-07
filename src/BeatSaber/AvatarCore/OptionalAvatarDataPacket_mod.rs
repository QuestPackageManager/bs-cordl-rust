#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct OptionalAvatarDataPacket {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub dataType: u32,
    pub data: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ByteArrayNetSerializable,
    >,
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "OptionalAvatarDataPacket";
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
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_OptionalAvatarData0(
        &mut self,
        optionalAvatarData: crate::BeatSaber::AvatarCore::OptionalAvatarData,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
        > = __cordl_object.invoke("Init", (optionalAvatarData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_u32_Il2CppArray_i32_1(
        &mut self,
        dataType: u32,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
        > = __cordl_object.invoke("Init", (dataType, data, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarDataPacket")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::BeatSaber::AvatarCore::OptionalAvatarDataPacket {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
