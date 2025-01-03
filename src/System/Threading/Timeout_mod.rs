#[cfg(feature = "System+Threading+Timeout")]
#[repr(C)]
#[derive(Debug)]
pub struct Timeout {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+Timeout")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Timeout => "System.Threading"
    ."Timeout"
);
#[cfg(feature = "System+Threading+Timeout")]
impl std::ops::Deref for crate::System::Threading::Timeout {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Timeout")]
impl std::ops::DerefMut for crate::System::Threading::Timeout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Timeout")]
impl crate::System::Threading::Timeout {}
#[cfg(feature = "System+Threading+Timeout")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::Timeout {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
