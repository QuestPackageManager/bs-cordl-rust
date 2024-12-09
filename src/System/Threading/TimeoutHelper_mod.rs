#[cfg(feature = "System+Threading+TimeoutHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeoutHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+TimeoutHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::TimeoutHelper =>
    "System.Threading"."TimeoutHelper"
);
#[cfg(feature = "System+Threading+TimeoutHelper")]
impl std::ops::Deref for crate::System::Threading::TimeoutHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+TimeoutHelper")]
impl std::ops::DerefMut for crate::System::Threading::TimeoutHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+TimeoutHelper")]
impl crate::System::Threading::TimeoutHelper {}
#[cfg(feature = "System+Threading+TimeoutHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::TimeoutHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
