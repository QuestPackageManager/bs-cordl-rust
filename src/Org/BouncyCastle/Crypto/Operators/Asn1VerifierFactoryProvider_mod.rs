#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct Asn1VerifierFactoryProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub publicKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider =>
    "Org.BouncyCastle.Crypto.Operators"."Asn1VerifierFactoryProvider"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    pub fn CreateVerifierFactory(
        &mut self,
        algorithmDetails: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IVerifierFactory>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IVerifierFactory,
        > = __cordl_object.invoke("CreateVerifierFactory", (algorithmDetails))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (publicKey))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (publicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignatureAlgNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerable,
        > = __cordl_object.invoke("get_SignatureAlgNames", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IVerifierFactoryProvider>
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IVerifierFactoryProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+Asn1VerifierFactoryProvider")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IVerifierFactoryProvider>
for crate::Org::BouncyCastle::Crypto::Operators::Asn1VerifierFactoryProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::IVerifierFactoryProvider {
        unsafe { std::mem::transmute(self) }
    }
}
