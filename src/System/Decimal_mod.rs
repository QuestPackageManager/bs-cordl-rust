#[cfg(feature = "System+Decimal+DecCalc+Buf12")]
#[repr(C)]
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecCalc_Decimal_RoundingMode {
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
#[derive(Debug, Clone)]
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
    pub fn CompareTo_Il2CppObject0(
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
    pub fn Equals_Il2CppObject0(
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
    pub fn ToString_IFormatProvider1(
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
    pub fn ToString_Il2CppString_IFormatProvider2(
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
    pub fn _ctor_Il2CppArray6(
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
}
#[cfg(feature = "System+Decimal")]
impl AsRef<crate::System::IComparable> for crate::System::Decimal {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<crate::System::IComparable> for crate::System::Decimal {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<crate::System::IComparable_1<crate::System::Decimal>>
for crate::System::Decimal {
    fn as_ref(&self) -> &crate::System::IComparable_1<crate::System::Decimal> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<crate::System::IComparable_1<crate::System::Decimal>>
for crate::System::Decimal {
    fn as_mut(&mut self) -> &mut crate::System::IComparable_1<crate::System::Decimal> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<crate::System::IConvertible> for crate::System::Decimal {
    fn as_ref(&self) -> &crate::System::IConvertible {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<crate::System::IConvertible> for crate::System::Decimal {
    fn as_mut(&mut self) -> &mut crate::System::IConvertible {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<crate::System::IEquatable_1<crate::System::Decimal>>
for crate::System::Decimal {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::System::Decimal> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<crate::System::IEquatable_1<crate::System::Decimal>>
for crate::System::Decimal {
    fn as_mut(&mut self) -> &mut crate::System::IEquatable_1<crate::System::Decimal> {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<crate::System::IFormattable> for crate::System::Decimal {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<crate::System::IFormattable> for crate::System::Decimal {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<crate::System::ISpanFormattable> for crate::System::Decimal {
    fn as_ref(&self) -> &crate::System::ISpanFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<crate::System::ISpanFormattable> for crate::System::Decimal {
    fn as_mut(&mut self) -> &mut crate::System::ISpanFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsRef<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Decimal {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Serialization::IDeserializationCallback {
        todo!()
    }
}
#[cfg(feature = "System+Decimal")]
impl AsMut<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Decimal {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IDeserializationCallback {
        todo!()
    }
}
#[cfg(feature = "System+Decimal+DecCalc")]
#[repr(C)]
#[derive(Debug, Clone)]
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
