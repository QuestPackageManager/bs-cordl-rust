#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+SM2KeyExchangePrivateParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct SM2KeyExchangePrivateParameters {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub mInitiator: bool,
    pub mStaticPrivateKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    >,
    pub mStaticPublicPoint: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::ECPoint,
    >,
    pub mEphemeralPrivateKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    >,
    pub mEphemeralPublicPoint: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::ECPoint,
    >,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
        staticPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
        ephemeralPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initiator, staticPrivateKey, ephemeralPrivateKey))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        initiator: bool,
        staticPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
        ephemeralPrivateKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initiator, staticPrivateKey, ephemeralPrivateKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EphemeralPrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        > = __cordl_object.invoke("get_EphemeralPrivateKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_EphemeralPublicPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = __cordl_object.invoke("get_EphemeralPublicPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsInitiator(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInitiator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StaticPrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        > = __cordl_object.invoke("get_StaticPrivateKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StaticPublicPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = __cordl_object.invoke("get_StaticPublicPoint", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+SM2KeyExchangePrivateParameters")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
> for crate::Org::BouncyCastle::Crypto::Parameters::SM2KeyExchangePrivateParameters {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::ICipherParameters,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+SM2KeyExchangePrivateParameters")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::ICipherParameters>,
> for crate::Org::BouncyCastle::Crypto::Parameters::SM2KeyExchangePrivateParameters {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::ICipherParameters,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
