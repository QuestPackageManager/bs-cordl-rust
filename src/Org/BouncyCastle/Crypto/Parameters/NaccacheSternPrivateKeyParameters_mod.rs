#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct NaccacheSternPrivateKeyParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyParameters,
    pub phiN: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub smallPrimes: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternPrivateKeyParameters")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternPrivateKeyParameters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Parameters";
    const CLASS_NAME: &'static str = "NaccacheSternPrivateKeyParameters";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn New_ArrayList0(
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lowerSigmaBound: i32,
        smallPrimes: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        phiN: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (g, n, lowerSigmaBound, smallPrimes, phiN))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IList1(
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lowerSigmaBound: i32,
        smallPrimes: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        phiN: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (g, n, lowerSigmaBound, smallPrimes, phiN))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_ArrayList0(
        &mut self,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lowerSigmaBound: i32,
        smallPrimes: quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
        phiN: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (g, n, lowerSigmaBound, smallPrimes, phiN))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IList1(
        &mut self,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        lowerSigmaBound: i32,
        smallPrimes: quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
        phiN: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (g, n, lowerSigmaBound, smallPrimes, phiN))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PhiN(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_PhiN", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SmallPrimes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::ArrayList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ArrayList,
        > = __cordl_object.invoke("get_SmallPrimes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SmallPrimesList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = __cordl_object
            .invoke("get_SmallPrimesList", ())?;
        Ok(__cordl_ret.into())
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
