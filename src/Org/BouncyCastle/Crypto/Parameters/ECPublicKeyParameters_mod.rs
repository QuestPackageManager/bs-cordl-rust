#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPublicKeyParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct ECPublicKeyParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::ECKeyParameters,
    pub q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPublicKeyParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."ECPublicKeyParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPublicKeyParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::ECKeyParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPublicKeyParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPublicKeyParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters {
    pub fn Equals_ECPublicKeyParameters1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_ECPoint_DerObjectIdentifier1(
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        publicKeyParamSet: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (q, publicKeyParamSet))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ECPoint_ECDomainParameters0(
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (q, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_ECPoint_DerObjectIdentifier3(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        publicKeyParamSet: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, q, publicKeyParamSet))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_ECPoint_ECDomainParameters2(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, q, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_ECPoint_DerObjectIdentifier1(
        &mut self,
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        publicKeyParamSet: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (q, publicKeyParamSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ECPoint_ECDomainParameters0(
        &mut self,
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (q, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_ECPoint_DerObjectIdentifier3(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        publicKeyParamSet: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, q, publicKeyParamSet))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_ECPoint_ECDomainParameters2(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, q, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Q(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = __cordl_object.invoke("get_Q", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPublicKeyParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
