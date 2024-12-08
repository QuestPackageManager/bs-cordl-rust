#[cfg(feature = "System+Net+ICredentialPolicy")]
#[repr(C)]
#[derive(Debug)]
pub struct ICredentialPolicy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+ICredentialPolicy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ICredentialPolicy => "System.Net"
    ."ICredentialPolicy"
);
#[cfg(feature = "System+Net+ICredentialPolicy")]
impl std::ops::Deref for crate::System::Net::ICredentialPolicy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ICredentialPolicy")]
impl std::ops::DerefMut for crate::System::Net::ICredentialPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ICredentialPolicy")]
impl crate::System::Net::ICredentialPolicy {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Net+ICredentialPolicy")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ICredentialPolicy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
