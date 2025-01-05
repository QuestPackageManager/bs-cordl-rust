#[cfg(feature = "IBitMaskUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct IBitMaskUtil {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "IBitMaskUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IBitMaskUtil => ""
    ."IBitMaskUtil"
);
#[cfg(feature = "IBitMaskUtil")]
impl std::ops::Deref for crate::GlobalNamespace::IBitMaskUtil {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBitMaskUtil")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBitMaskUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBitMaskUtil")]
impl crate::GlobalNamespace::IBitMaskUtil {
    pub fn FromBytes<T>(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBytes", (bytes, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBase64Char(digit: u64) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBase64Char", (digit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBase64Digit(c: char) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBase64Digit", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHexDigit(c: char) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHexDigit", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn NumberOfSetBits<T>(bitMask: T) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NumberOfSetBits", (bitMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBytes<T>(
        bitMask: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToBytes", (bitMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToShortString<T>(
        bitMask: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToShortString", (bitMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ByRefMut0<T>(
        stringSerializedMask: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        bitMask: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (stringSerializedMask, bitMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_i32_i32_ByRefMut1<T>(
        stringSerializedMask: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        offset: i32,
        length: i32,
        bitMask: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (stringSerializedMask, offset, length, bitMask))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IBitMaskUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IBitMaskUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
