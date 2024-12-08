#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
#[repr(C)]
#[derive(Debug)]
pub struct X509ChainImplMono {
    __cordl_parent: crate::System::Security::Cryptography::X509Certificates::X509ChainImpl,
    pub location: crate::System::Security::Cryptography::X509Certificates::StoreLocation,
    pub elements: *mut crate::System::Security::Cryptography::X509Certificates::X509ChainElementCollection,
    pub policy: *mut crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy,
    pub status: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
    >,
    pub max_path_length: i32,
    pub working_issuer_name: *mut crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName,
    pub working_public_key: *mut crate::System::Security::Cryptography::AsymmetricAlgorithm,
    pub bce_restriction: *mut crate::System::Security::Cryptography::X509Certificates::X509ChainElement,
    pub roots: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    pub cas: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    pub root_store: *mut crate::System::Security::Cryptography::X509Certificates::X509Store,
    pub ca_store: *mut crate::System::Security::Cryptography::X509Certificates::X509Store,
    pub user_root_store: *mut crate::System::Security::Cryptography::X509Certificates::X509Store,
    pub user_ca_store: *mut crate::System::Security::Cryptography::X509Certificates::X509Store,
    pub collection: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::X509Certificates::X509ChainImplMono =>
    "System.Security.Cryptography.X509Certificates"."X509ChainImplMono"
);
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
impl std::ops::Deref
for crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono {
    type Target = crate::System::Security::Cryptography::X509Certificates::X509ChainImpl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
impl crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono {
    pub fn AddStatus(
        &mut self,
        error: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddStatus", (error))?;
        Ok(__cordl_ret)
    }
    pub fn Build(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Build", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn BuildChainFrom(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = __cordl_object
            .invoke("BuildChainFrom", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn CheckRevocationOnChain(
        &mut self,
        flag: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckRevocationOnChain", (flag))?;
        Ok(__cordl_ret)
    }
    pub fn CheckRevocation_X509Certificate2_1(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        ca_cert: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        online: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = __cordl_object
            .invoke("CheckRevocation", (certificate, ca_cert, online))?;
        Ok(__cordl_ret)
    }
    pub fn CheckRevocation_i32_0(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        ca: i32,
        online: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = __cordl_object
            .invoke("CheckRevocation", (certificate, ca, online))?;
        Ok(__cordl_ret)
    }
    pub fn FindCrl(
        &mut self,
        caCertificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Security::X509::X509Crl> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::X509::X509Crl = __cordl_object
            .invoke("FindCrl", (caCertificate))?;
        Ok(__cordl_ret)
    }
    pub fn FindParent(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2 = __cordl_object
            .invoke("FindParent", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn GetSubjectKeyIdentifier(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetSubjectKeyIdentifier", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn IsChainComplete(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsChainComplete", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn IsSelfIssued(
        &mut self,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSelfIssued", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn IsSignedWith(
        &mut self,
        _cordl_signed: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        pubkey: *mut crate::System::Security::Cryptography::AsymmetricAlgorithm,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSignedWith", (_cordl_signed, pubkey))?;
        Ok(__cordl_ret)
    }
    pub fn New(useMachineContext: bool) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useMachineContext))?;
        Ok(__cordl_object)
    }
    pub fn PrepareForNextCertificate(
        &mut self,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareForNextCertificate", (n))?;
        Ok(__cordl_ret)
    }
    pub fn Process(
        &mut self,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Process", (n))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessCertificateExtensions(
        &mut self,
        element: *mut crate::System::Security::Cryptography::X509Certificates::X509ChainElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCertificateExtensions", (element))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessCrlEntryExtensions(
        &mut self,
        entry: *mut crate::Mono::Security::X509::X509Crl_X509CrlEntry,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ProcessCrlEntryExtensions", (entry))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessCrlExtensions(
        &mut self,
        crl: *mut crate::Mono::Security::X509::X509Crl,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessCrlExtensions", (crl))?;
        Ok(__cordl_ret)
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
    pub fn SelectBestFromCollection(
        &mut self,
        child: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        c: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2 = __cordl_object
            .invoke("SelectBestFromCollection", (child, c))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateChain(
        &mut self,
        flag: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateChain", (flag))?;
        Ok(__cordl_ret)
    }
    pub fn WrapUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WrapUp", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        useMachineContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (useMachineContext))?;
        Ok(__cordl_ret)
    }
    pub fn get_CertificateAuthorities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection = __cordl_object
            .invoke("get_CertificateAuthorities", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CertificateCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection = __cordl_object
            .invoke("get_CertificateCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChainElements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509ChainElementCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509ChainElementCollection = __cordl_object
            .invoke("get_ChainElements", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChainPolicy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509ChainPolicy = __cordl_object
            .invoke("get_ChainPolicy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ChainStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Security::Cryptography::X509Certificates::X509ChainStatus,
        > = __cordl_object.invoke("get_ChainStatus", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LMCAStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Store = __cordl_object
            .invoke("get_LMCAStore", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LMRootStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Store = __cordl_object
            .invoke("get_LMRootStore", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Roots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection = __cordl_object
            .invoke("get_Roots", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UserCAStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Store = __cordl_object
            .invoke("get_UserCAStore", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UserRootStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Store = __cordl_object
            .invoke("get_UserRootStore", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+Cryptography+X509Certificates+X509ChainImplMono")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::X509Certificates::X509ChainImplMono {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
