#[cfg(feature = "RomanNumerals")]
#[repr(C)]
#[derive(Debug)]
pub struct RomanNumerals {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "RomanNumerals")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RomanNumerals => ""
    ."RomanNumerals"
);
#[cfg(feature = "RomanNumerals")]
impl std::ops::Deref for crate::GlobalNamespace::RomanNumerals {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RomanNumerals")]
impl std::ops::DerefMut for crate::GlobalNamespace::RomanNumerals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RomanNumerals")]
impl crate::GlobalNamespace::RomanNumerals {}
#[cfg(feature = "RomanNumerals")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::RomanNumerals {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
