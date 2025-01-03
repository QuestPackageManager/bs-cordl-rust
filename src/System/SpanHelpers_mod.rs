#[cfg(feature = "System+SpanHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct SpanHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+SpanHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::SpanHelpers => "System"."SpanHelpers"
);
#[cfg(feature = "System+SpanHelpers")]
impl std::ops::Deref for crate::System::SpanHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+SpanHelpers")]
impl std::ops::DerefMut for crate::System::SpanHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+SpanHelpers")]
impl crate::System::SpanHelpers {
    pub fn ClearWithReferences(
        ip: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        pointerSizeLength: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearWithReferences", (ip, pointerSizeLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearWithoutReferences(
        b: quest_hook::libil2cpp::ByRefMut<u8>,
        byteLength: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearWithoutReferences", (b, byteLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndsWithCultureHelper(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
        compareInfo: quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndsWithCultureHelper", (span, value, compareInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndsWithCultureIgnoreCaseHelper(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
        compareInfo: quest_hook::libil2cpp::Gc<crate::System::Globalization::CompareInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndsWithCultureIgnoreCaseHelper", (span, value, compareInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndsWithOrdinalIgnoreCaseHelper(
        span: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EndsWithOrdinalIgnoreCaseHelper", (span, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfAny_ByRefMut_i32_ByRefMut_i32_0(
        searchSpace: quest_hook::libil2cpp::ByRefMut<u8>,
        searchSpaceLength: i32,
        value: quest_hook::libil2cpp::ByRefMut<u8>,
        valueLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfAny", (searchSpace, searchSpaceLength, value, valueLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfAny_ByRefMut_i32_ByRefMut_i32_1<T>(
        searchSpace: quest_hook::libil2cpp::ByRefMut<T>,
        searchSpaceLength: i32,
        value: quest_hook::libil2cpp::ByRefMut<T>,
        valueLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfAny", (searchSpace, searchSpaceLength, value, valueLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_T2<T>(
        searchSpace: quest_hook::libil2cpp::ByRefMut<T>,
        value: T,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (searchSpace, value, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf__cordl_char1(
        searchSpace: quest_hook::libil2cpp::ByRefMut<char>,
        value: char,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (searchSpace, value, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_u8_0(
        searchSpace: quest_hook::libil2cpp::ByRefMut<u8>,
        value: u8,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (searchSpace, value, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf(
        searchSpace: quest_hook::libil2cpp::ByRefMut<char>,
        value: char,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LastIndexOf", (searchSpace, value, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocateFirstFoundChar_Vector_1_0(
        _cordl_match: crate::System::Numerics::Vector_1<u16>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocateFirstFoundChar", (_cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocateFirstFoundChar_u64_1(
        _cordl_match: u64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocateFirstFoundChar", (_cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocateLastFoundChar_Vector_1_0(
        _cordl_match: crate::System::Numerics::Vector_1<u16>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocateLastFoundChar", (_cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocateLastFoundChar_u64_1(
        _cordl_match: u64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocateLastFoundChar", (_cordl_match))?;
        Ok(__cordl_ret.into())
    }
    pub fn SequenceCompareTo(
        first: quest_hook::libil2cpp::ByRefMut<char>,
        firstLength: i32,
        second: quest_hook::libil2cpp::ByRefMut<char>,
        secondLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SequenceCompareTo", (first, firstLength, second, secondLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual_i32_1<T>(
        first: quest_hook::libil2cpp::ByRefMut<T>,
        second: quest_hook::libil2cpp::ByRefMut<T>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SequenceEqual", (first, second, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual_u64_0(
        first: quest_hook::libil2cpp::ByRefMut<u8>,
        second: quest_hook::libil2cpp::ByRefMut<u8>,
        length: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SequenceEqual", (first, second, length))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+SpanHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::SpanHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
