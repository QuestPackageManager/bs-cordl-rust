#[cfg(feature = "System+Buffers+Text+FormattingHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct FormattingHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Buffers+Text+FormattingHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Buffers::Text::FormattingHelpers =>
    "System.Buffers.Text"."FormattingHelpers"
);
#[cfg(feature = "System+Buffers+Text+FormattingHelpers")]
impl std::ops::Deref for crate::System::Buffers::Text::FormattingHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+Text+FormattingHelpers")]
impl std::ops::DerefMut for crate::System::Buffers::Text::FormattingHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+Text+FormattingHelpers")]
impl crate::System::Buffers::Text::FormattingHelpers {
    pub fn CountDigits_u32_1(value: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CountDigits", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CountDigits_u64_0(value: u64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CountDigits", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CountHexDigits(value: u64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CountHexDigits", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Buffers+Text+FormattingHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Buffers::Text::FormattingHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
