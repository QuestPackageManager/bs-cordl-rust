#[cfg(feature = "System+Net+EndPointManager")]
#[repr(C)]
#[derive(Debug)]
pub struct EndPointManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+EndPointManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::EndPointManager => "System.Net"
    ."EndPointManager"
);
#[cfg(feature = "System+Net+EndPointManager")]
impl std::ops::Deref for crate::System::Net::EndPointManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+EndPointManager")]
impl std::ops::DerefMut for crate::System::Net::EndPointManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+EndPointManager")]
impl crate::System::Net::EndPointManager {}
#[cfg(feature = "System+Net+EndPointManager")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::EndPointManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
