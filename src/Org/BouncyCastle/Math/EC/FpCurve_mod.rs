#[cfg(feature = "Org+BouncyCastle+Math+EC+FpCurve")]
#[repr(C)]
#[derive(Debug)]
pub struct FpCurve {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::AbstractFpCurve,
    pub m_q: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_r: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub m_infinity: *mut crate::Org::BouncyCastle::Math::EC::FpPoint,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+FpCurve")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::EC::FpCurve =>
    "Org.BouncyCastle.Math.EC"."FpCurve"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+FpCurve")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::FpCurve {
    type Target = crate::Org::BouncyCastle::Math::EC::AbstractFpCurve;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+FpCurve")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::FpCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+FpCurve")]
impl crate::Org::BouncyCastle::Math::EC::FpCurve {
    pub const FP_DEFAULT_COORDS: i32 = 4i32;
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
    pub fn _ctor_BigInteger0(
        &mut self,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (q, a, b))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger_BigInteger_BigInteger1(
        &mut self,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (q, a, b, order, cofactor))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ECFieldElement_ECFieldElement2(
        &mut self,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        r: *mut crate::Org::BouncyCastle::Math::BigInteger,
        a: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (q, r, a, b))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ECFieldElement_ECFieldElement_BigInteger_BigInteger3(
        &mut self,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        r: *mut crate::Org::BouncyCastle::Math::BigInteger,
        a: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (q, r, a, b, order, cofactor))?;
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
    pub fn get_FieldSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_FieldSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn ImportPoint(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("ImportPoint", (p))?;
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
    pub fn New_BigInteger0(
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (q, a, b))?;
        Ok(__cordl_object)
    }
    pub fn New_BigInteger_BigInteger_BigInteger1(
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        a: *mut crate::Org::BouncyCastle::Math::BigInteger,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (q, a, b, order, cofactor))?;
        Ok(__cordl_object)
    }
    pub fn New_ECFieldElement_ECFieldElement2(
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        r: *mut crate::Org::BouncyCastle::Math::BigInteger,
        a: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (q, r, a, b))?;
        Ok(__cordl_object)
    }
    pub fn New_ECFieldElement_ECFieldElement_BigInteger_BigInteger3(
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        r: *mut crate::Org::BouncyCastle::Math::BigInteger,
        a: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        order: *mut crate::Org::BouncyCastle::Math::BigInteger,
        cofactor: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (q, r, a, b, order, cofactor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+FpCurve")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Math::EC::FpCurve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
