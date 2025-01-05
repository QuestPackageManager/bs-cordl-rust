#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryPrimitives {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Buffers::Binary::BinaryPrimitives =>
    "System.Buffers.Binary"."BinaryPrimitives"
);
#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
impl std::ops::Deref for crate::System::Buffers::Binary::BinaryPrimitives {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
impl std::ops::DerefMut for crate::System::Buffers::Binary::BinaryPrimitives {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
impl crate::System::Buffers::Binary::BinaryPrimitives {
    pub fn ReverseEndianness_i32_0(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReverseEndianness", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReverseEndianness_i64_1(value: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReverseEndianness", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReverseEndianness_u16_2(value: u16) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReverseEndianness", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReverseEndianness_u32_3(value: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReverseEndianness", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReverseEndianness_u64_4(value: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReverseEndianness", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Buffers+Binary+BinaryPrimitives")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Buffers::Binary::BinaryPrimitives {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
