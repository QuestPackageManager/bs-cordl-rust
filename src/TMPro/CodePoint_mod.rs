#[cfg(feature = "cordl_class_TMPro+CodePoint")]
#[repr(C)]
#[derive(Debug)]
pub struct CodePoint {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_TMPro+CodePoint")]
unsafe impl quest_hook::libil2cpp::Type for crate::TMPro::CodePoint {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "TMPro";
    const CLASS_NAME: &'static str = "CodePoint";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "TMPro+CodePoint")]
impl std::ops::Deref for crate::TMPro::CodePoint {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+CodePoint")]
impl std::ops::DerefMut for crate::TMPro::CodePoint {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+CodePoint")]
impl crate::TMPro::CodePoint {
    pub const APOSTROPHE: u32 = 39u32;
    pub const DOUBLE_QUOTE: u32 = 34u32;
    pub const HIGH_SURROGATE_END: u32 = 56319u32;
    pub const HIGH_SURROGATE_START: u32 = 55296u32;
    pub const HYPHEN: u32 = 8208u32;
    pub const HYPHEN_MINUS: u32 = 45u32;
    pub const LOW_SURROGATE_END: u32 = 57343u32;
    pub const LOW_SURROGATE_START: u32 = 56320u32;
    pub const MINUS: u32 = 45u32;
    pub const NON_BREAKING_HYPHEN: u32 = 8209u32;
    pub const NUMBER_SIGN: u32 = 35u32;
    pub const PERCENTAGE: u32 = 37u32;
    pub const PERIOD: u32 = 46u32;
    pub const PLUS: u32 = 43u32;
    pub const RIGHT_SINGLE_QUOTATION: u32 = 8217u32;
    pub const SOFT_HYPHEN: u32 = 173u32;
    pub const SPACE: u32 = 32u32;
    pub const UNICODE_PLANE01_START: u32 = 65536u32;
    pub const WORD_JOINER: u32 = 8288u32;
    pub const ZERO_WIDTH_SPACE: u32 = 8203u32;
}
#[cfg(feature = "cordl_class_TMPro+CodePoint")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::CodePoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
