#[cfg(feature = "System+Math")]
#[repr(C)]
#[derive(Debug)]
pub struct Math {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Math")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Math => "System"."Math"
);
#[cfg(feature = "System+Math")]
impl std::ops::Deref for crate::System::Math {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Math")]
impl std::ops::DerefMut for crate::System::Math {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Math")]
impl crate::System::Math {
    pub fn Abs_Decimal2(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Abs", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Abs_f32_4(value: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Abs", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Abs_f64_3(value: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Abs", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Abs_i32_0(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Abs", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Abs_i64_1(value: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Abs", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Acos(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Acos", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Asin(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Asin", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Atan(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Atan", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Atan2(y: f64, x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Atan2", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Ceiling(a: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Ceiling", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clamp(value: i32, min: i32, max: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clamp", (value, min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cos(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cos", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cosh(value: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cosh", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn DivRem(
        a: i32,
        b: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DivRem", (a, b, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exp(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exp", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Floor(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Floor", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log10(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log10", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log_f64_0(a: f64, newBase: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (a, newBase))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log_f64_1(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_Decimal_Decimal1(
        val1: crate::System::Decimal,
        val2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_f32_f32_7(val1: f32, val2: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_f64_f64_2(val1: f64, val2: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_i16_i16_3(val1: i16, val2: i16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_i32_i32_4(val1: i32, val2: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_i64_i64_5(val1: i64, val2: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_i8_i8_6(val1: i8, val2: i8) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_u16_u16_8(val1: u16, val2: u16) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_u32_u32_9(val1: u32, val2: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_u64_u64_10(val1: u64, val2: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_u8_u8_0(val1: u8, val2: u8) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_Decimal_Decimal1(
        val1: crate::System::Decimal,
        val2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_f32_f32_7(val1: f32, val2: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_f64_f64_2(val1: f64, val2: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_i16_i16_3(val1: i16, val2: i16) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_i32_i32_4(val1: i32, val2: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_i64_i64_5(val1: i64, val2: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_i8_i8_6(val1: i8, val2: i8) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_u16_u16_8(val1: u16, val2: u16) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_u32_u32_9(val1: u32, val2: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_u64_u64_10(val1: u64, val2: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_u8_u8_0(val1: u8, val2: u8) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn ModF(
        x: f64,
        intptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ModF", (x, intptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn Pow(x: f64, y: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Pow", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn Round_Decimal0(
        d: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Round", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Round_f64_1(a: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Round", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn Round_f64_MidpointRounding3(
        value: f64,
        mode: crate::System::MidpointRounding,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Round", (value, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Round_f64_i32_2(
        value: f64,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Round", (value, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn Round_f64_i32_MidpointRounding4(
        value: f64,
        digits: i32,
        mode: crate::System::MidpointRounding,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Round", (value, digits, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sign_f64_0(value: f64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sign", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sign_i64_1(value: i64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sign", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sin(a: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sin", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sinh(value: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sinh", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sqrt(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sqrt", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Tan(a: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Tan", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn Tanh(value: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Tanh", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowAbsOverflow() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowAbsOverflow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowMinMaxException<T>(
        min: T,
        max: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowMinMaxException", (min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn Truncate_Decimal0(
        d: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Truncate", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Truncate_f64_1(d: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Truncate", (d))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Math")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Math {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
