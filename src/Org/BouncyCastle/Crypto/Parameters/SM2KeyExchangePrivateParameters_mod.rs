#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+SM2KeyExchangePrivateParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct SM2KeyExchangePrivateParameters {
    __cordl_parent: crate::System::Object,
    pub mInitiator: bool,
    pub mStaticPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    pub mStaticPublicPoint: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    pub mEphemeralPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    pub mEphemeralPublicPoint: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+SM2KeyExchangePrivateParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::SM2KeyExchangePrivateParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."SM2KeyExchangePrivateParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+SM2KeyExchangePrivateParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::SM2KeyExchangePrivateParameters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+SM2KeyExchangePrivateParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::SM2KeyExchangePrivateParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+SM2KeyExchangePrivateParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::SM2KeyExchangePrivateParameters {
    pub fn New(
        initiator: bool,
        staticPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        ephemeralPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initiator, staticPrivateKey, ephemeralPrivateKey))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        initiator: bool,
        staticPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        ephemeralPrivateKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initiator, staticPrivateKey, ephemeralPrivateKey))?;
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
    pub fn get_EphemeralPublicPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("get_EphemeralPublicPoint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsInitiator(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInitiator", ())?;
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
    pub fn get_StaticPublicPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("get_StaticPublicPoint", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+SM2KeyExchangePrivateParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::SM2KeyExchangePrivateParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
