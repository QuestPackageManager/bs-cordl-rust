#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
#[repr(C)]
#[derive(Debug)]
pub struct G_FloatString {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::Utils::NumString::G_FloatString =>
    "Tayx.Graphy.Utils.NumString"."G_FloatString"
);
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
impl std::ops::Deref for crate::Tayx::Graphy::Utils::NumString::G_FloatString {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
impl std::ops::DerefMut for crate::Tayx::Graphy::Utils::NumString::G_FloatString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
impl crate::Tayx::Graphy::Utils::NumString::G_FloatString {
    pub const m_floatFormat: &'static str = "0.0";
}
#[cfg(feature = "Tayx+Graphy+Utils+NumString+G_FloatString")]
impl quest_hook::libil2cpp::ObjectType
for crate::Tayx::Graphy::Utils::NumString::G_FloatString {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
