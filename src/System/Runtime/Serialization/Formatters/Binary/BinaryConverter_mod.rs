#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BinaryConverter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::Formatters::Binary::BinaryConverter =>
    "System.Runtime.Serialization.Formatters.Binary"."BinaryConverter"
);
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
impl std::ops::Deref
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
impl std::ops::DerefMut
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
impl crate::System::Runtime::Serialization::Formatters::Binary::BinaryConverter {}
#[cfg(feature = "System+Runtime+Serialization+Formatters+Binary+BinaryConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::Formatters::Binary::BinaryConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
