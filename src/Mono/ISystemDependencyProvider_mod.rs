#[cfg(feature = "Mono+ISystemDependencyProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ISystemDependencyProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+ISystemDependencyProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::ISystemDependencyProvider => "Mono"
    ."ISystemDependencyProvider"
);
#[cfg(feature = "Mono+ISystemDependencyProvider")]
impl std::ops::Deref for crate::Mono::ISystemDependencyProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+ISystemDependencyProvider")]
impl std::ops::DerefMut for crate::Mono::ISystemDependencyProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+ISystemDependencyProvider")]
impl crate::Mono::ISystemDependencyProvider {
    pub fn get_CertificateProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::ISystemCertificateProvider> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::ISystemCertificateProvider = __cordl_object
            .invoke("get_CertificateProvider", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Mono+ISystemDependencyProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::ISystemDependencyProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
