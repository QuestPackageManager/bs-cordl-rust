#[cfg(feature = "TMPro+CodePoint")]
#[repr(C)]
#[derive(Debug)]
pub struct CodePoint {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+CodePoint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::CodePoint => "TMPro"."CodePoint"
);
#[cfg(feature = "TMPro+CodePoint")]
impl std::ops::Deref for crate::TMPro::CodePoint {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+CodePoint")]
impl std::ops::DerefMut for crate::TMPro::CodePoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+CodePoint")]
impl crate::TMPro::CodePoint {
    pub const APOSTROPHE: u32 = 3227557927u32;
    pub const DOUBLE_QUOTE: u32 = 723854114u32;
    pub const HIGH_SURROGATE_END: u32 = 4292542656u32;
    pub const HIGH_SURROGATE_START: u32 = 14155968u32;
    pub const HYPHEN: u32 = 295702688u32;
    pub const HYPHEN_MINUS: u32 = 2695725101u32;
    pub const LOW_SURROGATE_END: u32 = 4292804800u32;
    pub const LOW_SURROGATE_START: u32 = 14418112u32;
    pub const MINUS: u32 = 2150444589u32;
    pub const NON_BREAKING_HYPHEN: u32 = 195039648u32;
    pub const NUMBER_SIGN: u32 = 757802275u32;
    pub const PERCENTAGE: u32 = 774712101u32;
    pub const PERIOD: u32 = 2910858542u32;
    pub const PLUS: u32 = 758000939u32;
    pub const RIGHT_SINGLE_QUOTATION: u32 = 2686917024u32;
    pub const SOFT_HYPHEN: u32 = 278965632u32;
    pub const SPACE: u32 = 623059488u32;
    pub const UNICODE_PLANE01_START: u32 = 448u32;
    pub const WORD_JOINER: u32 = 12607648u32;
    pub const ZERO_WIDTH_SPACE: u32 = 429919136u32;
}
#[cfg(feature = "TMPro+CodePoint")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::CodePoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
