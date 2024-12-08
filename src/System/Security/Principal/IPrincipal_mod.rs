#[cfg(feature = "System+Security+Principal+IPrincipal")]
#[repr(C)]
#[derive(Debug)]
pub struct IPrincipal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Security+Principal+IPrincipal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Principal::IPrincipal =>
    "System.Security.Principal"."IPrincipal"
);
#[cfg(feature = "System+Security+Principal+IPrincipal")]
impl std::ops::Deref for crate::System::Security::Principal::IPrincipal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Principal+IPrincipal")]
impl std::ops::DerefMut for crate::System::Security::Principal::IPrincipal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Principal+IPrincipal")]
impl crate::System::Security::Principal::IPrincipal {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Security+Principal+IPrincipal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Principal::IPrincipal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
