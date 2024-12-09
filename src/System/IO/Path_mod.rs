#[cfg(feature = "System+IO+Path")]
#[repr(C)]
#[derive(Debug)]
pub struct Path {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+Path")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Path => "System.IO"."Path"
);
#[cfg(feature = "System+IO+Path")]
impl std::ops::Deref for crate::System::IO::Path {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Path")]
impl std::ops::DerefMut for crate::System::IO::Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Path")]
impl crate::System::IO::Path {
    #[cfg(feature = "System+IO+Path+__c")]
    pub type __c = crate::System::IO::Path___c;
}
#[cfg(feature = "System+IO+Path")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::Path {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
