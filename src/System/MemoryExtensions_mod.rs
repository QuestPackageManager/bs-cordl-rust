#[cfg(feature = "System+MemoryExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct MemoryExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+MemoryExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::MemoryExtensions => "System"
    ."MemoryExtensions"
);
#[cfg(feature = "System+MemoryExtensions")]
impl std::ops::Deref for crate::System::MemoryExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+MemoryExtensions")]
impl std::ops::DerefMut for crate::System::MemoryExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+MemoryExtensions")]
impl crate::System::MemoryExtensions {
    pub fn AsSpan_Gc1(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsSpan", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsSpan_i32_0<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Span_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Span_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsSpan", (array, start))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsSpan_i32_2(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsSpan", (text, start))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsSpan_i32_i32_3(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsSpan", (text, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsSpan_i32_i32_4<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Span_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Span_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AsSpan", (array, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains(
        source: crate::System::ReadOnlySpan_1<char>,
        value: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (source, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo<T>(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        destination: crate::System::Span_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyTo", (source, destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndsWith_ReadOnlySpan_1_ReadOnlySpan_1_1<T>(
        span: crate::System::ReadOnlySpan_1<T>,
        value: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndsWith", (span, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndsWith_StringComparison0(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
        comparisonType: crate::System::StringComparison,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndsWith", (span, value, comparisonType))?;
        Ok(__cordl_ret.into())
    }
    pub fn EqualsOrdinal(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EqualsOrdinal", (span, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn EqualsOrdinalIgnoreCase(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EqualsOrdinalIgnoreCase", (span, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf<T>(
        span: crate::System::ReadOnlySpan_1<T>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (span, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfAny<T>(
        span: crate::System::ReadOnlySpan_1<T>,
        values: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfAny", (span, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTypeComparableAsBytes<T>(
        _cordl_size: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTypeComparableAsBytes", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual<T>(
        span: crate::System::ReadOnlySpan_1<T>,
        other: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SequenceEqual", (span, other))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartsWith<T>(
        span: crate::System::ReadOnlySpan_1<T>,
        value: crate::System::ReadOnlySpan_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StartsWith", (span, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUpperInvariant(
        source: crate::System::ReadOnlySpan_1<char>,
        destination: crate::System::Span_1<char>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUpperInvariant", (source, destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn Trim(
        span: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Trim", (span))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrimEnd(
        span: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrimEnd", (span))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrimStart(
        span: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TrimStart", (span))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+MemoryExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::System::MemoryExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
