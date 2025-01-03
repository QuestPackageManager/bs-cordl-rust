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
    pub fn BuildChainFrom(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = __cordl_object
            .invoke("BuildChainFrom", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCrls(
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ski: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        store: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Store>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Crl,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckCrls", (subject, ski, store))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn CheckRevocation_X509Certificate2_1(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        ca_cert: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        online: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = __cordl_object
            .invoke("CheckRevocation", (certificate, ca_cert, online))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckRevocation_i32_0(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn FindCrl(
        &mut self,
        caCertificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Crl,
        > = __cordl_object.invoke("FindCrl", (caCertificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindParent(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        > = __cordl_object.invoke("FindParent", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAuthorityKeyIdentifier_X509Certificate2_0(
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAuthorityKeyIdentifier", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAuthorityKeyIdentifier_X509Crl1(
        crl: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAuthorityKeyIdentifier", (crl))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAuthorityKeyIdentifier_X509Extension2(
        ext: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Extension>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAuthorityKeyIdentifier", (ext))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSubjectKeyIdentifier(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetSubjectKeyIdentifier", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsChainComplete(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsChainComplete", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSelfIssued(
        &mut self,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSelfIssued", (certificate))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSignedWith(
        &mut self,
        _cordl_signed: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        pubkey: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSignedWith", (_cordl_signed, pubkey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        useMachineContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useMachineContext))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCertificateExtensions(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509ChainElement,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCertificateExtensions", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlEntryExtensions(
        &mut self,
        entry: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Crl_X509CrlEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ProcessCrlEntryExtensions", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessCrlExtensions(
        &mut self,
        crl: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessCrlExtensions", (crl))?;
        Ok(__cordl_ret.into())
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
    pub fn SelectBestFromCollection(
        &mut self,
        child: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        c: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        > = __cordl_object.invoke("SelectBestFromCollection", (child, c))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn WrapUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WrapUp", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_CertificateAuthorities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        > = __cordl_object.invoke("get_CertificateAuthorities", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CertificateCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        > = __cordl_object.invoke("get_CertificateCollection", ())?;
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
    pub fn get_LMCAStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        > = __cordl_object.invoke("get_LMCAStore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LMRootStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        > = __cordl_object.invoke("get_LMRootStore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Roots(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2Collection,
        > = __cordl_object.invoke("get_Roots", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserCAStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        > = __cordl_object.invoke("get_UserCAStore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserRootStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Store,
        > = __cordl_object.invoke("get_UserRootStore", ())?;
        Ok(__cordl_ret.into())
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
