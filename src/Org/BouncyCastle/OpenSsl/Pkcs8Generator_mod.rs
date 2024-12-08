#[cfg(feature = "Org+BouncyCastle+OpenSsl+Pkcs8Generator")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs8Generator {
    __cordl_parent: crate::System::Object,
    pub password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub algorithm: *mut crate::System::String,
    pub iterationCount: i32,
    pub privKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+Pkcs8Generator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::OpenSsl::Pkcs8Generator =>
    "Org.BouncyCastle.OpenSsl"."Pkcs8Generator"
);
#[cfg(feature = "Org+BouncyCastle+OpenSsl+Pkcs8Generator")]
impl std::ops::Deref for crate::Org::BouncyCastle::OpenSsl::Pkcs8Generator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+Pkcs8Generator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::OpenSsl::Pkcs8Generator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+Pkcs8Generator")]
impl crate::Org::BouncyCastle::OpenSsl::Pkcs8Generator {
    pub fn Generate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject = __cordl_object
            .invoke("Generate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_AsymmetricKeyParameter0(
        privKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privKey))?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        privKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        algorithm: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privKey, algorithm))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_AsymmetricKeyParameter0(
        &mut self,
        privKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        privKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        algorithm: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privKey, algorithm))?;
        Ok(__cordl_ret)
    }
    pub fn set_IterationCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IterationCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Password(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Password", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SecureRandom(
        &mut self,
        value: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SecureRandom", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+Pkcs8Generator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::OpenSsl::Pkcs8Generator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
