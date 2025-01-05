#[cfg(feature = "System+Buffer")]
#[repr(C)]
#[derive(Debug)]
pub struct Buffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Buffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Buffer => "System"."Buffer"
);
#[cfg(feature = "System+Buffer")]
impl std::ops::Deref for crate::System::Buffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffer")]
impl std::ops::DerefMut for crate::System::Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffer")]
impl crate::System::Buffer {
    pub fn BlockCopy(
        src: quest_hook::libil2cpp::Gc<crate::System::Array>,
        srcOffset: i32,
        dst: quest_hook::libil2cpp::Gc<crate::System::Array>,
        dstOffset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BlockCopy", (src, srcOffset, dst, dstOffset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ByteLength(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ByteLength", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfByte(
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: u8,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfByte", (src, value, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalBlockCopy(
        src: quest_hook::libil2cpp::Gc<crate::System::Array>,
        srcOffsetBytes: i32,
        dst: quest_hook::libil2cpp::Gc<crate::System::Array>,
        dstOffsetBytes: i32,
        byteCount: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InternalBlockCopy",
                (src, srcOffsetBytes, dst, dstOffsetBytes, byteCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalMemcpy(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalMemcpy", (dest, src, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Memcpy_Il2CppObject_i32_1(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Memcpy", (dest, src, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn Memcpy_i32_Il2CppArray_i32_i32_0(
        pDest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: i32,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        srcIndex: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Memcpy", (pDest, destIndex, src, srcIndex, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn Memmove_ByRefMut_ByRefMut_u64_1<T>(
        destination: quest_hook::libil2cpp::ByRefMut<T>,
        source: quest_hook::libil2cpp::ByRefMut<T>,
        elementCount: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Memmove", (destination, source, elementCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn Memmove_Il2CppObject_Il2CppObject_u32_0(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Memmove", (dest, src, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn MemoryCopy(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destinationSizeInBytes: i64,
        sourceBytesToCopy: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MemoryCopy",
                (source, destination, destinationSizeInBytes, sourceBytesToCopy),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ZeroMemory(
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ZeroMemory", (src, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ByteLength(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("_ByteLength", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn memcpy1(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("memcpy1", (dest, src, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn memcpy2(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("memcpy2", (dest, src, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn memcpy4(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("memcpy4", (dest, src, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Buffer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Buffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
