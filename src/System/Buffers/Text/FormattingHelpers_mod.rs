#[cfg(feature = "System+Buffers+Text+FormattingHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct FormattingHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Buffers+Text+FormattingHelpers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Buffers::Text::FormattingHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Buffers.Text";
    const CLASS_NAME: &'static str = "FormattingHelpers";
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
