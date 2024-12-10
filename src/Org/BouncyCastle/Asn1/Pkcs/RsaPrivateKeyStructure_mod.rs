#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+RsaPrivateKeyStructure")]
#[repr(C)]
#[derive(Debug)]
pub struct RsaPrivateKeyStructure {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub modulus: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub publicExponent: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub privateExponent: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub prime1: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub prime2: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub exponent1: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub exponent2: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub coefficient: *mut crate::Org::BouncyCastle::Math::BigInteger,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+RsaPrivateKeyStructure")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure =>
    "Org.BouncyCastle.Asn1.Pkcs"."RsaPrivateKeyStructure"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+RsaPrivateKeyStructure")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+RsaPrivateKeyStructure")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+RsaPrivateKeyStructure")]
impl crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure {
    pub fn New_Asn1Sequence1(
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object.into())
    }
    pub fn New_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger0(
        modulus: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        publicExponent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
        privateExponent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
        prime1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        prime2: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        exponent1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        exponent2: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        coefficient: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    modulus,
                    publicExponent,
                    privateExponent,
                    prime1,
                    prime2,
                    exponent1,
                    exponent2,
                    coefficient,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Object>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Asn1Object,
        > = __cordl_object.invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Asn1Sequence1(
        &mut self,
        seq: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Sequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger0(
        &mut self,
        modulus: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        publicExponent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
        privateExponent: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
        prime1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        prime2: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        exponent1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        exponent2: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        coefficient: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    modulus,
                    publicExponent,
                    privateExponent,
                    prime1,
                    prime2,
                    exponent1,
                    exponent2,
                    coefficient,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Coefficient(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Coefficient", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Exponent1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Exponent1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Exponent2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Exponent2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Modulus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Modulus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Prime1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Prime1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Prime2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Prime2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PrivateExponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_PrivateExponent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PublicExponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_PublicExponent", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+RsaPrivateKeyStructure")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
