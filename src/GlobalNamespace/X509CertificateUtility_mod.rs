#[cfg(feature = "X509CertificateUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertificateUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "X509CertificateUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::X509CertificateUtility => ""
    ."X509CertificateUtility"
);
#[cfg(feature = "X509CertificateUtility")]
impl std::ops::Deref for crate::GlobalNamespace::X509CertificateUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "X509CertificateUtility")]
impl std::ops::DerefMut for crate::GlobalNamespace::X509CertificateUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "X509CertificateUtility")]
impl crate::GlobalNamespace::X509CertificateUtility {
    #[cfg(feature = "X509CertificateUtility+PasswordFinder")]
    pub type PasswordFinder = crate::GlobalNamespace::X509CertificateUtility_PasswordFinder;
    #[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
    pub type RSACertificateEncryptionProvider = crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider;
    pub fn GetCertificateEncryptionProvider(
        privateKeyPem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ICertificateEncryptionProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ICertificateEncryptionProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCertificateEncryptionProvider", (privateKeyPem, password))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCertificateList(
        certificatePem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        certificateChainPem: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCertificateList", (certificatePem, certificateChainPem))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRSACertificateEncryptionProvider(
        privateKeyPem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        passwordFinder: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::X509CertificateUtility_PasswordFinder,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetRSACertificateEncryptionProvider",
                (privateKeyPem, passwordFinder),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootCertificates() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRootCertificates", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRootCertificatesRaw() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRootCertificatesRaw", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateCertificateChain(
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        certificateChain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<u8>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateCertificateChain", (certificate, certificateChain))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateCertificateChainDotNet(
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        certificateChain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<u8>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateCertificateChainDotNet", (certificate, certificateChain))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateCertificateChainUnity(
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        certificateChain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<u8>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateCertificateChainUnity", (certificate, certificateChain))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "X509CertificateUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::X509CertificateUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "X509CertificateUtility+PasswordFinder")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertificateUtility_PasswordFinder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
}
#[cfg(feature = "X509CertificateUtility+PasswordFinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::X509CertificateUtility_PasswordFinder => ""
    ."X509CertificateUtility/PasswordFinder"
);
#[cfg(feature = "X509CertificateUtility+PasswordFinder")]
impl std::ops::Deref for crate::GlobalNamespace::X509CertificateUtility_PasswordFinder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "X509CertificateUtility+PasswordFinder")]
impl std::ops::DerefMut
for crate::GlobalNamespace::X509CertificateUtility_PasswordFinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "X509CertificateUtility+PasswordFinder")]
impl crate::GlobalNamespace::X509CertificateUtility_PasswordFinder {
    pub fn GetPassword(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = __cordl_object.invoke("GetPassword", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray0(
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (password))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString1(
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (password))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (password))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (password))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "X509CertificateUtility+PasswordFinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::X509CertificateUtility_PasswordFinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "X509CertificateUtility+PasswordFinder")]
impl AsRef<crate::Org::BouncyCastle::OpenSsl::IPasswordFinder>
for crate::GlobalNamespace::X509CertificateUtility_PasswordFinder {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::OpenSsl::IPasswordFinder {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "X509CertificateUtility+PasswordFinder")]
impl AsMut<crate::Org::BouncyCastle::OpenSsl::IPasswordFinder>
for crate::GlobalNamespace::X509CertificateUtility_PasswordFinder {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::OpenSsl::IPasswordFinder {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertificateUtility_RSACertificateEncryptionProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _signer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ISigner>,
}
#[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider => ""
    ."X509CertificateUtility/RSACertificateEncryptionProvider"
);
#[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
impl std::ops::Deref
for crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
impl std::ops::DerefMut
for crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
impl crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKey))?;
        Ok(__cordl_object.into())
    }
    pub fn SignData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("SignData", (data, offset, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        privateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKey))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
impl AsRef<crate::GlobalNamespace::ICertificateEncryptionProvider>
for crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider {
    fn as_ref(&self) -> &crate::GlobalNamespace::ICertificateEncryptionProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
impl AsMut<crate::GlobalNamespace::ICertificateEncryptionProvider>
for crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ICertificateEncryptionProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
