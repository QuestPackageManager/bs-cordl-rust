#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1KeyWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1KeyWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub algorithm: *mut quest_hook::libil2cpp::Il2CppString,
    pub wrapper: *mut crate::Org::BouncyCastle::Crypto::IKeyWrapper,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1KeyWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Operators::Asn1KeyWrapper =>
    "Org.BouncyCastle.Crypto.Operators"."Asn1KeyWrapper"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1KeyWrapper")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Operators::Asn1KeyWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1KeyWrapper")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Operators::Asn1KeyWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1KeyWrapper")]
impl crate::Org::BouncyCastle::Crypto::Operators::Asn1KeyWrapper {
    pub fn New(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, cert))?;
        Ok(__cordl_object.into())
    }
    pub fn Wrap(
        &mut self,
        keyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBlockResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBlockResult,
        > = __cordl_object.invoke("Wrap", (keyData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AlgorithmDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_AlgorithmDetails", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1KeyWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::Asn1KeyWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1KeyWrapper")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IKeyWrapper>
for crate::Org::BouncyCastle::Crypto::Operators::Asn1KeyWrapper {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IKeyWrapper {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1KeyWrapper")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IKeyWrapper>
for crate::Org::BouncyCastle::Crypto::Operators::Asn1KeyWrapper {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IKeyWrapper {
        unsafe { std::mem::transmute(self) }
    }
}
