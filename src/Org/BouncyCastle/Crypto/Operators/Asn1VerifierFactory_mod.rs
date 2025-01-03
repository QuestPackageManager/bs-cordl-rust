#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1VerifierFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub algID: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    >,
    pub publicKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactory =>
    "Org.BouncyCastle.Crypto.Operators"."Asn1VerifierFactory"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactory")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactory")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactory")]
impl crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactory {
    pub fn CreateCalculator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IStreamCalculator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IStreamCalculator,
        > = __cordl_object.invoke("CreateCalculator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_AlgorithmIdentifier1(
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, publicKey))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString0(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, publicKey))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_AlgorithmIdentifier1(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, publicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, publicKey))?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactory")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IVerifierFactory>
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactory {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IVerifierFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactory")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IVerifierFactory>
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactory {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IVerifierFactory {
        unsafe { std::mem::transmute(self) }
    }
}
