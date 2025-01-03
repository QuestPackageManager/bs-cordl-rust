#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaPrivateKeyParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct DsaPrivateKeyParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::DsaKeyParameters,
    pub x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaPrivateKeyParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::DsaPrivateKeyParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."DsaPrivateKeyParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaPrivateKeyParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::DsaPrivateKeyParameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::DsaKeyParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaPrivateKeyParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::DsaPrivateKeyParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaPrivateKeyParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::DsaPrivateKeyParameters {
    pub fn Equals_DsaPrivateKeyParameters1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DsaPrivateKeyParameters,
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
    pub fn New(
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DsaParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (x, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::DsaParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (x, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_X(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_X", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaPrivateKeyParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::DsaPrivateKeyParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
