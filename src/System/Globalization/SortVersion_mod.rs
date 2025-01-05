#[cfg(feature = "System+Globalization+SortVersion")]
#[repr(C)]
#[derive(Debug)]
pub struct SortVersion {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_NlsVersion: i32,
    pub m_SortId: crate::System::Guid,
}
#[cfg(feature = "System+Globalization+SortVersion")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::SortVersion =>
    "System.Globalization"."SortVersion"
);
#[cfg(feature = "System+Globalization+SortVersion")]
impl std::ops::Deref for crate::System::Globalization::SortVersion {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+SortVersion")]
impl std::ops::DerefMut for crate::System::Globalization::SortVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+SortVersion")]
impl crate::System::Globalization::SortVersion {}
#[cfg(feature = "System+Globalization+SortVersion")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::SortVersion {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
