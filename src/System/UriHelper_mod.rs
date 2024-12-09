#[cfg(feature = "System+UriHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct UriHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+UriHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::UriHelper => "System"."UriHelper"
);
#[cfg(feature = "System+UriHelper")]
impl std::ops::Deref for crate::System::UriHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+UriHelper")]
impl std::ops::DerefMut for crate::System::UriHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+UriHelper")]
impl crate::System::UriHelper {}
#[cfg(feature = "System+UriHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::UriHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
