#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct NaccacheSternKeyParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    pub g: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub n: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub lowerSigmaBound: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."NaccacheSternKeyParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyParameters {
    type Target = crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyParameters {
    pub fn _ctor(
        &mut self,
        privateKey: bool,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        lowerSigmaBound: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKey, g, n, lowerSigmaBound))?;
        Ok(__cordl_ret)
    }
    pub fn get_Modulus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_Modulus", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LowerSigmaBound(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LowerSigmaBound", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_G(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_G", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        privateKey: bool,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        lowerSigmaBound: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKey, g, n, lowerSigmaBound))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
