#[cfg(feature = "System+Globalization+GlobalizationMode")]
#[repr(C)]
#[derive(Debug)]
pub struct GlobalizationMode {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Globalization+GlobalizationMode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::GlobalizationMode =>
    "System.Globalization"."GlobalizationMode"
);
#[cfg(feature = "System+Globalization+GlobalizationMode")]
impl std::ops::Deref for crate::System::Globalization::GlobalizationMode {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+GlobalizationMode")]
impl std::ops::DerefMut for crate::System::Globalization::GlobalizationMode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+GlobalizationMode")]
impl crate::System::Globalization::GlobalizationMode {}
#[cfg(feature = "System+Globalization+GlobalizationMode")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::GlobalizationMode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
