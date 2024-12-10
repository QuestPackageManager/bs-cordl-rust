#[cfg(feature = "Mono+SystemDependencyProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemDependencyProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _CertificateProvider_k__BackingField: *mut crate::Mono::SystemCertificateProvider,
}
#[cfg(feature = "Mono+SystemDependencyProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::SystemDependencyProvider => "Mono"
    ."SystemDependencyProvider"
);
#[cfg(feature = "Mono+SystemDependencyProvider")]
impl std::ops::Deref for crate::Mono::SystemDependencyProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+SystemDependencyProvider")]
impl std::ops::DerefMut for crate::Mono::SystemDependencyProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+SystemDependencyProvider")]
impl crate::Mono::SystemDependencyProvider {
    pub fn Mono_ISystemDependencyProvider_get_CertificateProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::ISystemCertificateProvider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::ISystemCertificateProvider,
        > = __cordl_object
            .invoke("Mono.ISystemDependencyProvider.get_CertificateProvider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CertificateProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::SystemCertificateProvider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::SystemCertificateProvider,
        > = __cordl_object.invoke("get_CertificateProvider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_X509Pal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::X509PalImpl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::X509PalImpl> = __cordl_object
            .invoke("get_X509Pal", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+SystemDependencyProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::SystemDependencyProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
