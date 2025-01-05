#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+DHBasicAgreement")]
#[repr(C)]
#[derive(Debug)]
pub struct DHBasicAgreement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub key: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::DHPrivateKeyParameters,
    >,
    pub dhParams: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+DHBasicAgreement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement =>
    "Org.BouncyCastle.Crypto.Agreement"."DHBasicAgreement"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+DHBasicAgreement")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+DHBasicAgreement")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+DHBasicAgreement")]
impl crate::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement {
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+DHBasicAgreement")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+DHBasicAgreement")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IBasicAgreement>
for crate::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IBasicAgreement {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+DHBasicAgreement")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IBasicAgreement>
for crate::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IBasicAgreement {
        unsafe { std::mem::transmute(self) }
    }
}
