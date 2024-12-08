#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct NaccacheSternPrivateKeyParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyParameters,
    pub phiN: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub smallPrimes: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::NaccacheSternPrivateKeyParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."NaccacheSternPrivateKeyParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternPrivateKeyParameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternPrivateKeyParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternPrivateKeyParameters {
    pub fn get_PhiN(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_PhiN", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SmallPrimes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ArrayList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ArrayList = __cordl_object
            .invoke("get_SmallPrimes", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ArrayList0(
        &mut self,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        lowerSigmaBound: i32,
        smallPrimes: *mut crate::System::Collections::ArrayList,
        phiN: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (g, n, lowerSigmaBound, smallPrimes, phiN))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IList1(
        &mut self,
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        lowerSigmaBound: i32,
        smallPrimes: *mut crate::System::Collections::IList,
        phiN: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (g, n, lowerSigmaBound, smallPrimes, phiN))?;
        Ok(__cordl_ret)
    }
    pub fn get_SmallPrimesList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("get_SmallPrimesList", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_ArrayList0(
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        lowerSigmaBound: i32,
        smallPrimes: *mut crate::System::Collections::ArrayList,
        phiN: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (g, n, lowerSigmaBound, smallPrimes, phiN))?;
        Ok(__cordl_object)
    }
    pub fn New_IList1(
        g: *mut crate::Org::BouncyCastle::Math::BigInteger,
        n: *mut crate::Org::BouncyCastle::Math::BigInteger,
        lowerSigmaBound: i32,
        smallPrimes: *mut crate::System::Collections::IList,
        phiN: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (g, n, lowerSigmaBound, smallPrimes, phiN))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternPrivateKeyParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
