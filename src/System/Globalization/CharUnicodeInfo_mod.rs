#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct CharUnicodeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Globalization::CharUnicodeInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "CharUnicodeInfo";
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
#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
impl std::ops::Deref for crate::System::Globalization::CharUnicodeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
impl std::ops::DerefMut for crate::System::Globalization::CharUnicodeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
impl crate::System::Globalization::CharUnicodeInfo {
    pub fn GetNumericValue(ch: char) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNumericValue", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnicodeCategory_Il2CppString_i32_1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::UnicodeCategory> {
        let __cordl_ret: crate::System::Globalization::UnicodeCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnicodeCategory", (s, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnicodeCategory__cordl_char0(
        ch: char,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::UnicodeCategory> {
        let __cordl_ret: crate::System::Globalization::UnicodeCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnicodeCategory", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnicodeCategory_i32_2(
        codePoint: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::UnicodeCategory> {
        let __cordl_ret: crate::System::Globalization::UnicodeCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnicodeCategory", (codePoint))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalConvertToUtf32_ByRefMut1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        charLength: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalConvertToUtf32", (s, index, charLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalConvertToUtf32_Il2CppString_i32_0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalConvertToUtf32", (s, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetCategoryValue(
        ch: i32,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetCategoryValue", (ch, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetNumericValue(ch: i32) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetNumericValue", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetUnicodeCategory_ByRefMut1(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        charLength: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::UnicodeCategory> {
        let __cordl_ret: crate::System::Globalization::UnicodeCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetUnicodeCategory", (str, index, charLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetUnicodeCategory_Il2CppString_i32_0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Globalization::UnicodeCategory> {
        let __cordl_ret: crate::System::Globalization::UnicodeCategory = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetUnicodeCategory", (value, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWhiteSpace_Il2CppString_i32_0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsWhiteSpace", (s, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWhiteSpace__cordl_char1(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsWhiteSpace", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CategoriesValue() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CategoriesValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CategoryLevel1Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CategoryLevel1Index", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CategoryLevel2Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CategoryLevel2Index", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CategoryLevel3Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_CategoryLevel3Index", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NumericLevel1Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_NumericLevel1Index", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NumericLevel2Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_NumericLevel2Index", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NumericLevel3Index() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_NumericLevel3Index", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NumericValues() -> quest_hook::libil2cpp::Result<
        crate::System::ReadOnlySpan_1<u8>,
    > {
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_NumericValues", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+CharUnicodeInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::CharUnicodeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
