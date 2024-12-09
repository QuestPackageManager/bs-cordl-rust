#[cfg(feature = "System+Globalization+EncodingTable")]
#[repr(C)]
#[derive(Debug)]
pub struct EncodingTable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+EncodingTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::EncodingTable =>
    "System.Globalization"."EncodingTable"
);
#[cfg(feature = "System+Globalization+EncodingTable")]
impl std::ops::Deref for crate::System::Globalization::EncodingTable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+EncodingTable")]
impl std::ops::DerefMut for crate::System::Globalization::EncodingTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+EncodingTable")]
impl crate::System::Globalization::EncodingTable {}
#[cfg(feature = "System+Globalization+EncodingTable")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::EncodingTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
