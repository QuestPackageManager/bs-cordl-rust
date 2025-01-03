#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+X448Agreement")]
#[repr(C)]
#[derive(Debug)]
pub struct X448Agreement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub privateKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::X448PrivateKeyParameters,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+X448Agreement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::X448Agreement =>
    "Org.BouncyCastle.Crypto.Agreement"."X448Agreement"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+X448Agreement")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Agreement::X448Agreement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+X448Agreement")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Agreement::X448Agreement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+X448Agreement")]
impl crate::Org::BouncyCastle::Crypto::Agreement::X448Agreement {
    pub fn CalculateAgreement(
        &mut self,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CalculateAgreement", (publicKey, buf, off))?;
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
    pub fn get_AgreementSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_AgreementSize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+X448Agreement")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::X448Agreement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+X448Agreement")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IRawAgreement>
for crate::Org::BouncyCastle::Crypto::Agreement::X448Agreement {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IRawAgreement {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+X448Agreement")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IRawAgreement>
for crate::Org::BouncyCastle::Crypto::Agreement::X448Agreement {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IRawAgreement {
        unsafe { std::mem::transmute(self) }
    }
}
