#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
#[repr(C)]
#[derive(Debug)]
pub struct Converter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::Converter =>
    "System.Runtime.Serialization.Formatters.Binary"."Converter"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::Converter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::Converter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
impl crate::System::Runtime::Serialization::Formatters::Binary::Converter {}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+Converter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::Converter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
