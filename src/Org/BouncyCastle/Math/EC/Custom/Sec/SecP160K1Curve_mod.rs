#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve")]
#[repr(C)]
#[derive(Debug)]
pub struct SecP160K1Curve {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::AbstractFpCurve,
    pub m_infinity: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Point,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Custom.Sec";
    const CLASS_NAME: &'static str = "SecP160K1Curve";
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
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve {
    type Target = crate::Org::BouncyCastle::Math::EC::AbstractFpCurve;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve")]
impl crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve {
    pub const SECP160K1_DEFAULT_COORDS: i32 = 2i32;
    pub const SECP160K1_FE_INTS: i32 = 5i32;
    #[cfg(
        feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve+SecP160K1LookupTable"
    )]
    pub type SecP160K1LookupTable = crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve_SecP160K1LookupTable;
    pub fn CloneCurve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECCurve>,
                0usize,
            >("CloneCurve")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CloneCurve", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECCurve,
        > = unsafe { method.invoke_unchecked(self, ()) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "CreateCacheSafeLookupTable", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECLookupTable,
        > = unsafe { method.invoke_unchecked(self, (points, off, len)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
                4usize,
            >("CreateRawPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateRawPoint", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { method.invoke_unchecked(self, (x, y, zs, withCompression)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
                3usize,
            >("CreateRawPoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateRawPoint", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { method.invoke_unchecked(self, (x, y, withCompression)) };
        Ok(__cordl_ret.into())
    }
    pub fn FromBigInteger(
        &mut self,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Math::EC::ECFieldElement,
                >,
                1usize,
            >("FromBigInteger")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FromBigInteger", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        > = unsafe { method.invoke_unchecked(self, (x)) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RandomFieldElement(
        &mut self,
        r: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "RandomFieldElement", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        > = unsafe { method.invoke_unchecked(self, (r)) };
        Ok(__cordl_ret.into())
    }
    pub fn RandomFieldElementMult(
        &mut self,
        r: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECFieldElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "RandomFieldElementMult", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        > = unsafe { method.invoke_unchecked(self, (r)) };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsCoordinateSystem(
        &mut self,
        coord: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), bool, 1usize>("SupportsCoordinateSystem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SupportsCoordinateSystem", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (coord)) };
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
    pub fn get_FieldSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_FieldSize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_FieldSize", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Infinity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
                0usize,
            >("get_Infinity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Infinity", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Q(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                0usize,
            >("get_Q")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Q", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve+SecP160K1LookupTable"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SecP160K1Curve_SecP160K1LookupTable {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::AbstractECLookupTable,
    pub m_outer: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve,
    >,
    pub m_table: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    pub m_size: i32,
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve+SecP160K1LookupTable"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve_SecP160K1LookupTable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Math.EC.Custom.Sec";
    const CLASS_NAME: &'static str = "SecP160K1Curve/SecP160K1LookupTable";
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
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve+SecP160K1LookupTable"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve_SecP160K1LookupTable {
    type Target = crate::Org::BouncyCastle::Math::EC::AbstractECLookupTable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve+SecP160K1LookupTable"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve_SecP160K1LookupTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve+SecP160K1LookupTable"
)]
impl crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve_SecP160K1LookupTable {
    pub fn CreatePoint(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        y: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
                2usize,
            >("CreatePoint")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreatePoint", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { method.invoke_unchecked(self, (x, y)) };
        Ok(__cordl_ret.into())
    }
    pub fn Lookup(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
                1usize,
            >("Lookup")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Lookup", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn LookupVar(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::EC::ECPoint>,
                1usize,
            >("LookupVar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LookupVar", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::ECPoint,
        > = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        outer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve,
        >,
        table: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
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
        outer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve,
        >,
        table: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (outer, table, _cordl_size))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Size(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Size")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Size", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160K1Curve+SecP160K1LookupTable"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160K1Curve_SecP160K1LookupTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
