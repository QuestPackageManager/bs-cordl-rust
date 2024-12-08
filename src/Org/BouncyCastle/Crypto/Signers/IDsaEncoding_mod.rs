#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IDsaEncoding")]
#[repr(C)]
#[derive(Debug)]
pub struct IDsaEncoding {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IDsaEncoding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Signers::IDsaEncoding
    => "Org.BouncyCastle.Crypto.Signers"."IDsaEncoding"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IDsaEncoding")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Signers::IDsaEncoding {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IDsaEncoding")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Signers::IDsaEncoding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IDsaEncoding")]
impl crate::Org::BouncyCastle::Crypto::Signers::IDsaEncoding {
    pub fn Decode(
        &mut self,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        encoding: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::BigInteger,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("Decode", (n, encoding))?;
        Ok(__cordl_ret)
    }
    pub fn Encode(
        &mut self,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        r: *mut crate::Org::BouncyCastle::Math::BigInteger,
        s: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Encode", (n, r, s))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+IDsaEncoding")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Signers::IDsaEncoding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
