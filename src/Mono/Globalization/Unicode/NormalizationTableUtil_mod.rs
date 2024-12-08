#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct NormalizationTableUtil {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Globalization::Unicode::NormalizationTableUtil =>
    "Mono.Globalization.Unicode"."NormalizationTableUtil"
);
#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::NormalizationTableUtil {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::NormalizationTableUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
impl crate::Mono::Globalization::Unicode::NormalizationTableUtil {}
#[cfg(feature = "Mono+Globalization+Unicode+NormalizationTableUtil")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Globalization::Unicode::NormalizationTableUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
