#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct RsaPrivateCrtKeyParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters,
    pub e: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub p: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub q: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub dP: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub dQ: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub qInv: *mut crate::Org::BouncyCastle::Math::BigInteger,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."RsaPrivateCrtKeyParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters {
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger0(
        modulus: *mut crate::Org::BouncyCastle::Math::BigInteger,
        publicExponent: *mut crate::Org::BouncyCastle::Math::BigInteger,
        privateExponent: *mut crate::Org::BouncyCastle::Math::BigInteger,
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        dP: *mut crate::Org::BouncyCastle::Math::BigInteger,
        dQ: *mut crate::Org::BouncyCastle::Math::BigInteger,
        qInv: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (modulus, publicExponent, privateExponent, p, q, dP, dQ, qInv),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_RsaPrivateKeyStructure1(
        rsaPrivateKey: *mut crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rsaPrivateKey))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger_BigInteger0(
        &mut self,
        modulus: *mut crate::Org::BouncyCastle::Math::BigInteger,
        publicExponent: *mut crate::Org::BouncyCastle::Math::BigInteger,
        privateExponent: *mut crate::Org::BouncyCastle::Math::BigInteger,
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        dP: *mut crate::Org::BouncyCastle::Math::BigInteger,
        dQ: *mut crate::Org::BouncyCastle::Math::BigInteger,
        qInv: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (modulus, publicExponent, privateExponent, p, q, dP, dQ, qInv),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_RsaPrivateKeyStructure1(
        &mut self,
        rsaPrivateKey: *mut crate::Org::BouncyCastle::Asn1::Pkcs::RsaPrivateKeyStructure,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rsaPrivateKey))?;
        Ok(__cordl_ret)
    }
    pub fn get_DP(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_DP", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DQ(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_DQ", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_P(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_P", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PublicExponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_PublicExponent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Q(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_Q", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_QInv(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_QInv", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+RsaPrivateCrtKeyParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::RsaPrivateCrtKeyParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
