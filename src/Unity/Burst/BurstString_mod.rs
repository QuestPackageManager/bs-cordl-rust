#[cfg(feature = "Unity+Burst+BurstString")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstString {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+BurstString")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString => "Unity.Burst"
    ."BurstString"
);
#[cfg(feature = "Unity+Burst+BurstString")]
impl std::ops::Deref for crate::Unity::Burst::BurstString {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstString")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstString")]
impl crate::Unity::Burst::BurstString {
    pub const DoubleNumberBufferLength: i32 = 18i32;
    pub const DoublePrecision: i32 = 17i32;
    pub const DoublePrecisionCustomFormat: i32 = 15i32;
    pub const SingleNumberBufferLength: i32 = 10i32;
    pub const SinglePrecision: i32 = 9i32;
    pub const SinglePrecisionCustomFormat: i32 = 7i32;
    #[cfg(feature = "Unity+Burst+BurstString+CutoffMode")]
    pub type CutoffMode = crate::Unity::Burst::BurstString_CutoffMode;
    #[cfg(feature = "Unity+Burst+BurstString+FormatOptions")]
    pub type FormatOptions = crate::Unity::Burst::BurstString_FormatOptions;
    #[cfg(feature = "Unity+Burst+BurstString+NumberBuffer")]
    pub type NumberBuffer = crate::Unity::Burst::BurstString_NumberBuffer;
    #[cfg(feature = "Unity+Burst+BurstString+NumberBufferKind")]
    pub type NumberBufferKind = crate::Unity::Burst::BurstString_NumberBufferKind;
    #[cfg(feature = "Unity+Burst+BurstString+NumberFormatKind")]
    pub type NumberFormatKind = crate::Unity::Burst::BurstString_NumberFormatKind;
    #[cfg(feature = "Unity+Burst+BurstString+PreserveAttribute")]
    pub type PreserveAttribute = crate::Unity::Burst::BurstString_PreserveAttribute;
    #[cfg(feature = "Unity+Burst+BurstString+PrintFloatFormat")]
    pub type PrintFloatFormat = crate::Unity::Burst::BurstString_PrintFloatFormat;
    #[cfg(feature = "Unity+Burst+BurstString+tBigInt")]
    pub type tBigInt = crate::Unity::Burst::BurstString_tBigInt;
    #[cfg(feature = "Unity+Burst+BurstString+tFloatUnion32")]
    pub type tFloatUnion32 = crate::Unity::Burst::BurstString_tFloatUnion32;
    #[cfg(feature = "Unity+Burst+BurstString+tFloatUnion64")]
    pub type tFloatUnion64 = crate::Unity::Burst::BurstString_tFloatUnion64;
    pub fn AlignLeft(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        align: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlignLeft", (dest, destIndex, destLength, align, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn AlignRight(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        align: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlignRight", (dest, destIndex, destLength, align, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_Add(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        lhs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::BurstString_tBigInt>,
        rhs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::BurstString_tBigInt>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_Add", (pResult, lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_Add_internal(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        pLarge: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        pSmall: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::BurstString_tBigInt>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_Add_internal", (pResult, pLarge, pSmall))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_Compare(
        lhs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::BurstString_tBigInt>,
        rhs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::BurstString_tBigInt>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_Compare", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_DivideWithRemainder_MaxQuotient9(
        pDividend: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        divisor: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_DivideWithRemainder_MaxQuotient9", (pDividend, divisor))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_Multiply10(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_Multiply10", (pResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_Multiply2_ByRefMut0(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        input: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::BurstString_tBigInt>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_Multiply2", (pResult, input))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_Multiply2_ByRefMut1(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_Multiply2", (pResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_MultiplyPow10(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        input: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::BurstString_tBigInt>,
        exponent: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_MultiplyPow10", (pResult, input, exponent))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_Multiply_ByRefMut0(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        lhs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::BurstString_tBigInt>,
        rhs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::BurstString_tBigInt>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_Multiply", (pResult, lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_Multiply_internal(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        pLarge: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        pSmall: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::BurstString_tBigInt>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_Multiply_internal", (pResult, pLarge, pSmall))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_Multiply_u32_1(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        lhs: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::BurstString_tBigInt>,
        rhs: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_Multiply", (pResult, lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_Pow10(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        exponent: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_Pow10", (pResult, exponent))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_Pow2(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        exponent: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_Pow2", (pResult, exponent))?;
        Ok(__cordl_ret.into())
    }
    pub fn BigInt_ShiftLeft(
        pResult: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_tBigInt,
        >,
        shift: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BigInt_ShiftLeft", (pResult, shift))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertDoubleToString(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: f64,
        formatOptions: crate::Unity::Burst::BurstString_FormatOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertDoubleToString",
                (dest, destIndex, destLength, value, formatOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertFloatToString(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: f32,
        formatOptions: crate::Unity::Burst::BurstString_FormatOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertFloatToString",
                (dest, destIndex, destLength, value, formatOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertIntegerToString(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: i64,
        options: crate::Unity::Burst::BurstString_FormatOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertIntegerToString",
                (dest, destIndex, destLength, value, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertUnsignedIntegerToString(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: u64,
        options: crate::Unity::Burst::BurstString_FormatOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertUnsignedIntegerToString",
                (dest, destIndex, destLength, value, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFixedString(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destLength: i32,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyFixedString", (dest, destLength, src, srcLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dragon4(
        mantissa: u64,
        exponent: i32,
        mantissaHighBitIdx: u32,
        hasUnequalMargins: bool,
        cutoffMode: crate::Unity::Burst::BurstString_CutoffMode,
        cutoffNumber: u32,
        pOutBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: u32,
        pOutExponent: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Dragon4",
                (
                    mantissa,
                    exponent,
                    mantissaHighBitIdx,
                    hasUnequalMargins,
                    cutoffMode,
                    cutoffNumber,
                    pOutBuffer,
                    bufferSize,
                    pOutExponent,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatDecimalOrHexadecimal(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_NumberBuffer,
        >,
        zeroPadding: i32,
        outputPositiveSign: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatDecimalOrHexadecimal",
                (dest, destIndex, destLength, number, zeroPadding, outputPositiveSign),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatGeneral(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_NumberBuffer,
        >,
        nMaxDigits: i32,
        expChar: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatGeneral",
                (dest, destIndex, destLength, number, nMaxDigits, expChar),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatInfinityNaN(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        mantissa: u64,
        isNegative: bool,
        formatOptions: crate::Unity::Burst::BurstString_FormatOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatInfinityNaN",
                (dest, destIndex, destLength, mantissa, isNegative, formatOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatNumber(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        number: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_NumberBuffer,
        >,
        nMaxDigits: i32,
        options: crate::Unity::Burst::BurstString_FormatOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatNumber",
                (dest, destIndex, destLength, number, nMaxDigits, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatPositional(
        pOutBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: u32,
        mantissa: u64,
        exponent: i32,
        mantissaHighBitIdx: u32,
        hasUnequalMargins: bool,
        precision: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatPositional",
                (
                    pOutBuffer,
                    bufferSize,
                    mantissa,
                    exponent,
                    mantissaHighBitIdx,
                    hasUnequalMargins,
                    precision,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatScientific(
        pOutBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: u32,
        mantissa: u64,
        exponent: i32,
        mantissaHighBitIdx: u32,
        hasUnequalMargins: bool,
        precision: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FormatScientific",
                (
                    pOutBuffer,
                    bufferSize,
                    mantissa,
                    exponent,
                    mantissaHighBitIdx,
                    hasUnequalMargins,
                    precision,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_Il2CppObject_i32_0(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        srcLength: i32,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Format",
                (dest, destIndex, destLength, src, srcLength, formatOptionsRaw),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Format__cordl_bool3(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: bool,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format__cordl_char4(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: char,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_f32_1(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: f32,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_f64_2(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: f64,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_i16_10(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: i16,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_i32_11(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: i32,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_i64_12(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: i64,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_i8_9(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: i8,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_u16_6(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: u16,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_u32_7(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: u32,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_u64_8(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: u64,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn Format_u8_5(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        destIndex: quest_hook::libil2cpp::ByRefMut<i32>,
        destLength: i32,
        value: u8,
        formatOptionsRaw: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Format", (dest, destIndex, destLength, value, formatOptionsRaw))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLengthForFormatGeneral(
        number: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_NumberBuffer,
        >,
        nMaxDigits: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLengthForFormatGeneral", (number, nMaxDigits))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLengthIntegerToString(
        value: i64,
        basis: i32,
        zeroPadding: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLengthIntegerToString", (value, basis, zeroPadding))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogBase2(val: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogBase2", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn OptsSplit(
        fullFormat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        padding: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        format: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OptsSplit", (fullFormat, padding, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseFormatToFormatOptions(
        fullFormat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::BurstString_FormatOptions> {
        let __cordl_ret: crate::Unity::Burst::BurstString_FormatOptions = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseFormatToFormatOptions", (fullFormat))?;
        Ok(__cordl_ret.into())
    }
    pub fn RoundNumber(
        number: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Burst::BurstString_NumberBuffer,
        >,
        pos: i32,
        isCorrectlyRounded: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RoundNumber", (number, pos, isCorrectlyRounded))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldRoundUp(
        dig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        i: i32,
        isCorrectlyRounded: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldRoundUp", (dig, i, isCorrectlyRounded))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValueToIntegerChar(
        value: i32,
        uppercase: bool,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValueToIntegerChar", (value, uppercase))?;
        Ok(__cordl_ret.into())
    }
    pub fn g_PowerOf10_Big(
        i: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::BurstString_tBigInt> {
        let __cordl_ret: crate::Unity::Burst::BurstString_tBigInt = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("g_PowerOf10_Big", (i))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstString")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::BurstString {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstString+CutoffMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BurstString_CutoffMode {
    FractionLength = 2i32,
    TotalLength = 1i32,
    Unique = 0i32,
}
#[cfg(feature = "Unity+Burst+BurstString+CutoffMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString_CutoffMode =>
    "Unity.Burst"."BurstString/CutoffMode"
);
#[cfg(feature = "Unity+Burst+BurstString+FormatOptions")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BurstString_FormatOptions {
    pub Kind: crate::Unity::Burst::BurstString_NumberFormatKind,
    pub AlignAndSize: i8,
    pub Specifier: u8,
    pub Lowercase: bool,
}
#[cfg(feature = "Unity+Burst+BurstString+FormatOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString_FormatOptions =>
    "Unity.Burst"."BurstString/FormatOptions"
);
#[cfg(feature = "Unity+Burst+BurstString+FormatOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::BurstString_FormatOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+BurstString+FormatOptions")]
impl crate::Unity::Burst::BurstString_FormatOptions {
    pub fn EncodeToRaw(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "EncodeToRaw",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBase(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBase",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        kind: crate::Unity::Burst::BurstString_NumberFormatKind,
        alignAndSize: i8,
        specifier: u8,
        lowercase: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (kind, alignAndSize, specifier, lowercase),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Uppercase(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Uppercase",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstString+NumberBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BurstString_NumberBuffer {
    pub _buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Kind: crate::Unity::Burst::BurstString_NumberBufferKind,
    pub DigitsCount: i32,
    pub Scale: i32,
    pub IsNegative: bool,
}
#[cfg(feature = "Unity+Burst+BurstString+NumberBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString_NumberBuffer =>
    "Unity.Burst"."BurstString/NumberBuffer"
);
#[cfg(feature = "Unity+Burst+BurstString+NumberBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::BurstString_NumberBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+BurstString+NumberBuffer")]
impl crate::Unity::Burst::BurstString_NumberBuffer {
    pub fn GetDigitsPointer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetDigitsPointer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        kind: crate::Unity::Burst::BurstString_NumberBufferKind,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        digitsCount: i32,
        scale: i32,
        isNegative: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (kind, buffer, digitsCount, scale, isNegative),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstString+NumberBufferKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BurstString_NumberBufferKind {
    Float = 1i32,
    Integer = 0i32,
}
#[cfg(feature = "Unity+Burst+BurstString+NumberBufferKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString_NumberBufferKind =>
    "Unity.Burst"."BurstString/NumberBufferKind"
);
#[cfg(feature = "Unity+Burst+BurstString+NumberFormatKind")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BurstString_NumberFormatKind {
    Decimal = 1u8,
    DecimalForceSigned = 2u8,
    General = 0u8,
    Hexadecimal = 3u8,
}
#[cfg(feature = "Unity+Burst+BurstString+NumberFormatKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString_NumberFormatKind =>
    "Unity.Burst"."BurstString/NumberFormatKind"
);
#[cfg(feature = "Unity+Burst+BurstString+PreserveAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstString_PreserveAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "Unity+Burst+BurstString+PreserveAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString_PreserveAttribute =>
    "Unity.Burst"."BurstString/PreserveAttribute"
);
#[cfg(feature = "Unity+Burst+BurstString+PreserveAttribute")]
impl std::ops::Deref for crate::Unity::Burst::BurstString_PreserveAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstString+PreserveAttribute")]
impl std::ops::DerefMut for crate::Unity::Burst::BurstString_PreserveAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+BurstString+PreserveAttribute")]
impl crate::Unity::Burst::BurstString_PreserveAttribute {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstString+PreserveAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::BurstString_PreserveAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+BurstString+PrintFloatFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BurstString_PrintFloatFormat {
    Positional = 0i32,
    Scientific = 1i32,
}
#[cfg(feature = "Unity+Burst+BurstString+PrintFloatFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString_PrintFloatFormat =>
    "Unity.Burst"."BurstString/PrintFloatFormat"
);
#[cfg(feature = "Unity+Burst+BurstString+tBigInt")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BurstString_tBigInt {
    pub m_length: i32,
    pub m_blocks: crate::Unity::Burst::tBigInt_BurstString__m_blocks_e__FixedBuffer,
}
#[cfg(feature = "Unity+Burst+BurstString+tBigInt")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString_tBigInt =>
    "Unity.Burst"."BurstString/tBigInt"
);
#[cfg(feature = "Unity+Burst+BurstString+tBigInt")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::BurstString_tBigInt {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+BurstString+tBigInt")]
impl crate::Unity::Burst::BurstString_tBigInt {
    pub const c_BigInt_MaxBlocks: i32 = 35i32;
    #[cfg(feature = "Unity+Burst+BurstString+tBigInt+_m_blocks_e__FixedBuffer")]
    pub type _m_blocks_e__FixedBuffer = crate::Unity::Burst::tBigInt_BurstString__m_blocks_e__FixedBuffer;
    pub fn GetBlock(&mut self, idx: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBlock",
            (idx),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLength",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetU32(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetU32",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsZero(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsZero",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetU32(
        &mut self,
        val: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetU32",
            (val),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetU64(
        &mut self,
        val: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetU64",
            (val),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetZero(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetZero",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstString+tFloatUnion32")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BurstString_tFloatUnion32 {
    padding: [u8; 4usize],
}
#[cfg(feature = "Unity+Burst+BurstString+tFloatUnion32")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString_tFloatUnion32 =>
    "Unity.Burst"."BurstString/tFloatUnion32"
);
#[cfg(feature = "Unity+Burst+BurstString+tFloatUnion32")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::BurstString_tFloatUnion32 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+BurstString+tFloatUnion32")]
impl crate::Unity::Burst::BurstString_tFloatUnion32 {
    pub fn GetExponent(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetExponent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMantissa(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetMantissa",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNegative(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNegative",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstString+tFloatUnion64")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BurstString_tFloatUnion64 {
    padding: [u8; 8usize],
}
#[cfg(feature = "Unity+Burst+BurstString+tFloatUnion64")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString_tFloatUnion64 =>
    "Unity.Burst"."BurstString/tFloatUnion64"
);
#[cfg(feature = "Unity+Burst+BurstString+tFloatUnion64")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::BurstString_tFloatUnion64 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+BurstString+tFloatUnion64")]
impl crate::Unity::Burst::BurstString_tFloatUnion64 {
    pub fn GetExponent(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetExponent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMantissa(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetMantissa",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNegative(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNegative",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+BurstString+tBigInt+_m_blocks_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct tBigInt_BurstString__m_blocks_e__FixedBuffer {
    pub FixedElementField: u32,
}
#[cfg(feature = "Unity+Burst+BurstString+tBigInt+_m_blocks_e__FixedBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::tBigInt_BurstString__m_blocks_e__FixedBuffer => "Unity.Burst"
    ."BurstString/tBigInt/<m_blocks>e__FixedBuffer"
);
#[cfg(feature = "Unity+Burst+BurstString+tBigInt+_m_blocks_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::tBigInt_BurstString__m_blocks_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+BurstString+tBigInt+_m_blocks_e__FixedBuffer")]
impl crate::Unity::Burst::tBigInt_BurstString__m_blocks_e__FixedBuffer {}
