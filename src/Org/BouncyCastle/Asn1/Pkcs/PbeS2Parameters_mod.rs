#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PbeS2Parameters")]
#[repr(C)]
#[derive(Debug)]
pub struct PbeS2Parameters {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub func: *mut crate::Org::BouncyCastle::Asn1::Pkcs::KeyDerivationFunc,
    pub scheme: *mut crate::Org::BouncyCastle::Asn1::Pkcs::EncryptionScheme,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PbeS2Parameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Pkcs::PbeS2Parameters
    => "Org.BouncyCastle.Asn1.Pkcs"."PbeS2Parameters"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PbeS2Parameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Pkcs::PbeS2Parameters {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PbeS2Parameters")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Pkcs::PbeS2Parameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PbeS2Parameters")]
impl crate::Org::BouncyCastle::Asn1::Pkcs::PbeS2Parameters {
    pub fn New_Asn1Sequence1(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_KeyDerivationFunc_EncryptionScheme0(
        keyDevFunc: *mut crate::Org::BouncyCastle::Asn1::Pkcs::KeyDerivationFunc,
        encScheme: *mut crate::Org::BouncyCastle::Asn1::Pkcs::EncryptionScheme,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyDevFunc, encScheme))?;
        Ok(__cordl_object)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence1(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_KeyDerivationFunc_EncryptionScheme0(
        &mut self,
        keyDevFunc: *mut crate::Org::BouncyCastle::Asn1::Pkcs::KeyDerivationFunc,
        encScheme: *mut crate::Org::BouncyCastle::Asn1::Pkcs::EncryptionScheme,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyDevFunc, encScheme))?;
        Ok(__cordl_ret)
    }
    pub fn get_EncryptionScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Pkcs::EncryptionScheme,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Pkcs::EncryptionScheme = __cordl_object
            .invoke("get_EncryptionScheme", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyDerivationFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Pkcs::KeyDerivationFunc,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Pkcs::KeyDerivationFunc = __cordl_object
            .invoke("get_KeyDerivationFunc", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PbeS2Parameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Pkcs::PbeS2Parameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
