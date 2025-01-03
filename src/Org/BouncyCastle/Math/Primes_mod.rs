#[cfg(feature = "Org+BouncyCastle+Math+Primes")]
#[repr(C)]
#[derive(Debug)]
pub struct Primes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+Primes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::Primes =>
    "Org.BouncyCastle.Math"."Primes"
);
#[cfg(feature = "Org+BouncyCastle+Math+Primes")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::Primes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CheckCandidate(
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckCandidate", (n, name))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnhancedMRProbablePrimeTest(
        candidate: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        iterations: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::Primes_MROutput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Primes_MROutput,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnhancedMRProbablePrimeTest", (candidate, random, iterations))?;
        Ok(__cordl_ret.into())
    }
    pub fn Extract32(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Extract32", (bs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateSTRandomPrime(
        hash: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        length: i32,
        inputSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::Primes_STOutput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Primes_STOutput,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateSTRandomPrime", (hash, length, inputSeed))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAnySmallFactors(
        candidate: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAnySmallFactors", (candidate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Hash(
        d: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outPos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Hash", (d, input, output, outPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn HashGen(
        d: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HashGen", (d, seed, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplHasAnySmallFactors(
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplHasAnySmallFactors", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplMRProbablePrimeToBase(
        w: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        wSubOne: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        m: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        a: i32,
        b: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplMRProbablePrimeToBase", (w, wSubOne, m, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplSTRandomPrime(
        d: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        length: i32,
        primeSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::Primes_STOutput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Primes_STOutput,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ImplSTRandomPrime", (d, length, primeSeed))?;
        Ok(__cordl_ret.into())
    }
    pub fn Inc(
        seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        c: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Inc", (seed, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMRProbablePrime(
        candidate: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        iterations: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMRProbablePrime", (candidate, random, iterations))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMRProbablePrimeToBase(
        candidate: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        baseValue: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMRProbablePrimeToBase", (candidate, baseValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrime32(x: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrime32", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Math+Primes+MROutput")]
#[repr(C)]
#[derive(Debug)]
pub struct Primes_MROutput {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        factor: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (provablyComposite, factor))?;
        Ok(__cordl_object.into())
    }
    pub fn ProbablyPrime() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::Primes_MROutput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Primes_MROutput,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ProbablyPrime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProvablyCompositeNotPrimePower() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::Primes_MROutput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Primes_MROutput,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProvablyCompositeNotPrimePower", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProvablyCompositeWithFactor(
        factor: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::Primes_MROutput>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Primes_MROutput,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProvablyCompositeWithFactor", (factor))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        provablyComposite: bool,
        factor: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (provablyComposite, factor))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Factor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Factor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNotPrimePower(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNotPrimePower", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsProvablyComposite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsProvablyComposite", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Math+Primes+STOutput")]
#[repr(C)]
#[derive(Debug)]
pub struct Primes_STOutput {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        prime: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        primeSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        primeGenCounter: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (prime, primeSeed, primeGenCounter))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        prime: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        primeSeed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        primeGenCounter: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (prime, primeSeed, primeGenCounter))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Prime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Prime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PrimeGenCounter(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PrimeGenCounter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PrimeSeed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_PrimeSeed", ())?;
        Ok(__cordl_ret.into())
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
