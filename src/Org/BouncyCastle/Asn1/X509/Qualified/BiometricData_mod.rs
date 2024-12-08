#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+BiometricData")]
#[repr(C)]
#[derive(Debug)]
pub struct BiometricData {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub typeOfBiometricData: *mut crate::Org::BouncyCastle::Asn1::X509::Qualified::TypeOfBiometricData,
    pub hashAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub biometricDataHash: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub sourceDataUri: *mut crate::Org::BouncyCastle::Asn1::DerIA5String,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+BiometricData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::Qualified::BiometricData =>
    "Org.BouncyCastle.Asn1.X509.Qualified"."BiometricData"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+BiometricData")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::Qualified::BiometricData {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+BiometricData")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::X509::Qualified::BiometricData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+BiometricData")]
impl crate::Org::BouncyCastle::Asn1::X509::Qualified::BiometricData {
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_TypeOfBiometricData_AlgorithmIdentifier_Asn1OctetString2(
        typeOfBiometricData: *mut crate::Org::BouncyCastle::Asn1::X509::Qualified::TypeOfBiometricData,
        hashAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        biometricDataHash: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (typeOfBiometricData, hashAlgorithm, biometricDataHash),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_TypeOfBiometricData_AlgorithmIdentifier_Asn1OctetString_DerIA5String1(
        typeOfBiometricData: *mut crate::Org::BouncyCastle::Asn1::X509::Qualified::TypeOfBiometricData,
        hashAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        biometricDataHash: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        sourceDataUri: *mut crate::Org::BouncyCastle::Asn1::DerIA5String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (typeOfBiometricData, hashAlgorithm, biometricDataHash, sourceDataUri),
            )?;
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
    pub fn _ctor_TypeOfBiometricData_AlgorithmIdentifier_Asn1OctetString2(
        &mut self,
        typeOfBiometricData: *mut crate::Org::BouncyCastle::Asn1::X509::Qualified::TypeOfBiometricData,
        hashAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        biometricDataHash: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeOfBiometricData, hashAlgorithm, biometricDataHash))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TypeOfBiometricData_AlgorithmIdentifier_Asn1OctetString_DerIA5String1(
        &mut self,
        typeOfBiometricData: *mut crate::Org::BouncyCastle::Asn1::X509::Qualified::TypeOfBiometricData,
        hashAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        biometricDataHash: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        sourceDataUri: *mut crate::Org::BouncyCastle::Asn1::DerIA5String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (typeOfBiometricData, hashAlgorithm, biometricDataHash, sourceDataUri),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_BiometricDataHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_BiometricDataHash", ())?;
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
    pub fn get_SourceDataUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerIA5String,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerIA5String = __cordl_object
            .invoke("get_SourceDataUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeOfBiometricData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::Qualified::TypeOfBiometricData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::Qualified::TypeOfBiometricData = __cordl_object
            .invoke("get_TypeOfBiometricData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+Qualified+BiometricData")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::Qualified::BiometricData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
