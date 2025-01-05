#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
#[repr(C)]
#[derive(Debug)]
pub struct Unsafe {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::Unsafe =>
    "System.Runtime.CompilerServices"."Unsafe"
);
#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::Unsafe {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::Unsafe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
impl crate::System::Runtime::CompilerServices::Unsafe {
    pub fn AddByteOffset_IntPtr0<T>(
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
    pub fn AddByteOffset_u64_1<T>(
        source: quest_hook::libil2cpp::ByRefMut<T>,
        byteOffset: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddByteOffset", (source, byteOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_ByRefMut_IntPtr1<T>(
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
    pub fn Add_Il2CppObject_i32_2<T>(
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
    pub fn AsRef_Il2CppObject0<T>(
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
    pub fn As_Il2CppObject0<T>(
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
    pub fn InitBlockUnaligned(
        startAddress: quest_hook::libil2cpp::ByRefMut<u8>,
        value: u8,
        byteCount: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitBlockUnaligned", (startAddress, value, byteCount))?;
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
    pub fn ReadUnaligned<T>(
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
    pub fn SizeOf<T>() -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SizeOf", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteUnaligned<T>(
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
}
#[cfg(feature = "System+Runtime+CompilerServices+Unsafe")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::Unsafe {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
