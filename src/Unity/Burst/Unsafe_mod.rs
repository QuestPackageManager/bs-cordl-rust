#[cfg(feature = "Unity+Burst+Unsafe")]
#[repr(C)]
#[derive(Debug)]
pub struct Unsafe {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Unity+Burst+Unsafe")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Unsafe => "Unity.Burst"."Unsafe"
);
#[cfg(feature = "Unity+Burst+Unsafe")]
impl std::ops::Deref for crate::Unity::Burst::Unsafe {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Unsafe")]
impl std::ops::DerefMut for crate::Unity::Burst::Unsafe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Unsafe")]
impl crate::Unity::Burst::Unsafe {
    pub fn AddByteOffset<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
        byteOffset: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddByteOffset", (source, byteOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_ByRefMut_IntPtr2<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
        elementOffset: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (source, elementOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_ByRefMut_i32_0<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
        elementOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (source, elementOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_Gc_i32_1<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        elementOffset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (source, elementOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreSame<T>(
        left: quest_hook::libil2cpp::ByRefMut<T>,
        right: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreSame", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsPointer<T>(
        value: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("AsPointer", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsRef_ByRefMut1<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsRef", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsRef_Gc0<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsRef", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn As_ByRefMut1<TFrom, TTo>(
        source: quest_hook::libil2cpp::ByRefMut<TFrom>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<TTo>>
    where
        TFrom: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TTo: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<TTo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("As", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn As_Gc0<T>(
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("As", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn ByteOffset<T>(
        origin: quest_hook::libil2cpp::ByRefMut<T>,
        target: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ByteOffset", (origin, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyBlockUnaligned_ByRefMut_ByRefMut1(
        destination: quest_hook::libil2cpp::ByRefMut<u8>,
        source: quest_hook::libil2cpp::ByRefMut<u8>,
        byteCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyBlockUnaligned", (destination, source, byteCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyBlockUnaligned_Gc_Gc0(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        byteCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyBlockUnaligned", (destination, source, byteCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyBlock_ByRefMut_ByRefMut1(
        destination: quest_hook::libil2cpp::ByRefMut<u8>,
        source: quest_hook::libil2cpp::ByRefMut<u8>,
        byteCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyBlock", (destination, source, byteCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyBlock_Gc_Gc0(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        byteCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyBlock", (destination, source, byteCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_ByRefMut_Gc1<T>(
        destination: quest_hook::libil2cpp::ByRefMut<T>,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy", (destination, source))?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy_Gc_ByRefMut0<T>(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        source: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Copy", (destination, source))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitBlockUnaligned_ByRefMut1(
        startAddress: quest_hook::libil2cpp::ByRefMut<u8>,
        value: u8,
        byteCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitBlockUnaligned", (startAddress, value, byteCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitBlockUnaligned_Gc0(
        startAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: u8,
        byteCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitBlockUnaligned", (startAddress, value, byteCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitBlock_ByRefMut1(
        startAddress: quest_hook::libil2cpp::ByRefMut<u8>,
        value: u8,
        byteCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitBlock", (startAddress, value, byteCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitBlock_Gc0(
        startAddress: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: u8,
        byteCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitBlock", (startAddress, value, byteCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAddressGreaterThan<T>(
        left: quest_hook::libil2cpp::ByRefMut<T>,
        right: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAddressGreaterThan", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAddressLessThan<T>(
        left: quest_hook::libil2cpp::ByRefMut<T>,
        right: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAddressLessThan", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Read<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Read", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnaligned_ByRefMut1<T>(
        source: quest_hook::libil2cpp::ByRefMut<u8>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUnaligned", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadUnaligned_Gc0<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadUnaligned", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn SizeOf<T>() -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SizeOf", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SubtractByteOffset<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
        byteOffset: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubtractByteOffset", (source, byteOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract_ByRefMut_IntPtr2<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
        elementOffset: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Subtract", (source, elementOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract_ByRefMut_i32_0<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
        elementOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Subtract", (source, elementOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract_Gc_i32_1<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        elementOffset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Subtract", (source, elementOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unbox<T>(
        _cordl_box: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unbox", (_cordl_box))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write<T>(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write", (destination, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUnaligned_ByRefMut1<T>(
        destination: quest_hook::libil2cpp::ByRefMut<u8>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUnaligned", (destination, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUnaligned_Gc0<T>(
        destination: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteUnaligned", (destination, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Unsafe")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Unsafe {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
