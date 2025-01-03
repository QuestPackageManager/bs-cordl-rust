#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
#[repr(C)]
#[derive(Debug)]
pub struct ECMqvBasicAgreement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub privParams: *mut crate::Org::BouncyCastle::Crypto::Parameters::MqvPrivateParameters,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement =>
    "Org.BouncyCastle.Crypto.Agreement"."ECMqvBasicAgreement"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement {
    pub fn CalculateAgreement(
        &mut self,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("CalculateAgreement", (pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateMqvAgreement(
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        >,
        d1U: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
        d2U: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
        >,
        Q2U: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
        Q1V: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
        Q2V: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateMqvAgreement", (parameters, d1U, d2U, Q2U, Q1V, Q2V))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetFieldSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IBasicAgreement>
for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IBasicAgreement {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvBasicAgreement")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IBasicAgreement>
for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IBasicAgreement {
        unsafe { std::mem::transmute(self) }
    }
}
