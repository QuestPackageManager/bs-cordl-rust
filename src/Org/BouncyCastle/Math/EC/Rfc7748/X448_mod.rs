#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc7748+X448")]
#[repr(C)]
#[derive(Debug)]
pub struct X448 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc7748+X448")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Rfc7748::X448 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Rfc7748";
    const CLASS_NAME: &'static str = "X448";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc7748+X448")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Rfc7748::X448 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc7748+X448")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Rfc7748::X448 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc7748+X448")]
impl crate::Org::BouncyCastle::Math::EC::Rfc7748::X448 {
    pub const C_A: u32 = 156326u32;
    pub const C_A24: u32 = 39082u32;
    pub const PointSize: i32 = 56i32;
    pub const ScalarSize: i32 = 56i32;
    pub fn CalculateAgreement(
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        kOff: i32,
        u: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        uOff: i32,
        r: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        rOff: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                ),
                bool,
                6usize,
            >("CalculateAgreement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CalculateAgreement", 6usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (k, kOff, u, uOff, r, rOff))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Decode32(
        bs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>, i32),
                u32,
                2usize,
            >("Decode32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Decode32", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (bs, off)) };
        Ok(__cordl_ret.into())
    }
    pub fn DecodeScalar(
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        kOff: i32,
        n: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DecodeScalar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DecodeScalar", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (k, kOff, n))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePrivateKey(
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Security::SecureRandom,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GeneratePrivateKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GeneratePrivateKey", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (random, k))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePublicKey(
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        kOff: i32,
        r: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        rOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("GeneratePublicKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GeneratePublicKey", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (k, kOff, r, rOff))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PointDouble(
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("PointDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PointDouble", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (x, z))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Precompute() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("Precompute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Precompute", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScalarMult(
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        kOff: i32,
        u: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        uOff: i32,
        r: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        rOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("ScalarMult")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScalarMult", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (k, kOff, u, uOff, r, rOff))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScalarMultBase(
        k: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        kOff: i32,
        r: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        rOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("ScalarMultBase")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScalarMultBase", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (k, kOff, r, rOff))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Rfc7748+X448")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Rfc7748::X448 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
