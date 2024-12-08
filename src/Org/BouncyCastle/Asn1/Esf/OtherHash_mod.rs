#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+OtherHash")]
#[repr(C)]
#[derive(Debug)]
pub struct OtherHash {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub sha1Hash: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub otherHash: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+OtherHash")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Esf::OtherHash =>
    "Org.BouncyCastle.Asn1.Esf"."OtherHash"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+OtherHash")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Esf::OtherHash {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+OtherHash")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Esf::OtherHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+OtherHash")]
impl crate::Org::BouncyCastle::Asn1::Esf::OtherHash {
    pub fn _ctor_Il2CppArray0(
        &mut self,
        sha1Hash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sha1Hash))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1OctetString1(
        &mut self,
        sha1Hash: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sha1Hash))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_OtherHashAlgAndValue2(
        &mut self,
        otherHash: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (otherHash))?;
        Ok(__cordl_ret)
    }
    pub fn get_HashAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_HashAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetHashValue", ())?;
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
    pub fn New_Il2CppArray0(
        sha1Hash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sha1Hash))?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1OctetString1(
        sha1Hash: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sha1Hash))?;
        Ok(__cordl_object)
    }
    pub fn New_OtherHashAlgAndValue2(
        otherHash: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (otherHash))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+OtherHash")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Esf::OtherHash {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
