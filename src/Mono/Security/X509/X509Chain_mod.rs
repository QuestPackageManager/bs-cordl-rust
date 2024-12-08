#[cfg(feature = "Mono+Security+X509+X509Chain")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Chain {
    __cordl_parent: crate::System::Object,
    pub roots: *mut crate::Mono::Security::X509::X509CertificateCollection,
    pub certs: *mut crate::Mono::Security::X509::X509CertificateCollection,
    pub _root: *mut crate::Mono::Security::X509::X509Certificate,
    pub _chain: *mut crate::Mono::Security::X509::X509CertificateCollection,
    pub _status: crate::Mono::Security::X509::X509ChainStatusFlags,
}
#[cfg(feature = "Mono+Security+X509+X509Chain")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::X509Chain =>
    "Mono.Security.X509"."X509Chain"
);
#[cfg(feature = "Mono+Security+X509+X509Chain")]
impl std::ops::Deref for crate::Mono::Security::X509::X509Chain {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509Chain")]
impl std::ops::DerefMut for crate::Mono::Security::X509::X509Chain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509Chain")]
impl crate::Mono::Security::X509::X509Chain {
    pub fn Build(
        &mut self,
        leaf: *mut crate::Mono::Security::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Build", (leaf))?;
        Ok(__cordl_ret)
    }
    pub fn FindCertificateParent(
        &mut self,
        child: *mut crate::Mono::Security::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::X509::X509Certificate = __cordl_object
            .invoke("FindCertificateParent", (child))?;
        Ok(__cordl_ret)
    }
    pub fn FindCertificateRoot(
        &mut self,
        potentialRoot: *mut crate::Mono::Security::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::X509::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::X509::X509Certificate = __cordl_object
            .invoke("FindCertificateRoot", (potentialRoot))?;
        Ok(__cordl_ret)
    }
    pub fn IsParent(
        &mut self,
        child: *mut crate::Mono::Security::X509::X509Certificate,
        parent: *mut crate::Mono::Security::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsParent", (child, parent))?;
        Ok(__cordl_ret)
    }
    pub fn IsTrusted(
        &mut self,
        potentialTrusted: *mut crate::Mono::Security::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTrusted", (potentialTrusted))?;
        Ok(__cordl_ret)
    }
    pub fn IsValid(
        &mut self,
        cert: *mut crate::Mono::Security::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValid", (cert))?;
        Ok(__cordl_ret)
    }
    pub fn LoadCertificates(
        &mut self,
        collection: *mut crate::Mono::Security::X509::X509CertificateCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadCertificates", (collection))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TrustAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::X509::X509CertificateCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::X509::X509CertificateCollection = __cordl_object
            .invoke("get_TrustAnchors", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Security+X509+X509Chain")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::X509::X509Chain {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}