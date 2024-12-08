#[cfg(feature = "System+UncNameHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct UncNameHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+UncNameHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::UncNameHelper => "System"
    ."UncNameHelper"
);
#[cfg(feature = "System+UncNameHelper")]
impl std::ops::Deref for crate::System::UncNameHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+UncNameHelper")]
impl std::ops::DerefMut for crate::System::UncNameHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+UncNameHelper")]
impl crate::System::UncNameHelper {}
#[cfg(feature = "System+UncNameHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::UncNameHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
