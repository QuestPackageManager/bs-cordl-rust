#[cfg(feature = "System+Data+Common+SqlConvert")]
#[repr(C)]
#[derive(Debug)]
pub struct SqlConvert {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Data+Common+SqlConvert")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::Common::SqlConvert =>
    "System.Data.Common"."SqlConvert"
);
#[cfg(feature = "System+Data+Common+SqlConvert")]
impl std::ops::Deref for crate::System::Data::Common::SqlConvert {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+SqlConvert")]
impl std::ops::DerefMut for crate::System::Data::Common::SqlConvert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Common+SqlConvert")]
impl crate::System::Data::Common::SqlConvert {}
#[cfg(feature = "System+Data+Common+SqlConvert")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Common::SqlConvert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
