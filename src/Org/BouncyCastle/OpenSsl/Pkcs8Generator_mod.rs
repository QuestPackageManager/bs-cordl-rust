#[cfg(feature = "Org+BouncyCastle+OpenSsl+Pkcs8Generator")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs8Generator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub iterationCount: i32,
    pub privKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    >,
    pub random: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Security::SecureRandom,
    >,
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+Pkcs8Generator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::OpenSsl::Pkcs8Generator =>
    "Org.BouncyCastle.OpenSsl"."Pkcs8Generator"
);
#[cfg(feature = "Org+BouncyCastle+OpenSsl+Pkcs8Generator")]
impl std::ops::Deref for crate::Org::BouncyCastle::OpenSsl::Pkcs8Generator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IO::Pem::PemObject,
        > = __cordl_object.invoke("Generate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_AsymmetricKeyParameter0(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privKey))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString1(
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privKey, algorithm))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_AsymmetricKeyParameter0(
        &mut self,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        privKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privKey, algorithm))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_Password(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Password", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SecureRandom(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SecureRandom", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+OpenSsl+Pkcs8Generator")]
impl AsRef<crate::Org::BouncyCastle::Utilities::IO::Pem::PemObjectGenerator>
for crate::Org::BouncyCastle::OpenSsl::Pkcs8Generator {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Utilities::IO::Pem::PemObjectGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+OpenSsl+Pkcs8Generator")]
impl AsMut<crate::Org::BouncyCastle::Utilities::IO::Pem::PemObjectGenerator>
for crate::Org::BouncyCastle::OpenSsl::Pkcs8Generator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Utilities::IO::Pem::PemObjectGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
