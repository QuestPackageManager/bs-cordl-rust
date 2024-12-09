#[cfg(feature = "Org+BouncyCastle+Cms+Pkcs5Scheme2Utf8PbeKey")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs5Scheme2Utf8PbeKey {
    __cordl_parent: crate::Org::BouncyCastle::Cms::CmsPbeKey,
}
#[cfg(feature = "Org+BouncyCastle+Cms+Pkcs5Scheme2Utf8PbeKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::Pkcs5Scheme2Utf8PbeKey
    => "Org.BouncyCastle.Cms"."Pkcs5Scheme2Utf8PbeKey"
);
#[cfg(feature = "Org+BouncyCastle+Cms+Pkcs5Scheme2Utf8PbeKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::Pkcs5Scheme2Utf8PbeKey {
    type Target = crate::Org::BouncyCastle::Cms::CmsPbeKey;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+Pkcs5Scheme2Utf8PbeKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::Pkcs5Scheme2Utf8PbeKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+Pkcs5Scheme2Utf8PbeKey")]
impl crate::Org::BouncyCastle::Cms::Pkcs5Scheme2Utf8PbeKey {
    pub fn GetEncoded(
        &mut self,
        algorithmOid: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter = __cordl_object
            .invoke("GetEncoded", (algorithmOid))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray_AlgorithmIdentifier3(
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        keyDerivationAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (password, keyDerivationAlgorithm))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_Il2CppArray_i32_2(
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (password, salt, iterationCount))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_AlgorithmIdentifier1(
        password: *mut quest_hook::libil2cpp::Il2CppString,
        keyDerivationAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (password, keyDerivationAlgorithm))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_Il2CppArray_i32_0(
        password: *mut quest_hook::libil2cpp::Il2CppString,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (password, salt, iterationCount))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppArray_AlgorithmIdentifier3(
        &mut self,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        keyDerivationAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (password, keyDerivationAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_Il2CppArray_i32_2(
        &mut self,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (password, salt, iterationCount))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_AlgorithmIdentifier1(
        &mut self,
        password: *mut quest_hook::libil2cpp::Il2CppString,
        keyDerivationAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (password, keyDerivationAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_Il2CppArray_i32_0(
        &mut self,
        password: *mut quest_hook::libil2cpp::Il2CppString,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iterationCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (password, salt, iterationCount))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+Pkcs5Scheme2Utf8PbeKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::Pkcs5Scheme2Utf8PbeKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
