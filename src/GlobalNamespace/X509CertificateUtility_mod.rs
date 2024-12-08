#[cfg(feature = "X509CertificateUtility+PasswordFinder")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertificateUtility_PasswordFinder {
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<char> = __cordl_object
            .invoke("GetPassword", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (password))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (password))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray0(
        password: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (password))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        password: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (password))?;
        Ok(__cordl_object)
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
#[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertificateUtility_RSACertificateEncryptionProvider {
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKey))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn SignData(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("SignData", (data, offset, length))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        privateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKey))?;
        Ok(__cordl_object)
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
#[cfg(feature = "X509CertificateUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct X509CertificateUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "X509CertificateUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for X509CertificateUtility => ""."X509CertificateUtility"
);
#[cfg(feature = "X509CertificateUtility")]
impl std::ops::Deref for X509CertificateUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "X509CertificateUtility")]
impl std::ops::DerefMut for X509CertificateUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "X509CertificateUtility")]
impl X509CertificateUtility {
    #[cfg(feature = "X509CertificateUtility+PasswordFinder")]
    pub type PasswordFinder = crate::GlobalNamespace::X509CertificateUtility_PasswordFinder;
    #[cfg(feature = "X509CertificateUtility+_GetCertificateList_d__9")]
    pub type _GetCertificateList_d__9 = crate::GlobalNamespace::X509CertificateUtility__GetCertificateList_d__9;
    #[cfg(feature = "X509CertificateUtility+__c")]
    pub type __c = crate::GlobalNamespace::X509CertificateUtility___c;
    #[cfg(feature = "X509CertificateUtility+RSACertificateEncryptionProvider")]
    pub type RSACertificateEncryptionProvider = crate::GlobalNamespace::X509CertificateUtility_RSACertificateEncryptionProvider;
}
#[cfg(feature = "X509CertificateUtility")]
impl quest_hook::libil2cpp::ObjectType for X509CertificateUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
