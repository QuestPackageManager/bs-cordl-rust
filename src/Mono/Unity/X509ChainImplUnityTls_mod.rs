#[cfg(feature = "Mono+Unity+X509ChainImplUnityTls")]
#[repr(C)]
#[derive(Debug)]
pub struct X509ChainImplUnityTls {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509ChainImpl,
    pub elements: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509ChainElementCollection,
    >,
    pub nativeCertificateChain: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
    pub policy: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy,
    >,
    pub chainStatusList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
        >,
    >,
    pub reverseOrder: bool,
}
#[cfg(feature = "Mono+Unity+X509ChainImplUnityTls")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::X509ChainImplUnityTls =>
    "Mono.Unity"."X509ChainImplUnityTls"
);
#[cfg(feature = "Mono+Unity+X509ChainImplUnityTls")]
impl std::ops::Deref for crate::Mono::Unity::X509ChainImplUnityTls {
    type Target = crate::System::Security::Cryptography::X509Certificates::X509ChainImpl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+X509ChainImplUnityTls")]
impl std::ops::DerefMut for crate::Mono::Unity::X509ChainImplUnityTls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+X509ChainImplUnityTls")]
impl crate::Mono::Unity::X509ChainImplUnityTls {
    pub fn AddStatus(
        &mut self,
        error: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStatus", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Build(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Build", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        nativeCertificateChain: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        reverseOrder: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nativeCertificateChain, reverseOrder))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        nativeCertificateChain: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        reverseOrder: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nativeCertificateChain, reverseOrder))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChainElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainElementCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainElementCollection,
        > = __cordl_object.invoke("get_ChainElements", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChainPolicy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy,
        > = __cordl_object.invoke("get_ChainPolicy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChainStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
            >,
        > = __cordl_object.invoke("get_ChainStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NativeCertificateChain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509list_ref = __cordl_object
            .invoke("get_NativeCertificateChain", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+X509ChainImplUnityTls")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::X509ChainImplUnityTls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
