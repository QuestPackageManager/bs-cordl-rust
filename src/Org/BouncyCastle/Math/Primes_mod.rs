#[cfg(feature = "Org+BouncyCastle+Math+Primes+MROutput")]
#[repr(C)]
#[derive(Debug)]
pub struct Primes_MROutput {
    __cordl_parent: crate::System::Object,
    pub mProvablyComposite: bool,
    pub mFactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes+MROutput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::Primes_MROutput =>
    "Org.BouncyCastle.Math"."Primes/MROutput"
);
#[cfg(feature = "Org+BouncyCastle+Math+Primes+MROutput")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::Primes_MROutput {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes+MROutput")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::Primes_MROutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes+MROutput")]
impl crate::Org::BouncyCastle::Math::Primes_MROutput {
    pub fn New(
        provablyComposite: bool,
        factor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (provablyComposite, factor))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        provablyComposite: bool,
        factor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (provablyComposite, factor))?;
        Ok(__cordl_ret)
    }
    pub fn get_Factor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_Factor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNotPrimePower(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNotPrimePower", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsProvablyComposite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsProvablyComposite", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes+MROutput")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::Primes_MROutput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes")]
#[repr(C)]
#[derive(Debug)]
pub struct Primes {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::Primes =>
    "Org.BouncyCastle.Math"."Primes"
);
#[cfg(feature = "Org+BouncyCastle+Math+Primes")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::Primes {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::Primes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes")]
impl crate::Org::BouncyCastle::Math::Primes {
    #[cfg(feature = "Org+BouncyCastle+Math+Primes+MROutput")]
    pub type MROutput = crate::Org::BouncyCastle::Math::Primes_MROutput;
    #[cfg(feature = "Org+BouncyCastle+Math+Primes+STOutput")]
    pub type STOutput = crate::Org::BouncyCastle::Math::Primes_STOutput;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Math::Primes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes+STOutput")]
#[repr(C)]
#[derive(Debug)]
pub struct Primes_STOutput {
    __cordl_parent: crate::System::Object,
    pub mPrime: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub mPrimeSeed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mPrimeGenCounter: i32,
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes+STOutput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::Primes_STOutput =>
    "Org.BouncyCastle.Math"."Primes/STOutput"
);
#[cfg(feature = "Org+BouncyCastle+Math+Primes+STOutput")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::Primes_STOutput {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes+STOutput")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::Primes_STOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes+STOutput")]
impl crate::Org::BouncyCastle::Math::Primes_STOutput {
    pub fn New(
        prime: *mut crate::Org::BouncyCastle::Math::BigInteger,
        primeSeed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        primeGenCounter: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (prime, primeSeed, primeGenCounter))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        prime: *mut crate::Org::BouncyCastle::Math::BigInteger,
        primeSeed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        primeGenCounter: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (prime, primeSeed, primeGenCounter))?;
        Ok(__cordl_ret)
    }
    pub fn get_Prime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_Prime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrimeGenCounter(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PrimeGenCounter", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrimeSeed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("get_PrimeSeed", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes+STOutput")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::Primes_STOutput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
