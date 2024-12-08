#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+Pbkdf2Params")]
#[repr(C)]
#[derive(Debug)]
pub struct Pbkdf2Params {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub octStr: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub iterationCount: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub keyLength: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub prf: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+Pbkdf2Params")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Pkcs::Pbkdf2Params =>
    "Org.BouncyCastle.Asn1.Pkcs"."Pbkdf2Params"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+Pbkdf2Params")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Pkcs::Pbkdf2Params {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+Pbkdf2Params")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Pkcs::Pbkdf2Params {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+Pbkdf2Params")]
impl crate::Org::BouncyCastle::Asn1::Pkcs::Pbkdf2Params {
    pub fn get_Prf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_Prf", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDefaultPrf(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDefaultPrf", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_KeyLength", ())?;
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
    pub fn GetSalt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetSalt", ())?;
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
    pub fn _ctor_Il2CppArray_i32_1(
        &mut self,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (salt, iterationCount))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_i32_i32_2(
        &mut self,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
        keyLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (salt, iterationCount, keyLength))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_i32_i32_AlgorithmIdentifier3(
        &mut self,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
        keyLength: i32,
        prf: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (salt, iterationCount, keyLength, prf))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_i32_AlgorithmIdentifier4(
        &mut self,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
        prf: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (salt, iterationCount, prf))?;
        Ok(__cordl_ret)
    }
    pub fn get_IterationCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_IterationCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_i32_1(
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (salt, iterationCount))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_i32_i32_2(
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
        keyLength: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (salt, iterationCount, keyLength))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_i32_i32_AlgorithmIdentifier3(
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
        keyLength: i32,
        prf: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (salt, iterationCount, keyLength, prf))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_i32_AlgorithmIdentifier4(
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
        prf: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (salt, iterationCount, prf))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+Pbkdf2Params")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Pkcs::Pbkdf2Params {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
