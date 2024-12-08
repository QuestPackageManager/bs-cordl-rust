#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mFieldElement")]
#[repr(C)]
#[derive(Debug)]
pub struct F2mFieldElement {
    __cordl_parent: crate::Org::BouncyCastle::Math::EC::AbstractF2mFieldElement,
    pub representation: i32,
    pub m: i32,
    pub ks: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub x: *mut crate::Org::BouncyCastle::Math::EC::LongArray,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mFieldElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Math::EC::F2mFieldElement =>
    "Org.BouncyCastle.Math.EC"."F2mFieldElement"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mFieldElement")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::F2mFieldElement {
    type Target = crate::Org::BouncyCastle::Math::EC::AbstractF2mFieldElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mFieldElement")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::F2mFieldElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mFieldElement")]
impl crate::Org::BouncyCastle::Math::EC::F2mFieldElement {
    pub const Gnb: i32 = 1i32;
    pub const Ppb: i32 = 3i32;
    pub const Tpb: i32 = 2i32;
    pub fn Add(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("Add", (b))?;
        Ok(__cordl_ret)
    }
    pub fn MultiplyMinusProduct(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("MultiplyMinusProduct", (b, x, y))?;
        Ok(__cordl_ret)
    }
    pub fn get_Representation(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Representation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_K1(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_K1", ())?;
        Ok(__cordl_ret)
    }
    pub fn Invert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("Invert", ())?;
        Ok(__cordl_ret)
    }
    pub fn TestBitZero(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TestBitZero", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsOne(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsOne", ())?;
        Ok(__cordl_ret)
    }
    pub fn SquareMinusProduct(
        &mut self,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("SquareMinusProduct", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn MultiplyPlusProduct(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("MultiplyPlusProduct", (b, x, y))?;
        Ok(__cordl_ret)
    }
    pub fn Sqrt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("Sqrt", ())?;
        Ok(__cordl_ret)
    }
    pub fn Divide(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("Divide", (b))?;
        Ok(__cordl_ret)
    }
    pub fn get_FieldName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_FieldName", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddOne(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("AddOne", ())?;
        Ok(__cordl_ret)
    }
    pub fn SquarePlusProduct(
        &mut self,
        x: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
        y: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("SquarePlusProduct", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn get_K2(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_K2", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn Equals_F2mFieldElement1(
        &mut self,
        other: *mut crate::Org::BouncyCastle::Math::EC::F2mFieldElement,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret)
    }
    pub fn SquarePow(
        &mut self,
        pow: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("SquarePow", (pow))?;
        Ok(__cordl_ret)
    }
    pub fn Square(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("Square", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FieldSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_FieldSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsZero(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsZero", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_M(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_M", ())?;
        Ok(__cordl_ret)
    }
    pub fn Subtract(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("Subtract", (b))?;
        Ok(__cordl_ret)
    }
    pub fn ToBigInteger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("ToBigInteger", ())?;
        Ok(__cordl_ret)
    }
    pub fn Negate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("Negate", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_BigInteger0(
        &mut self,
        m: i32,
        k1: i32,
        k2: i32,
        k3: i32,
        x: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (m, k1, k2, k3, x))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_BigInteger1(
        &mut self,
        m: i32,
        k: i32,
        x: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (m, k, x))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_LongArray2(
        &mut self,
        m: i32,
        ks: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        x: *mut crate::Org::BouncyCastle::Math::EC::LongArray,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (m, ks, x))?;
        Ok(__cordl_ret)
    }
    pub fn get_K3(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_K3", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BitLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BitLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn Multiply(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement = __cordl_object
            .invoke("Multiply", (b))?;
        Ok(__cordl_ret)
    }
    pub fn New_i32_i32_i32_BigInteger0(
        m: i32,
        k1: i32,
        k2: i32,
        k3: i32,
        x: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (m, k1, k2, k3, x))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_BigInteger1(
        m: i32,
        k: i32,
        x: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (m, k, x))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_LongArray2(
        m: i32,
        ks: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        x: *mut crate::Org::BouncyCastle::Math::EC::LongArray,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (m, ks, x))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+F2mFieldElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::F2mFieldElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
