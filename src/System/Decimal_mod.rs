#[cfg(feature = "System+Decimal+DecCalc+Buf12")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DecCalc_Decimal_Buf12 {
    padding: [u8; 16usize],
}
#[cfg(feature = "System+Decimal+DecCalc+Buf12")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DecCalc_Decimal_Buf12 => "System"
    ."Decimal/DecCalc/Buf12"
);
#[cfg(feature = "System+Decimal+DecCalc+Buf12")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::DecCalc_Decimal_Buf12 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Decimal+DecCalc+Buf12")]
impl crate::System::DecCalc_Decimal_Buf12 {
    pub fn get_High64(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_High64",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Low64(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Low64",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_High64(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_High64",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Low64(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Low64",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Decimal+DecCalc+Buf16")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DecCalc_Decimal_Buf16 {
    padding: [u8; 16usize],
}
#[cfg(feature = "System+Decimal+DecCalc+Buf16")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DecCalc_Decimal_Buf16 => "System"
    ."Decimal/DecCalc/Buf16"
);
#[cfg(feature = "System+Decimal+DecCalc+Buf16")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::DecCalc_Decimal_Buf16 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Decimal+DecCalc+Buf16")]
impl crate::System::DecCalc_Decimal_Buf16 {
    pub fn get_High64(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_High64",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Low64(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Low64",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_High64(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_High64",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Low64(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Low64",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Decimal+DecCalc+Buf24")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DecCalc_Decimal_Buf24 {
    padding: [u8; 24usize],
}
#[cfg(feature = "System+Decimal+DecCalc+Buf24")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DecCalc_Decimal_Buf24 => "System"
    ."Decimal/DecCalc/Buf24"
);
#[cfg(feature = "System+Decimal+DecCalc+Buf24")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::DecCalc_Decimal_Buf24 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Decimal+DecCalc+Buf24")]
impl crate::System::DecCalc_Decimal_Buf24 {
    pub fn get_Low64(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Low64",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_High64(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_High64",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Low64(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Low64",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Mid64(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Mid64",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Decimal+DecCalc+PowerOvfl")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DecCalc_Decimal_PowerOvfl {
    pub Hi: u32,
    pub MidLo: u64,
}
#[cfg(feature = "System+Decimal+DecCalc+PowerOvfl")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DecCalc_Decimal_PowerOvfl => "System"
    ."Decimal/DecCalc/PowerOvfl"
);
#[cfg(feature = "System+Decimal+DecCalc+PowerOvfl")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::DecCalc_Decimal_PowerOvfl {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Decimal+DecCalc+PowerOvfl")]
impl crate::System::DecCalc_Decimal_PowerOvfl {
    pub fn _ctor(
        &mut self,
        hi: u32,
        mid: u32,
        lo: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (hi, mid, lo),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Decimal+DecCalc+RoundingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DecCalc_Decimal_RoundingMode {
    #[default]
    AwayFromZero = 1i32,
    Ceiling = 4i32,
    Floor = 3i32,
    ToEven = 0i32,
    Truncate = 2i32,
}
#[cfg(feature = "System+Decimal+DecCalc+RoundingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DecCalc_Decimal_RoundingMode => "System"
    ."Decimal/DecCalc/RoundingMode"
);
#[cfg(feature = "System+Decimal")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Decimal {
    padding: [u8; 16usize],
}
#[cfg(feature = "System+Decimal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Decimal => "System"."Decimal"
);
#[cfg(feature = "System+Decimal")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Decimal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Decimal")]
impl crate::System::Decimal {
    pub const ScaleMask: i32 = 16711680i32;
    pub const ScaleShift: i32 = 16i32;
    pub const SignMask: i32 = -2147483648i32;
    #[cfg(feature = "System+Decimal+DecCalc")]
    pub type DecCalc = crate::System::Decimal_DecCalc;
    pub fn Abs(
        d: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Abs", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn AsMutable(
        d: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::System::Decimal_DecCalc,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("AsMutable", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Decimal1(
        &mut self,
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Gc0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn DecDivMod1E9(
        value: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecDivMod1E9", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Divide(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Divide", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Decimal1(
        &mut self,
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Gc0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBits(
        d: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetBits", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        let __cordl_ret: crate::System::TypeCode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTypeCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValid(flags: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValid", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max(
        d1: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
        d2: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min(
        d1: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
        d2: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Multiply(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Multiply", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Negate(
        d: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Negate", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Gc0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (s, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_NumberStyles_Gc1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        style: crate::System::Globalization::NumberStyles,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (s, style, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn Round_ByRefMut_MidpointRounding1(
        d: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
        decimals: i32,
        mode: crate::System::MidpointRounding,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Round", (d, decimals, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Round_Decimal0(
        d: crate::System::Decimal,
        decimals: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Round", (d, decimals))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToBoolean(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToBoolean",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToByte(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToByte",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToChar(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToChar",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDateTime(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToDateTime",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDecimal(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToDecimal",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDouble(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToDouble",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt16(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToInt16",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt32(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToInt32",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt64(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToInt64",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToSByte(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToSByte",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToSingle(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToSingle",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToType",
            (_cordl_type, provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt16(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToUInt16",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt32(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToUInt32",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt64(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToUInt64",
            (provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_IDeserializationCallback_OnDeserialization(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
            (sender),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByte(value: crate::System::Decimal) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDouble(d: crate::System::Decimal) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToDouble", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt16(value: crate::System::Decimal) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt32(d: crate::System::Decimal) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToInt64(d: crate::System::Decimal) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt64", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSByte(value: crate::System::Decimal) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSingle(d: crate::System::Decimal) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToSingle", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Gc1(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Gc_Gc2(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt16(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt16", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt32(d: crate::System::Decimal) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt32", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToUInt64(d: crate::System::Decimal) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToUInt64", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Truncate_ByRefMut1(
        d: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Truncate", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Truncate_Decimal0(
        d: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Truncate", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryFormat",
            (destination, charsWritten, format, provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        style: crate::System::Globalization::NumberStyles,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (s, style, provider, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_i32_8(
        &mut self,
        d: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
        flags: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (d, flags),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc6(
        &mut self,
        bits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (bits),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_4(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_5(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32__cordl_bool_u8_7(
        &mut self,
        lo: i32,
        mid: i32,
        hi: i32,
        isNegative: bool,
        scale: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (lo, mid, hi, isNegative, scale),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_2(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_1(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u64_3(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_High(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_High",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNegative(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNegative",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Low(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Low",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Low64(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Low64",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Mid(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Mid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Scale(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Scale",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_Decimal2(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_Decimal3(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_Decimal4(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_Decimal5(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_Decimal6(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f32_0(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_1(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit__cordl_char4(
        value: char,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i16_2(
        value: i16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_5(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i64_7(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i8_1(
        value: i8,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u16_3(
        value: u16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_6(
        value: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u64_8(
        value: u64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u8_0(
        value: u8,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        d: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Increment", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        d1: crate::System::Decimal,
        d2: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        d: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (d))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Decimal>>
for crate::System::Decimal {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::Decimal> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Decimal>>
for crate::System::Decimal {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::Decimal> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Decimal>>
for crate::System::Decimal {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::Decimal> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Decimal>>
for crate::System::Decimal {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::Decimal> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IComparable>>
for crate::System::Decimal {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IComparable> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IComparable>>
for crate::System::Decimal {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IComparable> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IConvertible>>
for crate::System::Decimal {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IConvertible> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IConvertible>>
for crate::System::Decimal {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IConvertible> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::System::Decimal {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::System::Decimal {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::ISpanFormattable>>
for crate::System::Decimal {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::ISpanFormattable> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::ISpanFormattable>>
for crate::System::Decimal {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::ISpanFormattable> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::IDeserializationCallback,
    >,
> for crate::System::Decimal {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::IDeserializationCallback,
    > {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::IDeserializationCallback,
    >,
> for crate::System::Decimal {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::IDeserializationCallback,
    > {
        todo!()
    }
}
#[cfg(feature = "System+Decimal+DecCalc")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Decimal_DecCalc {
    padding: [u8; 16usize],
}
#[cfg(feature = "System+Decimal+DecCalc")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Decimal_DecCalc => "System"
    ."Decimal/DecCalc"
);
#[cfg(feature = "System+Decimal+DecCalc")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Decimal_DecCalc {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Decimal+DecCalc")]
impl crate::System::Decimal_DecCalc {
    #[cfg(feature = "System+Decimal+DecCalc+Buf12")]
    pub type Buf12 = crate::System::DecCalc_Decimal_Buf12;
    #[cfg(feature = "System+Decimal+DecCalc+Buf16")]
    pub type Buf16 = crate::System::DecCalc_Decimal_Buf16;
    #[cfg(feature = "System+Decimal+DecCalc+Buf24")]
    pub type Buf24 = crate::System::DecCalc_Decimal_Buf24;
    #[cfg(feature = "System+Decimal+DecCalc+PowerOvfl")]
    pub type PowerOvfl = crate::System::DecCalc_Decimal_PowerOvfl;
    #[cfg(feature = "System+Decimal+DecCalc+RoundingMode")]
    pub type RoundingMode = crate::System::DecCalc_Decimal_RoundingMode;
    pub fn Add32To96(
        bufNum: quest_hook::libil2cpp::ByRefMut<crate::System::DecCalc_Decimal_Buf12>,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add32To96", (bufNum, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecAddSub(
        d1: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
        d2: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
        sign: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecAddSub", (d1, d2, sign))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecDivMod1E9(
        value: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DecDivMod1E9", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Div128By96(
        bufNum: quest_hook::libil2cpp::ByRefMut<crate::System::DecCalc_Decimal_Buf16>,
        bufDen: quest_hook::libil2cpp::ByRefMut<crate::System::DecCalc_Decimal_Buf12>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Div128By96", (bufNum, bufDen))?;
        Ok(__cordl_ret.into())
    }
    pub fn Div96By32(
        bufNum: quest_hook::libil2cpp::ByRefMut<crate::System::DecCalc_Decimal_Buf12>,
        den: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Div96By32", (bufNum, den))?;
        Ok(__cordl_ret.into())
    }
    pub fn Div96By64(
        bufNum: quest_hook::libil2cpp::ByRefMut<crate::System::DecCalc_Decimal_Buf12>,
        den: u64,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Div96By64", (bufNum, den))?;
        Ok(__cordl_ret.into())
    }
    pub fn Div96ByConst(
        high64: quest_hook::libil2cpp::ByRefMut<u64>,
        low: quest_hook::libil2cpp::ByRefMut<u32>,
        pow: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Div96ByConst", (high64, low, pow))?;
        Ok(__cordl_ret.into())
    }
    pub fn DivByConst(
        result: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        hiRes: u32,
        quotient: quest_hook::libil2cpp::ByRefMut<u32>,
        remainder: quest_hook::libil2cpp::ByRefMut<u32>,
        power: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DivByConst", (result, hiRes, quotient, remainder, power))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExponent_f32_0(f: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExponent", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExponent_f64_1(d: f64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExponent", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        d: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn IncreaseScale(
        bufNum: quest_hook::libil2cpp::ByRefMut<crate::System::DecCalc_Decimal_Buf12>,
        power: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IncreaseScale", (bufNum, power))?;
        Ok(__cordl_ret.into())
    }
    pub fn IncreaseScale64(
        bufNum: quest_hook::libil2cpp::ByRefMut<crate::System::DecCalc_Decimal_Buf12>,
        power: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IncreaseScale64", (bufNum, power))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalRound(
        d: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
        scale: u32,
        mode: crate::System::DecCalc_Decimal_RoundingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalRound", (d, scale, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LeadingZeroCount(value: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LeadingZeroCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverflowUnscale(
        bufQuo: quest_hook::libil2cpp::ByRefMut<crate::System::DecCalc_Decimal_Buf12>,
        scale: i32,
        sticky: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverflowUnscale", (bufQuo, scale, sticky))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScaleResult(
        bufRes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        hiRes: u32,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScaleResult", (bufRes, hiRes, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchScale(
        bufQuo: quest_hook::libil2cpp::ByRefMut<crate::System::DecCalc_Decimal_Buf12>,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SearchScale", (bufQuo, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32x32To64(a: u32, b: u32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32x32To64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt64x64To128(
        a: u64,
        b: u64,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt64x64To128", (a, b, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unscale(
        low: quest_hook::libil2cpp::ByRefMut<u32>,
        high64: quest_hook::libil2cpp::ByRefMut<u64>,
        scale: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unscale", (low, high64, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn VarDecCmp(
        d1: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
        d2: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VarDecCmp", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn VarDecCmpSub(
        d1: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
        d2: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VarDecCmpSub", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn VarDecDiv(
        d1: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
        d2: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VarDecDiv", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn VarDecFromR4(
        input: f32,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VarDecFromR4", (input, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn VarDecFromR8(
        input: f64,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VarDecFromR8", (input, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn VarDecMul(
        d1: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
        d2: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal_DecCalc>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VarDecMul", (d1, d2))?;
        Ok(__cordl_ret.into())
    }
    pub fn VarR4FromDec(
        value: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VarR4FromDec", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn VarR8FromDec(
        value: quest_hook::libil2cpp::ByRefMut<crate::System::Decimal>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VarR8FromDec", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_High(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_High",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNegative(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsNegative",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Low(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Low",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Low64(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Low64",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Mid(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Mid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_High(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_High",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Low(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Low",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Low64(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Low64",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Mid(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Mid",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
