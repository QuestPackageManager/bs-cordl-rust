#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve")]
#[repr(C)]
#[derive(Debug)]
pub struct SecP160R2Curve {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::AbstractFpCurve,
    pub m_infinity: *mut crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Point,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve =>
    "Org.BouncyCastle.Math.EC.Custom.Sec"."SecP160R2Curve"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve {
    type Target = crate::Org::BouncyCastle::Math::EC::AbstractFpCurve;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve")]
impl crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve {
    pub const SECP160R2_DEFAULT_COORDS: i32 = 2i32;
    pub const SECP160R2_FE_INTS: i32 = 5i32;
    #[cfg(
        feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve+SecP160R2LookupTable"
    )]
    pub type SecP160R2LookupTable = crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve_SecP160R2LookupTable;
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
    pub fn RandomFieldElementMult(
        &mut self,
        r: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("RandomFieldElementMult", (r))?;
        Ok(__cordl_ret)
    }
    pub fn RandomFieldElement(
        &mut self,
        r: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("RandomFieldElement", (r))?;
        Ok(__cordl_ret)
    }
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
    pub fn get_FieldSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_FieldSize", ())?;
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
    pub fn get_Q(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_Q", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve+SecP160R2LookupTable"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SecP160R2Curve_SecP160R2LookupTable {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::AbstractECLookupTable,
    pub m_outer: *mut crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve,
    pub m_table: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub m_size: i32,
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve+SecP160R2LookupTable"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve_SecP160R2LookupTable =>
    "Org.BouncyCastle.Math.EC.Custom.Sec"."SecP160R2Curve/SecP160R2LookupTable"
);
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve+SecP160R2LookupTable"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve_SecP160R2LookupTable {
    type Target = crate::Org::BouncyCastle::Math::EC::AbstractECLookupTable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve+SecP160R2LookupTable"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve_SecP160R2LookupTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve+SecP160R2LookupTable"
)]
impl crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve_SecP160R2LookupTable {
    pub fn get_Size(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Size", ())?;
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
    pub fn CreatePoint(
        &mut self,
        x: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        y: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
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
    pub fn _ctor(
        &mut self,
        outer: *mut crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve,
        table: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (outer, table, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        outer: *mut crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve,
        table: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outer, table, _cordl_size))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Math+EC+Custom+Sec+SecP160R2Curve+SecP160R2LookupTable"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Custom::Sec::SecP160R2Curve_SecP160R2LookupTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
