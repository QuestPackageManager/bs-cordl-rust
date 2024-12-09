#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve")]
#[repr(C)]
#[derive(Debug)]
pub struct F2mCurve {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::AbstractF2mCurve,
    pub m: i32,
    pub k1: i32,
    pub k2: i32,
    pub k3: i32,
    pub m_infinity: *mut crate::Org::BouncyCastle::Math::EC::F2mPoint,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::EC::F2mCurve =>
    "Org.BouncyCastle.Math.EC"."F2mCurve"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::F2mCurve {
    type Target = crate::Org::BouncyCastle::Math::EC::AbstractF2mCurve;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::F2mCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve")]
impl crate::Org::BouncyCastle::Math::EC::F2mCurve {
    pub const F2M_DEFAULT_COORDS: i32 = 6i32;
    #[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve+DefaultF2mLookupTable")]
    pub type DefaultF2mLookupTable = crate::Org::BouncyCastle::Math::EC::F2mCurve_DefaultF2mLookupTable;
    pub fn CloneCurve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECCurve,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECCurve = __cordl_object
            .invoke("CloneCurve", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateCacheSafeLookupTable(
        &mut self,
        points: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        >,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECLookupTable,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECLookupTable = __cordl_object
            .invoke("CreateCacheSafeLookupTable", (points, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDefaultMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier = __cordl_object
            .invoke("CreateDefaultMultiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateRawPoint_Il2CppArray__cordl_bool1(
        &mut self,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        zs: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        >,
        withCompression: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("CreateRawPoint", (x, y, zs, withCompression))?;
        Ok(__cordl_ret)
    }
    pub fn CreateRawPoint__cordl_bool0(
        &mut self,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        withCompression: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("CreateRawPoint", (x, y, withCompression))?;
        Ok(__cordl_ret)
    }
    pub fn FromBigInteger(
        &mut self,
        x: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("FromBigInteger", (x))?;
        Ok(__cordl_ret)
    }
    pub fn IsTrinomial(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTrinomial", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_BigInteger_BigInteger0(
        m: i32,
        k: i32,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (m, k, a, b))?;
        Ok(__cordl_object)
    }
    pub fn New_BigInteger_BigInteger_BigInteger_BigInteger1(
        m: i32,
        k: i32,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (m, k, a, b, order, cofactor))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_BigInteger_BigInteger2(
        m: i32,
        k1: i32,
        k2: i32,
        k3: i32,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (m, k1, k2, k3, a, b))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_BigInteger_BigInteger_BigInteger_BigInteger3(
        m: i32,
        k1: i32,
        k2: i32,
        k3: i32,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (m, k1, k2, k3, a, b, order, cofactor))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_ECFieldElement_ECFieldElement_BigInteger_BigInteger4(
        m: i32,
        k1: i32,
        k2: i32,
        k3: i32,
        a: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (m, k1, k2, k3, a, b, order, cofactor))?;
        Ok(__cordl_object)
    }
    pub fn SupportsCoordinateSystem(
        &mut self,
        coord: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SupportsCoordinateSystem", (coord))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger_BigInteger0(
        &mut self,
        m: i32,
        k: i32,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (m, k, a, b))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger_BigInteger_BigInteger_BigInteger1(
        &mut self,
        m: i32,
        k: i32,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (m, k, a, b, order, cofactor))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_BigInteger_BigInteger2(
        &mut self,
        m: i32,
        k1: i32,
        k2: i32,
        k3: i32,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (m, k1, k2, k3, a, b))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_BigInteger_BigInteger_BigInteger_BigInteger3(
        &mut self,
        m: i32,
        k1: i32,
        k2: i32,
        k3: i32,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (m, k1, k2, k3, a, b, order, cofactor))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_ECFieldElement_ECFieldElement_BigInteger_BigInteger4(
        &mut self,
        m: i32,
        k1: i32,
        k2: i32,
        k3: i32,
        a: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (m, k1, k2, k3, a, b, order, cofactor))?;
        Ok(__cordl_ret)
    }
    pub fn get_FieldSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_FieldSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Infinity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("get_Infinity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_K1(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_K1", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_K2(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_K2", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_K3(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_K3", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_M(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_M", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Math::EC::F2mCurve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve+DefaultF2mLookupTable")]
#[repr(C)]
#[derive(Debug)]
pub struct F2mCurve_DefaultF2mLookupTable {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::AbstractECLookupTable,
    pub m_outer: *mut crate::Org::BouncyCastle::Math::EC::F2mCurve,
    pub m_table: *mut quest_hook::libil2cpp::Il2CppArray<i64>,
    pub m_size: i32,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve+DefaultF2mLookupTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::F2mCurve_DefaultF2mLookupTable =>
    "Org.BouncyCastle.Math.EC"."F2mCurve/DefaultF2mLookupTable"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve+DefaultF2mLookupTable")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::F2mCurve_DefaultF2mLookupTable {
    type Target = crate::Org::BouncyCastle::Math::EC::AbstractECLookupTable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve+DefaultF2mLookupTable")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::F2mCurve_DefaultF2mLookupTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve+DefaultF2mLookupTable")]
impl crate::Org::BouncyCastle::Math::EC::F2mCurve_DefaultF2mLookupTable {
    pub fn CreatePoint(
        &mut self,
        x: *mut quest_hook::libil2cpp::Il2CppArray<i64>,
        y: *mut quest_hook::libil2cpp::Il2CppArray<i64>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("CreatePoint", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn Lookup(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("Lookup", (index))?;
        Ok(__cordl_ret)
    }
    pub fn LookupVar(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("LookupVar", (index))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        outer: *mut crate::Org::BouncyCastle::Math::EC::F2mCurve,
        table: *mut quest_hook::libil2cpp::Il2CppArray<i64>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outer, table, _cordl_size))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        outer: *mut crate::Org::BouncyCastle::Math::EC::F2mCurve,
        table: *mut quest_hook::libil2cpp::Il2CppArray<i64>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (outer, table, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn get_Size(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Size", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mCurve+DefaultF2mLookupTable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::F2mCurve_DefaultF2mLookupTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
