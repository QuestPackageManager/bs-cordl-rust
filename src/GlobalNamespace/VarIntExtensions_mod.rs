#[cfg(feature = "VarIntExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct VarIntExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "VarIntExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::VarIntExtensions => ""
    ."VarIntExtensions"
);
#[cfg(feature = "VarIntExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::VarIntExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VarIntExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::VarIntExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VarIntExtensions")]
impl crate::GlobalNamespace::VarIntExtensions {
    pub fn GetSize_i32_0(val: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSize", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSize_i64_2(val: i64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSize", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSize_u32_1(val: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSize", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSize_u64_3(val: u64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSize", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVarInt(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVarInt", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVarLong(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVarLong", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVarUInt(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVarUInt", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVarULong(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVarULong", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn PutVarInt(
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        val: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PutVarInt", (writer, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn PutVarLong(
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        val: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PutVarLong", (writer, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn PutVarUInt(
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        val: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PutVarUInt", (writer, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn PutVarULong(
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        val: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PutVarULong", (writer, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetVarUInt(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        value: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetVarUInt", (reader, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetVarULong(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        value: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetVarULong", (reader, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VarIntExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::VarIntExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
