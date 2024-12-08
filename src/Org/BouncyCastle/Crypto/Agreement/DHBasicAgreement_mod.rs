#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+DHBasicAgreement")]
#[repr(C)]
#[derive(Debug)]
pub struct DHBasicAgreement {
    __cordl_parent: crate::System::Object,
    pub key: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHPrivateKeyParameters,
    pub dhParams: *mut crate::Org::BouncyCastle::Crypto::Parameters::DHParameters,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+DHBasicAgreement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement =>
    "Org.BouncyCastle.Crypto.Agreement"."DHBasicAgreement"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+DHBasicAgreement")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Agreement::DHBasicAgreement {
    type Target = crate::System::Object;
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
        pubKey: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("CalculateAgreement", (pubKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetFieldSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetFieldSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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