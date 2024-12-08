#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPrivateKeyParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct ECPrivateKeyParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::ECKeyParameters,
    pub d: *mut crate::Org::BouncyCastle::Math::BigInteger,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPrivateKeyParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."ECPrivateKeyParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPrivateKeyParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::ECKeyParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPrivateKeyParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPrivateKeyParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters {
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn Equals_ECPrivateKeyParameters1(
        &mut self,
        other: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn get_D(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_D", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger_ECDomainParameters0(
        &mut self,
        d: *mut crate::Org::BouncyCastle::Math::BigInteger,
        parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (d, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger_DerObjectIdentifier1(
        &mut self,
        d: *mut crate::Org::BouncyCastle::Math::BigInteger,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (d, publicKeyParamSet))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_BigInteger_ECDomainParameters2(
        &mut self,
        algorithm: *mut crate::System::String,
        d: *mut crate::Org::BouncyCastle::Math::BigInteger,
        parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, d, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_BigInteger_DerObjectIdentifier3(
        &mut self,
        algorithm: *mut crate::System::String,
        d: *mut crate::Org::BouncyCastle::Math::BigInteger,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, d, publicKeyParamSet))?;
        Ok(__cordl_ret)
    }
    pub fn New_BigInteger_ECDomainParameters0(
        d: *mut crate::Org::BouncyCastle::Math::BigInteger,
        parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (d, parameters))?;
        Ok(__cordl_object)
    }
    pub fn New_BigInteger_DerObjectIdentifier1(
        d: *mut crate::Org::BouncyCastle::Math::BigInteger,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (d, publicKeyParamSet))?;
        Ok(__cordl_object)
    }
    pub fn New_String_BigInteger_ECDomainParameters2(
        algorithm: *mut crate::System::String,
        d: *mut crate::Org::BouncyCastle::Math::BigInteger,
        parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, d, parameters))?;
        Ok(__cordl_object)
    }
    pub fn New_String_BigInteger_DerObjectIdentifier3(
        algorithm: *mut crate::System::String,
        d: *mut crate::Org::BouncyCastle::Math::BigInteger,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, d, publicKeyParamSet))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPrivateKeyParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
