#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct BigIntegerCalculator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::BigIntegerCalculator =>
    "System.Numerics"."BigIntegerCalculator"
);
#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
impl std::ops::Deref for crate::System::Numerics::BigIntegerCalculator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
impl std::ops::DerefMut for crate::System::Numerics::BigIntegerCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
impl crate::System::Numerics::BigIntegerCalculator {
    pub fn AddDivisor(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        leftLength: i32,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        rightLength: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDivisor", (left, leftLength, right, rightLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSelf(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        leftLength: i32,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        rightLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddSelf", (left, leftLength, right, rightLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_Il2CppArray_Il2CppArray1(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Add", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_Il2CppArray_u32_0(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        right: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Add", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_Il2CppObject_i32_Il2CppObject_i32_Il2CppObject_i32_2(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        leftLength: i32,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        rightLength: i32,
        bits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitsLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (left, leftLength, right, rightLength, bits, bitsLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCopy(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateCopy", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn DivideGuessTooBig(
        q: u64,
        valHi: u64,
        valLo: u32,
        divHi: u32,
        divLo: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DivideGuessTooBig", (q, valHi, valLo, divHi, divLo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Divide_Il2CppArray_Il2CppArray1(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Divide", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Divide_Il2CppArray_u32_0(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        right: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Divide", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Divide_Il2CppObject_i32_Il2CppObject_i32_Il2CppObject_i32_2(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        leftLength: i32,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        rightLength: i32,
        bits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitsLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Divide", (left, leftLength, right, rightLength, bits, bitsLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn LeadingZeros(value: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LeadingZeros", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Multiply_Il2CppArray_Il2CppArray1(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Multiply", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Multiply_Il2CppArray_u32_0(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        right: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Multiply", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Multiply_Il2CppObject_i32_Il2CppObject_i32_Il2CppObject_i32_2(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        leftLength: i32,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        rightLength: i32,
        bits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitsLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Multiply",
                (left, leftLength, right, rightLength, bits, bitsLength),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Remainder_Il2CppArray1(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Remainder", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remainder_u32_0(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        right: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Remainder", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Square_Il2CppArray0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Square", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Square_Il2CppObject_i32_Il2CppObject_i32_1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        valueLength: i32,
        bits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitsLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Square", (value, valueLength, bits, bitsLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubtractCore(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        leftLength: i32,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        rightLength: i32,
        core: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        coreLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SubtractCore",
                (left, leftLength, right, rightLength, core, coreLength),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SubtractDivisor(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        leftLength: i32,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        rightLength: i32,
        q: u64,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SubtractDivisor", (left, leftLength, right, rightLength, q))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract_Il2CppArray_Il2CppArray1(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Subtract", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract_Il2CppArray_u32_0(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        right: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Subtract", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract_Il2CppObject_i32_Il2CppObject_i32_Il2CppObject_i32_2(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        leftLength: i32,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        rightLength: i32,
        bits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bitsLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Subtract",
                (left, leftLength, right, rightLength, bits, bitsLength),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Numerics::BigIntegerCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
