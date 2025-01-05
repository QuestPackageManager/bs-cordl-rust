#[cfg(feature = "System+Globalization+TextInfoToLowerData")]
#[repr(C)]
#[derive(Debug)]
pub struct TextInfoToLowerData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+TextInfoToLowerData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::TextInfoToLowerData =>
    "System.Globalization"."TextInfoToLowerData"
);
#[cfg(feature = "System+Globalization+TextInfoToLowerData")]
impl std::ops::Deref for crate::System::Globalization::TextInfoToLowerData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+TextInfoToLowerData")]
impl std::ops::DerefMut for crate::System::Globalization::TextInfoToLowerData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+TextInfoToLowerData")]
impl crate::System::Globalization::TextInfoToLowerData {}
#[cfg(feature = "System+Globalization+TextInfoToLowerData")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::TextInfoToLowerData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
