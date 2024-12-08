#[cfg(feature = "System+Data+SqlTypes+SQLResource")]
#[repr(C)]
#[derive(Debug)]
pub struct SQLResource {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Data+SqlTypes+SQLResource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::SqlTypes::SQLResource =>
    "System.Data.SqlTypes"."SQLResource"
);
#[cfg(feature = "System+Data+SqlTypes+SQLResource")]
impl std::ops::Deref for crate::System::Data::SqlTypes::SQLResource {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SQLResource")]
impl std::ops::DerefMut for crate::System::Data::SqlTypes::SQLResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+SqlTypes+SQLResource")]
impl crate::System::Data::SqlTypes::SQLResource {}
#[cfg(feature = "System+Data+SqlTypes+SQLResource")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::SqlTypes::SQLResource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
