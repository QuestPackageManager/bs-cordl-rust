#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+SimpleBigDecimal")]
#[repr(C)]
#[derive(Debug)]
pub struct SimpleBigDecimal {
    __cordl_parent: crate::System::Object,
    pub bigInt: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub scale: i32,
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+SimpleBigDecimal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal =>
    "Org.BouncyCastle.Math.EC.Abc"."SimpleBigDecimal"
);
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+SimpleBigDecimal")]
impl std::ops::Deref for crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+SimpleBigDecimal")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+SimpleBigDecimal")]
impl crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal {
    pub fn AdjustScale(
        &mut self,
        newScale: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal = __cordl_object
            .invoke("AdjustScale", (newScale))?;
        Ok(__cordl_ret)
    }
    pub fn get_IntValue(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_IntValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Scale(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Scale", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn Add_SimpleBigDecimal0(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal = __cordl_object
            .invoke("Add", (b))?;
        Ok(__cordl_ret)
    }
    pub fn Add_BigInteger1(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal = __cordl_object
            .invoke("Add", (b))?;
        Ok(__cordl_ret)
    }
    pub fn Divide_SimpleBigDecimal0(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal = __cordl_object
            .invoke("Divide", (b))?;
        Ok(__cordl_ret)
    }
    pub fn Divide_BigInteger1(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal = __cordl_object
            .invoke("Divide", (b))?;
        Ok(__cordl_ret)
    }
    pub fn Floor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("Floor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Subtract_SimpleBigDecimal0(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal = __cordl_object
            .invoke("Subtract", (b))?;
        Ok(__cordl_ret)
    }
    pub fn Subtract_BigInteger1(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal = __cordl_object
            .invoke("Subtract", (b))?;
        Ok(__cordl_ret)
    }
    pub fn Negate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal = __cordl_object
            .invoke("Negate", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckScale(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckScale", (b))?;
        Ok(__cordl_ret)
    }
    pub fn CompareTo_SimpleBigDecimal0(
        &mut self,
        val: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (val))?;
        Ok(__cordl_ret)
    }
    pub fn CompareTo_BigInteger1(
        &mut self,
        val: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (val))?;
        Ok(__cordl_ret)
    }
    pub fn get_LongValue(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_LongValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BigInteger_i32_0(
        &mut self,
        bigInt: *mut crate::Org::BouncyCastle::Math::BigInteger,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bigInt, scale))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SimpleBigDecimal1(
        &mut self,
        limBigDec: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (limBigDec))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn Round(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("Round", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn Multiply_SimpleBigDecimal0(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal = __cordl_object
            .invoke("Multiply", (b))?;
        Ok(__cordl_ret)
    }
    pub fn Multiply_BigInteger1(
        &mut self,
        b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal = __cordl_object
            .invoke("Multiply", (b))?;
        Ok(__cordl_ret)
    }
    pub fn ShiftLeft(
        &mut self,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal = __cordl_object
            .invoke("ShiftLeft", (n))?;
        Ok(__cordl_ret)
    }
    pub fn New_BigInteger_i32_0(
        bigInt: *mut crate::Org::BouncyCastle::Math::BigInteger,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bigInt, scale))?;
        Ok(__cordl_object)
    }
    pub fn New_SimpleBigDecimal1(
        limBigDec: *mut crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (limBigDec))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Math+EC+Abc+SimpleBigDecimal")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Math::EC::Abc::SimpleBigDecimal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
