#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedValue")]
#[repr(C)]
#[derive(Debug)]
pub struct EncryptedValue {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub intendedAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub symmAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub encSymmKey: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    pub keyAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub valueHint: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub encValue: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedValue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Crmf::EncryptedValue =>
    "Org.BouncyCastle.Asn1.Crmf"."EncryptedValue"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedValue")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedValue")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedValue")]
impl crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue {
    pub fn New_AlgorithmIdentifier_AlgorithmIdentifier_DerBitString_AlgorithmIdentifier_Asn1OctetString_DerBitString1(
        intendedAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        symmAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        encSymmKey: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
        keyAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        valueHint: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        encValue: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (intendedAlg, symmAlg, encSymmKey, keyAlg, valueHint, encValue),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
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
    pub fn _ctor_AlgorithmIdentifier_AlgorithmIdentifier_DerBitString_AlgorithmIdentifier_Asn1OctetString_DerBitString1(
        &mut self,
        intendedAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        symmAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        encSymmKey: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
        keyAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        valueHint: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        encValue: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (intendedAlg, symmAlg, encSymmKey, keyAlg, valueHint, encValue),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence0(
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
    pub fn get_EncSymmKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerBitString = __cordl_object
            .invoke("get_EncSymmKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerBitString = __cordl_object
            .invoke("get_EncValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IntendedAlg(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_IntendedAlg", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyAlg(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_KeyAlg", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SymmAlg(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_SymmAlg", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ValueHint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_ValueHint", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+EncryptedValue")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
