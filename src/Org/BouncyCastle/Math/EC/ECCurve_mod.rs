#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve")]
#[repr(C)]
#[derive(Debug)]
pub struct ECCurve {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_field: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::Field::IFiniteField,
    >,
    pub m_a: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    >,
    pub m_b: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    >,
    pub m_order: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub m_cofactor: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::BigInteger,
    >,
    pub m_coord: i32,
    pub m_endomorphism: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
    >,
    pub m_multiplier: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve")]
unsafe impl quest_hook::libil2cpp::Type for crate::Org::BouncyCastle::Math::EC::ECCurve {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC";
    const CLASS_NAME: &'static str = "ECCurve";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::ECCurve {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::ECCurve {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve")]
impl crate::Org::BouncyCastle::Math::EC::ECCurve {
    pub const COORD_AFFINE: i32 = 0i32;
    pub const COORD_HOMOGENEOUS: i32 = 1i32;
    pub const COORD_JACOBIAN: i32 = 2i32;
    pub const COORD_JACOBIAN_CHUDNOVSKY: i32 = 3i32;
    pub const COORD_JACOBIAN_MODIFIED: i32 = 4i32;
    pub const COORD_LAMBDA_AFFINE: i32 = 5i32;
    pub const COORD_LAMBDA_PROJECTIVE: i32 = 6i32;
    pub const COORD_SKEWED: i32 = 7i32;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+Config")]
    pub type Config = crate::Org::BouncyCastle::Math::EC::ECCurve_Config;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+DefaultLookupTable")]
    pub type DefaultLookupTable = crate::Org::BouncyCastle::Math::EC::ECCurve_DefaultLookupTable;
    pub fn CheckPoint(
        &mut self,
        point: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckPoint", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (point))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckPoints_Il2CppArray0(
        &mut self,
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::Org::BouncyCastle::Math::EC::ECPoint,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckPoints")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckPoints", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (points))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckPoints_i32_i32_1(
        &mut self,
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
            >,
        >,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::Org::BouncyCastle::Math::EC::ECPoint,
                                    >,
                                >,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("CheckPoints")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckPoints", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (points, off, len))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CloneCurve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECCurve,
                        >,
                        0usize,
                    >("CloneCurve")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CloneCurve", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECCurve,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Configure(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve_Config>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECCurve_Config,
                        >,
                        0usize,
                    >("Configure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Configure", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECCurve_Config,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateCacheSafeLookupTable(
        &mut self,
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
            >,
        >,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECLookupTable>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::Org::BouncyCastle::Math::EC::ECPoint,
                                    >,
                                >,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECLookupTable,
                        >,
                        3usize,
                    >("CreateCacheSafeLookupTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateCacheSafeLookupTable", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECLookupTable,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (points, off, len))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDefaultMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
                        >,
                        0usize,
                    >("CreateDefaultMultiplier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateDefaultMultiplier", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CreatePoint_BigInteger_BigInteger0(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        y: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        2usize,
                    >("CreatePoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreatePoint", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreatePoint__cordl_bool1(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        y: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        withCompression: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        3usize,
                    >("CreatePoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreatePoint", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (x, y, withCompression))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRawPoint_Il2CppArray__cordl_bool1(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
        y: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
        zs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                >,
            >,
        >,
        withCompression: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                                    >,
                                >,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        4usize,
                    >("CreateRawPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateRawPoint", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (x, y, zs, withCompression))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRawPoint__cordl_bool0(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
        y: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
        withCompression: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        3usize,
                    >("CreateRawPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateRawPoint", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (x, y, withCompression))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DecodePoint(
        &mut self,
        encoded: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        1usize,
                    >("DecodePoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DecodePoint", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (encoded))? };
        Ok(__cordl_ret.into())
    }
    pub fn DecompressPoint(
        &mut self,
        yTilde: i32,
        X1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        2usize,
                    >("DecompressPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DecompressPoint", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (yTilde, X1))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_ECCurve0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECCurve,
                        >),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromBigInteger(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                        >,
                        1usize,
                    >("FromBigInteger")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromBigInteger", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (x))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllCoordinateSystems() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<i32>,
                        >,
                        0usize,
                    >("GetAllCoordinateSystems")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAllCoordinateSystems", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEndomorphism(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
                        >,
                        0usize,
                    >("GetEndomorphism")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEndomorphism", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCode", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
                        >,
                        0usize,
                    >("GetMultiplier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMultiplier", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPreCompInfo(
        &mut self,
        point: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::ECPoint,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
                        >,
                        2usize,
                    >("GetPreCompInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPreCompInfo", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (point, name))? };
        Ok(__cordl_ret.into())
    }
    pub fn ImportPoint(
        &mut self,
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        1usize,
                    >("ImportPoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ImportPoint", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (p))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsValidFieldElement(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >),
                        bool,
                        1usize,
                    >("IsValidFieldElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsValidFieldElement", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (x))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        field: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IFiniteField,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (field))?;
        Ok(__cordl_object.into())
    }
    pub fn NormalizeAll_Il2CppArray0(
        &mut self,
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::Org::BouncyCastle::Math::EC::ECPoint,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("NormalizeAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NormalizeAll", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (points))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NormalizeAll_i32_i32_ECFieldElement1(
        &mut self,
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
            >,
        >,
        off: i32,
        len: i32,
        iso: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::Org::BouncyCastle::Math::EC::ECPoint,
                                    >,
                                >,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("NormalizeAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "NormalizeAll", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (points, off, len, iso))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Precompute(
        &mut self,
        point: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::ECPoint,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::Multiplier::IPreCompCallback,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
                        >,
                        3usize,
                    >("Precompute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Precompute", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::PreCompInfo,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (point, name, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RandomFieldElement(
        &mut self,
        r: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Security::SecureRandom,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                        >,
                        1usize,
                    >("RandomFieldElement")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RandomFieldElement", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (r))? };
        Ok(__cordl_ret.into())
    }
    pub fn RandomFieldElementMult(
        &mut self,
        r: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Security::SecureRandom,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                        >,
                        1usize,
                    >("RandomFieldElementMult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RandomFieldElementMult", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (r))? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsCoordinateSystem(
        &mut self,
        coord: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), bool, 1usize>("SupportsCoordinateSystem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SupportsCoordinateSystem", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (coord))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidatePoint_BigInteger_BigInteger0(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        y: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        2usize,
                    >("ValidatePoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidatePoint", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidatePoint__cordl_bool1(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        y: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        withCompression: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::BigInteger,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        3usize,
                    >("ValidatePoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidatePoint", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (x, y, withCompression))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        field: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IFiniteField,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::Field::IFiniteField,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (field))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_A(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                        >,
                        0usize,
                    >("get_A")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_A",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_B(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                        >,
                        0usize,
                    >("get_B")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_B",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Cofactor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                        0usize,
                    >("get_Cofactor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Cofactor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_CoordinateSystem(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_CoordinateSystem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_CoordinateSystem", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Field(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::Field::IFiniteField>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::Field::IFiniteField,
                        >,
                        0usize,
                    >("get_Field")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Field", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::Field::IFiniteField,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_FieldSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_FieldSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_FieldSize", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Infinity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        0usize,
                    >("get_Infinity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Infinity", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Order(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::BigInteger,
                        >,
                        0usize,
                    >("get_Order")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Order", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Math::EC::ECCurve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+Config")]
#[repr(C)]
#[derive(Debug)]
pub struct ECCurve_Config {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub outer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    pub coord: i32,
    pub endomorphism: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
    >,
    pub multiplier: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+Config")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::ECCurve_Config {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC";
    const CLASS_NAME: &'static str = "ECCurve/Config";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+Config")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::ECCurve_Config {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+Config")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::ECCurve_Config {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+Config")]
impl crate::Org::BouncyCastle::Math::EC::ECCurve_Config {
    pub fn Create(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECCurve,
                        >,
                        0usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECCurve,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        outer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        coord: i32,
        endomorphism: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        >,
        multiplier: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outer, coord, endomorphism, multiplier))?;
        Ok(__cordl_object.into())
    }
    pub fn SetCoordinateSystem(
        &mut self,
        coord: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve_Config>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECCurve_Config,
                        >,
                        1usize,
                    >("SetCoordinateSystem")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetCoordinateSystem", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECCurve_Config,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (coord))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetEndomorphism(
        &mut self,
        endomorphism: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve_Config>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECCurve_Config,
                        >,
                        1usize,
                    >("SetEndomorphism")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetEndomorphism", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECCurve_Config,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (endomorphism))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetMultiplier(
        &mut self,
        multiplier: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve_Config>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECCurve_Config,
                        >,
                        1usize,
                    >("SetMultiplier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetMultiplier", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECCurve_Config,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (multiplier))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        outer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        coord: i32,
        endomorphism: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
        >,
        multiplier: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::ECCurve,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::Endo::ECEndomorphism,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (outer, coord, endomorphism, multiplier))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+Config")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::ECCurve_Config {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+DefaultLookupTable")]
#[repr(C)]
#[derive(Debug)]
pub struct ECCurve_DefaultLookupTable {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::AbstractECLookupTable,
    pub m_outer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    pub m_table: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub m_size: i32,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+DefaultLookupTable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::ECCurve_DefaultLookupTable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC";
    const CLASS_NAME: &'static str = "ECCurve/DefaultLookupTable";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+DefaultLookupTable")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::ECCurve_DefaultLookupTable {
    type Target = crate::Org::BouncyCastle::Math::EC::AbstractECLookupTable;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+DefaultLookupTable")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::ECCurve_DefaultLookupTable {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+DefaultLookupTable")]
impl crate::Org::BouncyCastle::Math::EC::ECCurve_DefaultLookupTable {
    pub fn CreatePoint(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        2usize,
                    >("CreatePoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreatePoint", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (x, y))? };
        Ok(__cordl_ret.into())
    }
    pub fn Lookup(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        1usize,
                    >("Lookup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Lookup",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn LookupVar(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Math::EC::ECPoint,
                        >,
                        1usize,
                    >("LookupVar")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LookupVar", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        outer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        table: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outer, table, _cordl_size))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        outer: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
        table: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Math::EC::ECCurve,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (outer, table, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Size(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_Size")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Size", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+ECCurve+DefaultLookupTable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::ECCurve_DefaultLookupTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
