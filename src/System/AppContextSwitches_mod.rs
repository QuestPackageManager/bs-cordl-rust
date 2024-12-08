#[cfg(feature = "System+AppContextSwitches")]
#[repr(C)]
#[derive(Debug)]
pub struct AppContextSwitches {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+AppContextSwitches")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::AppContextSwitches => "System"
    ."AppContextSwitches"
);
#[cfg(feature = "System+AppContextSwitches")]
impl std::ops::Deref for crate::System::AppContextSwitches {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppContextSwitches")]
impl std::ops::DerefMut for crate::System::AppContextSwitches {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+AppContextSwitches")]
impl crate::System::AppContextSwitches {}
#[cfg(feature = "System+AppContextSwitches")]
impl quest_hook::libil2cpp::ObjectType for crate::System::AppContextSwitches {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}