#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECPublicKeyParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct ECPublicKeyParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::ECKeyParameters,
    pub q: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
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
        other: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPublicKeyParameters,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
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
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_ECPoint_DerObjectIdentifier1(
        q: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (q, publicKeyParamSet))?;
        Ok(__cordl_object)
    }
    pub fn New_ECPoint_ECDomainParameters0(
        q: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (q, parameters))?;
        Ok(__cordl_object)
    }
    pub fn New_String_ECPoint_DerObjectIdentifier3(
        algorithm: *mut crate::System::String,
        q: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, q, publicKeyParamSet))?;
        Ok(__cordl_object)
    }
    pub fn New_String_ECPoint_ECDomainParameters2(
        algorithm: *mut crate::System::String,
        q: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, q, parameters))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_ECPoint_DerObjectIdentifier1(
        &mut self,
        q: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (q, publicKeyParamSet))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ECPoint_ECDomainParameters0(
        &mut self,
        q: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (q, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_ECPoint_DerObjectIdentifier3(
        &mut self,
        algorithm: *mut crate::System::String,
        q: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, q, publicKeyParamSet))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_ECPoint_ECDomainParameters2(
        &mut self,
        algorithm: *mut crate::System::String,
        q: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, q, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn get_Q(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("get_Q", ())?;
        Ok(__cordl_ret)
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