#[cfg(feature = "Mono+Math+BigInteger")]
#[repr(C)]
#[derive(Debug)]
pub struct BigInteger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub length: u32,
    pub data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
}
#[cfg(feature = "Mono+Math+BigInteger")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Math::BigInteger {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Math";
    const CLASS_NAME: &'static str = "BigInteger";
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
#[cfg(feature = "Mono+Math+BigInteger")]
impl std::ops::Deref for crate::Mono::Math::BigInteger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger")]
impl std::ops::DerefMut for crate::Mono::Math::BigInteger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger")]
impl crate::Mono::Math::BigInteger {
    #[cfg(feature = "Mono+Math+BigInteger+Kernel")]
    pub type Kernel = crate::Mono::Math::BigInteger_Kernel;
    #[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
    pub type ModulusRing = crate::Mono::Math::BigInteger_ModulusRing;
    #[cfg(feature = "Mono+Math+BigInteger+Sign")]
    pub type Sign = crate::Mono::Math::BigInteger_Sign;
    pub fn BitCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("BitCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "BitCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "Clear", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (o))? };
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePseudoPrime(
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                1usize,
            >("GeneratePseudoPrime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "GeneratePseudoPrime", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateRandom_RandomNumberGenerator0(
        bits: i32,
        rng: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RandomNumberGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Security::Cryptography::RandomNumberGenerator,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("GenerateRandom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "GenerateRandom", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bits, rng))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateRandom_i32_1(
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                1usize,
            >("GenerateRandom")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "GenerateRandom", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                0usize,
            >("GetBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "GetBytes", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Incr2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Incr2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "Incr2", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsProbablePrime(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("IsProbablePrime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "IsProbablePrime", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn LowestSetBit(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("LowestSetBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "LowestSetBit", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ModInverse(
        &mut self,
        modulus: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                1usize,
            >("ModInverse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "ModInverse", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked(self, (modulus))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ModPow(
        &mut self,
        exp: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        n: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("ModPow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "ModPow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked(self, (exp, n))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_BigInteger1(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bi))?;
        Ok(__cordl_object.into())
    }
    pub fn New_BigInteger_Sign_u32_0(
        sign: crate::Mono::Math::BigInteger_Sign,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sign, len))?;
        Ok(__cordl_object.into())
    }
    pub fn New_BigInteger_u32_2(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bi, len))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray3(
        inData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (inData))?;
        Ok(__cordl_object.into())
    }
    pub fn New_u32_4(
        ui: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ui))?;
        Ok(__cordl_object.into())
    }
    pub fn Normalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Normalize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "Normalize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Randomize_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Randomize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "Randomize", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Randomize_RandomNumberGenerator0(
        &mut self,
        rng: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RandomNumberGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::RandomNumberGenerator,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Randomize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "Randomize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rng))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBit__cordl_bool1(
        &mut self,
        bitNum: u32,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32, bool), quest_hook::libil2cpp::Void, 2usize>("SetBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "SetBit", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bitNum, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBit_u32_0(
        &mut self,
        bitNum: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>("SetBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "SetBit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bitNum))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TestBit_i32_1(&mut self, bitNum: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), bool, 1usize>("TestBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "TestBit", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (bitNum))? };
        Ok(__cordl_ret.into())
    }
    pub fn TestBit_u32_0(&mut self, bitNum: u32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), bool, 1usize>("TestBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "TestBit", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (bitNum))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u32_0(
        &mut self,
        radix: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (radix))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_u32_Il2CppString1(
        &mut self,
        radix: u32,
        characterSet: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (radix, characterSet))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BigInteger1(
        &mut self,
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bi))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BigInteger_Sign_u32_0(
        &mut self,
        sign: crate::Mono::Math::BigInteger_Sign,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Mono::Math::BigInteger_Sign, u32),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (sign, len))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_BigInteger_u32_2(
        &mut self,
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, u32),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bi, len))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray3(
        &mut self,
        inData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (inData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_4(
        &mut self,
        ui: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ui))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Rng() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RandomNumberGenerator,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::RandomNumberGenerator,
                >,
                0usize,
            >("get_Rng")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "get_Rng", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RandomNumberGenerator,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Division(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("op_Division")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Division", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bi1, bi2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_BigInteger1(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                bool,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Equality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (bi1, bi2))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_u32_0(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        ui: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, u32),
                bool,
                2usize,
            >("op_Equality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Equality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (bi1, ui))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                bool,
                2usize,
            >("op_GreaterThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThan", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (bi1, bi2))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                bool,
                2usize,
            >("op_GreaterThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_GreaterThanOrEqual", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (bi1, bi2))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Implicit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_0(
        value: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u32),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                1usize,
            >("op_Implicit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Implicit", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_BigInteger1(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                bool,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Inequality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (bi1, bi2))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_u32_0(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        ui: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, u32),
                bool,
                2usize,
            >("op_Inequality")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Inequality", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (bi1, ui))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_LeftShift(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        shiftVal: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, i32),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("op_LeftShift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_LeftShift", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bi1, shiftVal))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                bool,
                2usize,
            >("op_LessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThan", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (bi1, bi2))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                bool,
                2usize,
            >("op_LessThanOrEqual")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_LessThanOrEqual", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (bi1, bi2))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_BigInteger1(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("op_Modulus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Modulus", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bi1, bi2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_u32_0(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        ui: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, u32),
                u32,
                2usize,
            >("op_Modulus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Modulus", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (bi, ui))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_BigInteger0(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Multiply", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bi1, bi2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_i32_1(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, i32),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("op_Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Multiply", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bi, i))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_RightShift(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        shiftVal: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, i32),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("op_RightShift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_RightShift", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bi1, shiftVal))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("op_Subtraction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger as quest_hook::libil2cpp::Type >
                    ::class(), "op_Subtraction", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bi1, bi2))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Math+BigInteger")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Math::BigInteger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
#[repr(C)]
#[derive(Debug)]
pub struct BigInteger_Kernel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Math::BigInteger_Kernel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Math";
    const CLASS_NAME: &'static str = "BigInteger/Kernel";
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
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
impl std::ops::Deref for crate::Mono::Math::BigInteger_Kernel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
impl std::ops::DerefMut for crate::Mono::Math::BigInteger_Kernel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
impl crate::Mono::Math::BigInteger_Kernel {
    pub fn Compare(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Math::BigInteger_Sign> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                crate::Mono::Math::BigInteger_Sign,
                2usize,
            >("Compare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "Compare", 2usize
                )
            });
        let __cordl_ret: crate::Mono::Math::BigInteger_Sign = unsafe {
            method.invoke_unchecked((), (bi1, bi2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DwordDivMod(
        n: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        d: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, u32),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    >,
                >,
                2usize,
            >("DwordDivMod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "DwordDivMod", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
            >,
        > = unsafe { method.invoke_unchecked((), (n, d))? };
        Ok(__cordl_ret.into())
    }
    pub fn DwordMod(
        n: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        d: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, u32),
                u32,
                2usize,
            >("DwordMod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "DwordMod", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (n, d))? };
        Ok(__cordl_ret.into())
    }
    pub fn LeftShift(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, i32),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("LeftShift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "LeftShift", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bi, n))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MinusEq(
        big: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        small: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("MinusEq")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "MinusEq", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (big, small))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Multiply(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOffset: u32,
        xLen: u32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOffset: u32,
        yLen: u32,
        d: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        dOffset: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    u32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    u32,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    u32,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "Multiply", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked((), (x, xOffset, xLen, y, yOffset, yLen, d, dOffset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyByDword(
        n: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        f: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, u32),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("MultiplyByDword")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "MultiplyByDword", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (n, f))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyMod2p32pmod(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        xOffset: i32,
        xLen: i32,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        yOffest: i32,
        yLen: i32,
        d: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        dOffset: i32,
        _cordl_mod: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                9usize,
            >("MultiplyMod2p32pmod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "MultiplyMod2p32pmod", 9usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (x, xOffset, xLen, y, yOffest, yLen, d, dOffset, _cordl_mod),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PlusEq(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("PlusEq")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "PlusEq", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (bi1, bi2))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RightShift(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, i32),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("RightShift")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "RightShift", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bi, n))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SingleByteDivideInPlace(
        n: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        d: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, u32),
                u32,
                2usize,
            >("SingleByteDivideInPlace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "SingleByteDivideInPlace", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (n, d))? };
        Ok(__cordl_ret.into())
    }
    pub fn Subtract(
        big: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        small: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("Subtract")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "Subtract", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (big, small))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn modInverse_BigInteger1(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        modulus: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("modInverse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "modInverse", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked((), (bi, modulus))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn modInverse_u32_0(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        modulus: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>, u32),
                u32,
                2usize,
            >("modInverse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "modInverse", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (bi, modulus))? };
        Ok(__cordl_ret.into())
    }
    pub fn multiByteDivide(
        bi1: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        bi2: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    >,
                >,
                2usize,
            >("multiByteDivide")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_Kernel as quest_hook::libil2cpp::Type
                    > ::class(), "multiByteDivide", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
            >,
        > = unsafe { method.invoke_unchecked((), (bi1, bi2))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Kernel")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Math::BigInteger_Kernel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
#[repr(C)]
#[derive(Debug)]
pub struct BigInteger_ModulusRing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_mod: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    pub constant: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
}
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Math::BigInteger_ModulusRing {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Math";
    const CLASS_NAME: &'static str = "BigInteger/ModulusRing";
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
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
impl std::ops::Deref for crate::Mono::Math::BigInteger_ModulusRing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
impl std::ops::DerefMut for crate::Mono::Math::BigInteger_ModulusRing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
impl crate::Mono::Math::BigInteger_ModulusRing {
    pub fn BarrettReduction(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_ModulusRing as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("BarrettReduction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_ModulusRing as
                    quest_hook::libil2cpp::Type > ::class(), "BarrettReduction", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (x))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Difference(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        b: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_ModulusRing as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("Difference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_ModulusRing as
                    quest_hook::libil2cpp::Type > ::class(), "Difference", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked(self, (a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Multiply(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        b: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_ModulusRing as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("Multiply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_ModulusRing as
                    quest_hook::libil2cpp::Type > ::class(), "Multiply", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked(self, (a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        modulus: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (modulus))?;
        Ok(__cordl_object.into())
    }
    pub fn Pow_BigInteger0(
        &mut self,
        a: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        k: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_ModulusRing as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                    quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("Pow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_ModulusRing as
                    quest_hook::libil2cpp::Type > ::class(), "Pow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked(self, (a, k))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Pow_u32_1(
        &mut self,
        b: u32,
        exp: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_ModulusRing as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32, quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>),
                quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
                2usize,
            >("Pow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_ModulusRing as
                    quest_hook::libil2cpp::Type > ::class(), "Pow", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = unsafe {
            method.invoke_unchecked(self, (b, exp))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        modulus: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Math::BigInteger_ModulusRing as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Math::BigInteger_ModulusRing as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (modulus))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Math+BigInteger+ModulusRing")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Math::BigInteger_ModulusRing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Sign")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BigInteger_Sign {
    #[default]
    Negative = -1i32,
    Positive = 1i32,
    Zero = 0i32,
}
#[cfg(feature = "Mono+Math+BigInteger+Sign")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Math::BigInteger_Sign {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Math";
    const CLASS_NAME: &'static str = "BigInteger/Sign";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Sign")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Mono::Math::BigInteger_Sign {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Sign")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Mono::Math::BigInteger_Sign {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Sign")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Mono::Math::BigInteger_Sign {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Mono+Math+BigInteger+Sign")]
unsafe impl quest_hook::libil2cpp::Return for crate::Mono::Math::BigInteger_Sign {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
