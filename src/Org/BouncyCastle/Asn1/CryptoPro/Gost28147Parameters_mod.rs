#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+Gost28147Parameters")]
#[repr(C)]
#[derive(Debug)]
pub struct Gost28147Parameters {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub iv: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub paramSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+Gost28147Parameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::CryptoPro::Gost28147Parameters =>
    "Org.BouncyCastle.Asn1.CryptoPro"."Gost28147Parameters"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+Gost28147Parameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::CryptoPro::Gost28147Parameters {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+Gost28147Parameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::CryptoPro::Gost28147Parameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+Gost28147Parameters")]
impl crate::Org::BouncyCastle::Asn1::CryptoPro::Gost28147Parameters {
    pub fn _ctor(
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
    pub fn New(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+CryptoPro+Gost28147Parameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::CryptoPro::Gost28147Parameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
