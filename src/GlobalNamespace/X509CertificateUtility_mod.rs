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
    #[cfg(feature = "X509CertificateUtility+_GetCertificateList_d__9")]
    pub type _GetCertificateList_d__9 = crate::GlobalNamespace::X509CertificateUtility__GetCertificateList_d__9;
    #[cfg(feature = "X509CertificateUtility+__c")]
    pub type __c = crate::GlobalNamespace::X509CertificateUtility___c;
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
    pub _password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
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
    pub _signer: *mut crate::Org::BouncyCastle::Crypto::ISigner,
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
