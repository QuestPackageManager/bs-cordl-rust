#[cfg(feature = "Mono+Security+BitConverterLE")]
#[repr(C)]
#[derive(Debug)]
pub struct BitConverterLE {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Security+BitConverterLE")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::BitConverterLE =>
    "Mono.Security"."BitConverterLE"
);
#[cfg(feature = "Mono+Security+BitConverterLE")]
impl std::ops::Deref for crate::Mono::Security::BitConverterLE {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+BitConverterLE")]
impl std::ops::DerefMut for crate::Mono::Security::BitConverterLE {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+BitConverterLE")]
impl crate::Mono::Security::BitConverterLE {
    pub fn GetBytes_i32_0(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetBytes", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes_i64_1(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetBytes", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUIntBytes(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUIntBytes", (bytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetULongBytes(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetULongBytes", (bytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (value, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (value, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn UIntFromBytes(
        dst: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UIntFromBytes", (dst, src, startIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn UShortFromBytes(
        dst: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UShortFromBytes", (dst, src, startIndex))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+BitConverterLE")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::BitConverterLE {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
