#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPrivateParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct MqvPrivateParameters {
    __cordl_parent: crate::System::Object,
    pub staticPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    pub ephemeralPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    pub ephemeralPublicKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPrivateParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::MqvPrivateParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."MqvPrivateParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPrivateParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::MqvPrivateParameters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPrivateParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::MqvPrivateParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPrivateParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::MqvPrivateParameters {
    pub fn New_ECPrivateKeyParameters_ECPrivateKeyParameters0(
        staticPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        ephemeralPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (staticPrivateKey, ephemeralPrivateKey))?;
        Ok(__cordl_object)
    }
    pub fn New_ECPublicKeyParameters1(
        staticPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        ephemeralPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        ephemeralPublicKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (staticPrivateKey, ephemeralPrivateKey, ephemeralPublicKey),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor_ECPrivateKeyParameters_ECPrivateKeyParameters0(
        &mut self,
        staticPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        ephemeralPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (staticPrivateKey, ephemeralPrivateKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ECPublicKeyParameters1(
        &mut self,
        staticPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        ephemeralPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        ephemeralPublicKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (staticPrivateKey, ephemeralPrivateKey, ephemeralPublicKey),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_EphemeralPrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters = __cordl_object
            .invoke("get_EphemeralPrivateKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EphemeralPublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters = __cordl_object
            .invoke("get_EphemeralPublicKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StaticPrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters = __cordl_object
            .invoke("get_StaticPrivateKey", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+MqvPrivateParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::MqvPrivateParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
