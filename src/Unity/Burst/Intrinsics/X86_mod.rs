#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx+CMP")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Avx_X86_CMP {
    EQ_OQ = 0i32,
    EQ_OS = 16i32,
    EQ_UQ = 8i32,
    EQ_US = 24i32,
    FALSE_OQ = 11i32,
    FALSE_OS = 27i32,
    GE_OQ = 29i32,
    GE_OS = 13i32,
    GT_OQ = 30i32,
    GT_OS = 14i32,
    LE_OQ = 18i32,
    LE_OS = 2i32,
    LT_OQ = 17i32,
    LT_OS = 1i32,
    NEQ_OQ = 12i32,
    NEQ_OS = 28i32,
    NEQ_UQ = 4i32,
    NEQ_US = 20i32,
    NGE_UQ = 25i32,
    NGE_US = 9i32,
    NGT_UQ = 26i32,
    NGT_US = 10i32,
    NLE_UQ = 22i32,
    NLE_US = 6i32,
    NLT_UQ = 21i32,
    NLT_US = 5i32,
    ORD_Q = 7i32,
    ORD_S = 23i32,
    TRUE_UQ = 15i32,
    TRUE_US = 31i32,
    UNORD_Q = 3i32,
    UNORD_S = 19i32,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx+CMP")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::Avx_X86_CMP =>
    "Unity.Burst.Intrinsics"."X86/Avx/CMP"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Fma_X86_Union {
    padding: [u8; 4usize],
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::Fma_X86_Union =>
    "Unity.Burst.Intrinsics"."X86/Fma/Union"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::Intrinsics::Fma_X86_Union {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
impl crate::Unity::Burst::Intrinsics::Fma_X86_Union {}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+SIDD")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sse4_2_X86_SIDD {
    BIT_MASK = 0i32,
    CMP_EQUAL_EACH = 8i32,
    CMP_EQUAL_ORDERED = 12i32,
    CMP_RANGES = 4i32,
    MASKED_NEGATIVE_POLARITY = 48i32,
    MASKED_POSITIVE_POLARITY = 32i32,
    MOST_SIGNIFICANT = 64i32,
    NEGATIVE_POLARITY = 16i32,
    SBYTE_OPS = 2i32,
    SWORD_OPS = 3i32,
    UWORD_OPS = 1i32,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+SIDD")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::Sse4_2_X86_SIDD =>
    "Unity.Burst.Intrinsics"."X86/Sse4_2/SIDD"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Sse4_2_X86_StrBoolArray {
    pub Bits: crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray => "Unity.Burst.Intrinsics"
    ."X86/Sse4_2/StrBoolArray"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
impl crate::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray {
    #[cfg(
        feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer"
    )]
    pub type _Bits_e__FixedBuffer = crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer;
    pub fn GetBit(
        &mut self,
        aindex: i32,
        bindex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBit",
            (aindex, bindex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBit(
        &mut self,
        aindex: i32,
        bindex: i32,
        val: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetBit",
            (aindex, bindex, val),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer {
    pub FixedElementField: u16,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer =>
    "Unity.Burst.Intrinsics"."X86/Sse4_2/StrBoolArray/<Bits>e__FixedBuffer"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
impl crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer {}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
#[repr(C)]
#[derive(Debug)]
pub struct X86 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86 =>
    "Unity.Burst.Intrinsics"."X86"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
impl crate::Unity::Burst::Intrinsics::X86 {
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
    pub type Avx = crate::Unity::Burst::Intrinsics::X86_Avx;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
    pub type Avx2 = crate::Unity::Burst::Intrinsics::X86_Avx2;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
    pub type Bmi1 = crate::Unity::Burst::Intrinsics::X86_Bmi1;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
    pub type Bmi2 = crate::Unity::Burst::Intrinsics::X86_Bmi2;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
    pub type F16C = crate::Unity::Burst::Intrinsics::X86_F16C;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
    pub type Fma = crate::Unity::Burst::Intrinsics::X86_Fma;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+MXCSRBits")]
    pub type MXCSRBits = crate::Unity::Burst::Intrinsics::X86_MXCSRBits;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
    pub type Popcnt = crate::Unity::Burst::Intrinsics::X86_Popcnt;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingMode")]
    pub type RoundingMode = crate::Unity::Burst::Intrinsics::X86_RoundingMode;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
    pub type RoundingScope = crate::Unity::Burst::Intrinsics::X86_RoundingScope;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
    pub type Sse = crate::Unity::Burst::Intrinsics::X86_Sse;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
    pub type Sse2 = crate::Unity::Burst::Intrinsics::X86_Sse2;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
    pub type Sse3 = crate::Unity::Burst::Intrinsics::X86_Sse3;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
    pub type Sse4_1 = crate::Unity::Burst::Intrinsics::X86_Sse4_1;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
    pub type Sse4_2 = crate::Unity::Burst::Intrinsics::X86_Sse4_2;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
    pub type Ssse3 = crate::Unity::Burst::Intrinsics::X86_Ssse3;
    pub fn BurstIntrinsicGetCSRFromManaged() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BurstIntrinsicGetCSRFromManaged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BurstIntrinsicSetCSRFromManaged(
        _cordl__: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BurstIntrinsicSetCSRFromManaged", (_cordl__))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoGetCSRTrampoline() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoGetCSRTrampoline", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoSetCSRTrampoline(
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoSetCSRTrampoline", (bits))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenericCSharpLoad(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenericCSharpLoad", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenericCSharpStore(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenericCSharpStore", (ptr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNaN_u32_0(v: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNaN", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNaN_u64_1(v: u64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNaN", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn Saturate_To_Int16(val: i32) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Saturate_To_Int16", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn Saturate_To_Int8(val: i32) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Saturate_To_Int8", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn Saturate_To_UnsignedInt16(val: i32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Saturate_To_UnsignedInt16", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn Saturate_To_UnsignedInt8(val: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Saturate_To_UnsignedInt8", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MXCSR() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::X86_MXCSRBits,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::X86_MXCSRBits = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MXCSR", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn getcsr_raw() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("getcsr_raw", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MXCSR(
        value: crate::Unity::Burst::Intrinsics::X86_MXCSRBits,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_MXCSR", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn setcsr_raw(
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("setcsr_raw", (bits))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Avx {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Avx =>
    "Unity.Burst.Intrinsics"."X86/Avx"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Avx {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Avx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
impl crate::Unity::Burst::Intrinsics::X86_Avx {
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx+CMP")]
    pub type CMP = crate::Unity::Burst::Intrinsics::Avx_X86_CMP;
    pub fn Select4(
        src1: crate::Unity::Burst::Intrinsics::v256,
        src2: crate::Unity::Burst::Intrinsics::v256,
        control: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Select4", (src1, src2, control))?;
        Ok(__cordl_ret.into())
    }
    pub fn broadcast_ss(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("broadcast_ss", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmp_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmp_pd", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmp_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmp_ps", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmp_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmp_sd", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmp_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmp_ss", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAvxSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsAvxSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn maskload_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("maskload_pd", (mem_addr, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn maskload_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("maskload_ps", (mem_addr, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn maskstore_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("maskstore_pd", (mem_addr, mask, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn maskstore_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("maskstore_ps", (mem_addr, mask, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_add_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_add_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_addsub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_addsub_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_addsub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_addsub_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_and_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_and_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_and_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_and_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_andnot_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_andnot_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_andnot_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_andnot_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blend_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_blend_pd", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blend_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_blend_ps", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blendv_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_blendv_pd", (a, b, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blendv_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_blendv_ps", (a, b, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcast_pd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_broadcast_pd", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcast_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_broadcast_ps", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcast_sd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_broadcast_sd", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcast_ss(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_broadcast_ss", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castpd128_pd256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castpd128_pd256", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castpd256_pd128(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castpd256_pd128", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castpd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castpd_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castpd_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castpd_si256", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castps128_ps256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castps128_ps256", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castps256_ps128(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castps256_ps128", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castps_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castps_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castps_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castps_si256", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castsi128_si256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castsi128_si256", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castsi256_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castsi256_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castsi256_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castsi256_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castsi256_si128(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_castsi256_si128", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_ceil_pd(
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_ceil_pd", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_ceil_ps(
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_ceil_ps", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmp_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cmp_pd", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmp_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cmp_ps", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi32_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepi32_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi32_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepi32_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtpd_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtpd_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtpd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtpd_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtps_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtps_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtps_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtps_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtss_f32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtss_f32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvttpd_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvttpd_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvttps_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvttps_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_div_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_div_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_div_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_div_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_dp_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_dp_ps", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extract_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_extract_epi32", (a, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extract_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_extract_epi64", (a, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extractf128_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_extractf128_pd", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extractf128_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_extractf128_ps", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extractf128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_extractf128_si256", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_floor_pd(
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_floor_pd", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_floor_ps(
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_floor_ps", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hadd_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_hadd_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hadd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_hadd_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hsub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_hsub_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hsub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_hsub_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insert_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        i: i32,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_insert_epi16", (a, i, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insert_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        i: i32,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_insert_epi32", (a, i, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insert_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        i: i64,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_insert_epi64", (a, i, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insert_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        i: i32,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_insert_epi8", (a, i, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insertf128_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_insertf128_pd", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insertf128_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_insertf128_ps", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insertf128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_insertf128_si256", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_lddqu_si256(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_lddqu_si256", (mem_addr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_load_pd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_load_pd", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_load_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_load_ps", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_load_si256(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_load_si256", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu2_m128(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_loadu2_m128", (hiaddr, loaddr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu2_m128d(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_loadu2_m128d", (hiaddr, loaddr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu2_m128i(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_loadu2_m128i", (hiaddr, loaddr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu_pd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_loadu_pd", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_loadu_ps", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu_si256(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_loadu_si256", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskload_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_maskload_pd", (mem_addr, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskload_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_maskload_ps", (mem_addr, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskstore_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_maskstore_pd", (mem_addr, mask, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskstore_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_maskstore_ps", (mem_addr, mask, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_max_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_max_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_min_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_min_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_movedup_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_movedup_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_movehdup_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_movehdup_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_moveldup_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_moveldup_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_movemask_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_movemask_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_movemask_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_movemask_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mul_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mul_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mul_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mul_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_or_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_or_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_or_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_or_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute2f128_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permute2f128_pd", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute2f128_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permute2f128_ps", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute2f128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permute2f128_si256", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permute_pd", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permute_ps", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permutevar_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permutevar_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permutevar_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permutevar_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_rcp_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_rcp_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_round_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_round_pd", (a, rounding))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_round_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_round_ps", (a, rounding))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_rsqrt_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_rsqrt_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_epi16(
        a: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set1_epi16", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_epi32(
        a: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set1_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_epi64x(
        a: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set1_epi64x", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_epi8(
        a: u8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set1_epi8", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_pd(
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set1_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_ps(
        a: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set1_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_epi16(
        e15_: i16,
        e14_: i16,
        e13_: i16,
        e12_: i16,
        e11_: i16,
        e10_: i16,
        e9_: i16,
        e8_: i16,
        e7_: i16,
        e6_: i16,
        e5_: i16,
        e4_: i16,
        e3_: i16,
        e2_: i16,
        e1_: i16,
        e0_: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "mm256_set_epi16",
                (
                    e15_,
                    e14_,
                    e13_,
                    e12_,
                    e11_,
                    e10_,
                    e9_,
                    e8_,
                    e7_,
                    e6_,
                    e5_,
                    e4_,
                    e3_,
                    e2_,
                    e1_,
                    e0_,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_epi32(
        e7: i32,
        e6: i32,
        e5: i32,
        e4: i32,
        e3: i32,
        e2: i32,
        e1: i32,
        e0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set_epi32", (e7, e6, e5, e4, e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_epi64x(
        e3: i64,
        e2: i64,
        e1: i64,
        e0: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set_epi64x", (e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_epi8(
        e31_: u8,
        e30_: u8,
        e29_: u8,
        e28_: u8,
        e27_: u8,
        e26_: u8,
        e25_: u8,
        e24_: u8,
        e23_: u8,
        e22_: u8,
        e21_: u8,
        e20_: u8,
        e19_: u8,
        e18_: u8,
        e17_: u8,
        e16_: u8,
        e15_: u8,
        e14_: u8,
        e13_: u8,
        e12_: u8,
        e11_: u8,
        e10_: u8,
        e9_: u8,
        e8_: u8,
        e7_: u8,
        e6_: u8,
        e5_: u8,
        e4_: u8,
        e3_: u8,
        e2_: u8,
        e1_: u8,
        e0_: u8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "mm256_set_epi8",
                (
                    e31_,
                    e30_,
                    e29_,
                    e28_,
                    e27_,
                    e26_,
                    e25_,
                    e24_,
                    e23_,
                    e22_,
                    e21_,
                    e20_,
                    e19_,
                    e18_,
                    e17_,
                    e16_,
                    e15_,
                    e14_,
                    e13_,
                    e12_,
                    e11_,
                    e10_,
                    e9_,
                    e8_,
                    e7_,
                    e6_,
                    e5_,
                    e4_,
                    e3_,
                    e2_,
                    e1_,
                    e0_,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_m128(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set_m128", (hi, lo))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_m128d(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set_m128d", (hi, lo))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_m128i(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set_m128i", (hi, lo))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_pd(
        d: f64,
        c: f64,
        b: f64,
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set_pd", (d, c, b, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_ps(
        e7: f32,
        e6: f32,
        e5: f32,
        e4: f32,
        e3: f32,
        e2: f32,
        e1: f32,
        e0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_set_ps", (e7, e6, e5, e4, e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_epi16(
        e15_: i16,
        e14_: i16,
        e13_: i16,
        e12_: i16,
        e11_: i16,
        e10_: i16,
        e9_: i16,
        e8_: i16,
        e7_: i16,
        e6_: i16,
        e5_: i16,
        e4_: i16,
        e3_: i16,
        e2_: i16,
        e1_: i16,
        e0_: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "mm256_setr_epi16",
                (
                    e15_,
                    e14_,
                    e13_,
                    e12_,
                    e11_,
                    e10_,
                    e9_,
                    e8_,
                    e7_,
                    e6_,
                    e5_,
                    e4_,
                    e3_,
                    e2_,
                    e1_,
                    e0_,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_epi32(
        e7: i32,
        e6: i32,
        e5: i32,
        e4: i32,
        e3: i32,
        e2: i32,
        e1: i32,
        e0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_setr_epi32", (e7, e6, e5, e4, e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_epi64x(
        e3: i64,
        e2: i64,
        e1: i64,
        e0: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_setr_epi64x", (e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_epi8(
        e31_: u8,
        e30_: u8,
        e29_: u8,
        e28_: u8,
        e27_: u8,
        e26_: u8,
        e25_: u8,
        e24_: u8,
        e23_: u8,
        e22_: u8,
        e21_: u8,
        e20_: u8,
        e19_: u8,
        e18_: u8,
        e17_: u8,
        e16_: u8,
        e15_: u8,
        e14_: u8,
        e13_: u8,
        e12_: u8,
        e11_: u8,
        e10_: u8,
        e9_: u8,
        e8_: u8,
        e7_: u8,
        e6_: u8,
        e5_: u8,
        e4_: u8,
        e3_: u8,
        e2_: u8,
        e1_: u8,
        e0_: u8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "mm256_setr_epi8",
                (
                    e31_,
                    e30_,
                    e29_,
                    e28_,
                    e27_,
                    e26_,
                    e25_,
                    e24_,
                    e23_,
                    e22_,
                    e21_,
                    e20_,
                    e19_,
                    e18_,
                    e17_,
                    e16_,
                    e15_,
                    e14_,
                    e13_,
                    e12_,
                    e11_,
                    e10_,
                    e9_,
                    e8_,
                    e7_,
                    e6_,
                    e5_,
                    e4_,
                    e3_,
                    e2_,
                    e1_,
                    e0_,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_m128(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_setr_m128", (hi, lo))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_m128d(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_setr_m128d", (hi, lo))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_m128i(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_setr_m128i", (hi, lo))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_pd(
        d: f64,
        c: f64,
        b: f64,
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_setr_pd", (d, c, b, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_ps(
        e7: f32,
        e6: f32,
        e5: f32,
        e4: f32,
        e3: f32,
        e2: f32,
        e1: f32,
        e0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_setr_ps", (e7, e6, e5, e4, e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setzero_pd() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_setzero_pd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setzero_ps() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_setzero_ps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setzero_si256() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_setzero_si256", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shuffle_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_shuffle_pd", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shuffle_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_shuffle_ps", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sqrt_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sqrt_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sqrt_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sqrt_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_store_pd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_store_pd", (ptr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_store_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_store_ps", (ptr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_store_si256(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        v: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_store_si256", (ptr, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu2_m128(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_storeu2_m128", (hiaddr, loaddr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu2_m128d(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_storeu2_m128d", (hiaddr, loaddr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu2_m128i(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_storeu2_m128i", (hiaddr, loaddr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu_pd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_storeu_pd", (ptr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_storeu_ps", (ptr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu_si256(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        v: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_storeu_si256", (ptr, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_stream_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_stream_pd", (mem_addr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_stream_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_stream_ps", (mem_addr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_stream_si256(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_stream_si256", (mem_addr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sub_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sub_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testc_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_testc_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testc_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_testc_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testc_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_testc_si256", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testnzc_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_testnzc_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testnzc_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_testnzc_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testnzc_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_testnzc_si256", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testz_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_testz_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testz_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_testz_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testz_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_testz_si256", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_undefined_pd() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_undefined_pd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_undefined_ps() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_undefined_ps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_undefined_si256() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_undefined_si256", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpackhi_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpackhi_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpacklo_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpacklo_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_xor_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_xor_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_xor_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_xor_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_zeroall() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_zeroall", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_zeroupper() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_zeroupper", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_zextpd128_pd256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_zextpd128_pd256", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_zextps128_ps256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_zextps128_ps256", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_zextsi128_si256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_zextsi128_si256", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn permute_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("permute_pd", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn permute_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("permute_ps", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn permutevar_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("permutevar_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn permutevar_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("permutevar_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn testc_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("testc_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn testc_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("testc_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn testnzc_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("testnzc_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn testnzc_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("testnzc_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn testz_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("testz_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn testz_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("testz_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn undefined_pd() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v128,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("undefined_pd", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn undefined_ps() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v128,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("undefined_ps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn undefined_si128() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v128,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("undefined_si128", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Avx {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Avx2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Avx2 =>
    "Unity.Burst.Intrinsics"."X86/Avx2"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Avx2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Avx2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
impl crate::Unity::Burst::Intrinsics::X86_Avx2 {
    pub fn EmulatedGather_Il2CppObject_Il2CppObject_Il2CppObject_i32_i32_Il2CppObject0<
        T,
        U,
    >(
        dptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        indexPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        scale: i32,
        n: i32,
        mask: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EmulatedGather", (dptr, base_addr, indexPtr, scale, n, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn EmulatedGather_Il2CppObject_Il2CppObject_Il2CppObject_i32_i32_Il2CppObject1<
        T,
        U,
    >(
        dptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        indexPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        scale: i32,
        n: i32,
        mask: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EmulatedGather", (dptr, base_addr, indexPtr, scale, n, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn blend_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blend_epi32", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn broadcastb_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("broadcastb_epi8", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn broadcastd_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("broadcastd_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn broadcastq_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("broadcastq_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn broadcastsd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("broadcastsd_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn broadcastss_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("broadcastss_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn broadcastw_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("broadcastw_epi16", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAvx2Supported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsAvx2Supported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn i32gather_epi32(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("i32gather_epi32", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn i32gather_epi64(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("i32gather_epi64", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn i32gather_pd(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("i32gather_pd", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn i32gather_ps(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("i32gather_ps", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn i64gather_epi32(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("i64gather_epi32", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn i64gather_epi64(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("i64gather_epi64", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn i64gather_pd(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("i64gather_pd", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn i64gather_ps(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("i64gather_ps", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mask_i32gather_epi32(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mask_i32gather_epi32", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mask_i32gather_epi64(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mask_i32gather_epi64", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mask_i32gather_pd(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mask_i32gather_pd", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mask_i32gather_ps(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mask_i32gather_ps", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mask_i64gather_epi32(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mask_i64gather_epi32", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mask_i64gather_epi64(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mask_i64gather_epi64", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mask_i64gather_pd(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mask_i64gather_pd", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mask_i64gather_ps(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mask_i64gather_ps", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn maskload_epi32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("maskload_epi32", (mem_addr, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn maskload_epi64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("maskload_epi64", (mem_addr, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn maskstore_epi32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("maskstore_epi32", (mem_addr, mask, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn maskstore_epi64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("maskstore_epi64", (mem_addr, mask, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_abs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_abs_epi16", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_abs_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_abs_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_abs_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_abs_epi8", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_add_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_add_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_add_epi64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_add_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_adds_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_adds_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_adds_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_adds_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_adds_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_adds_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_adds_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_adds_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_alignr_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_alignr_epi8", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_and_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_and_si256", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_andnot_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_andnot_si256", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_avg_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_avg_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_avg_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_avg_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blend_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_blend_epi16", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blend_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_blend_epi32", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blendv_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_blendv_epi8", (a, b, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastb_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_broadcastb_epi8", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastd_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_broadcastd_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastq_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_broadcastq_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastsd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_broadcastsd_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastsi128_si256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_broadcastsi128_si256", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastss_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_broadcastss_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastw_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_broadcastw_epi16", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_bslli_epi128(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_bslli_epi128", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_bsrli_epi128(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_bsrli_epi128", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpeq_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cmpeq_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpeq_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cmpeq_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpeq_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cmpeq_epi64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpeq_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cmpeq_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpgt_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cmpgt_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpgt_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cmpgt_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpgt_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cmpgt_epi64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpgt_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cmpgt_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi16_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepi16_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi16_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepi16_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi32_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepi32_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi8_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepi8_epi16", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi8_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepi8_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi8_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepi8_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu16_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepu16_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu16_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepu16_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu32_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepu32_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu8_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepu8_epi16", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu8_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepu8_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu8_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtepu8_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtsd_f64(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtsd_f64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtsi256_si32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtsi256_si32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtsi256_si64(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtsi256_si64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extract_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_extract_epi16", (a, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extract_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_extract_epi8", (a, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extracti128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_extracti128_si256", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hadd_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_hadd_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hadd_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_hadd_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hadds_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_hadds_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hsub_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_hsub_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hsub_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_hsub_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hsubs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_hsubs_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i32gather_epi32(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_i32gather_epi32", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i32gather_epi64(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_i32gather_epi64", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i32gather_pd(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_i32gather_pd", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i32gather_ps(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_i32gather_ps", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i64gather_epi32(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_i64gather_epi32", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i64gather_epi64(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_i64gather_epi64", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i64gather_pd(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_i64gather_pd", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i64gather_ps(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_i64gather_ps", (base_addr, vindex, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_inserti128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_inserti128_si256", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_madd_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_madd_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maddubs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_maddubs_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i32gather_epi32(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "mm256_mask_i32gather_epi32",
                (src, base_addr, vindex, mask, scale),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i32gather_epi64(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "mm256_mask_i32gather_epi64",
                (src, base_addr, vindex, mask, scale),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i32gather_pd(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mask_i32gather_pd", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i32gather_ps(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mask_i32gather_ps", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i64gather_epi32(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "mm256_mask_i64gather_epi32",
                (src, base_addr, vindex, mask, scale),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i64gather_epi64(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "mm256_mask_i64gather_epi64",
                (src, base_addr, vindex, mask, scale),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i64gather_pd(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mask_i64gather_pd", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i64gather_ps(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mask_i64gather_ps", (src, base_addr, vindex, mask, scale))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskload_epi32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_maskload_epi32", (mem_addr, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskload_epi64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_maskload_epi64", (mem_addr, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskstore_epi32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_maskstore_epi32", (mem_addr, mask, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskstore_epi64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_maskstore_epi64", (mem_addr, mask, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_max_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_max_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_max_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_max_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epu32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_max_epu32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_max_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_min_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_min_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_min_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_min_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epu32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_min_epu32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_min_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_movemask_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_movemask_epi8", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mpsadbw_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mpsadbw_epu8", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mul_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mul_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mul_epu32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mul_epu32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mulhi_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mulhi_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mulhi_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mulhi_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mulhrs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mulhrs_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mullo_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mullo_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mullo_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_mullo_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_or_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_or_si256", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_packs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_packs_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_packs_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_packs_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_packus_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_packus_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_packus_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_packus_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute2x128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permute2x128_si256", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute4x64_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permute4x64_epi64", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute4x64_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permute4x64_pd", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permutevar8x32_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        idx: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permutevar8x32_epi32", (a, idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permutevar8x32_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        idx: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_permutevar8x32_ps", (a, idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sad_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sad_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shuffle_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_shuffle_epi32", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shuffle_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_shuffle_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shufflehi_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_shufflehi_epi16", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shufflelo_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_shufflelo_epi16", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sign_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sign_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sign_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sign_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sign_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sign_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sll_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sll_epi16", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sll_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sll_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sll_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sll_epi64", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_slli_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_slli_epi16", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_slli_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_slli_epi32", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_slli_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_slli_epi64", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_slli_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_slli_si256", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sllv_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sllv_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sllv_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sllv_epi64", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sra_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sra_epi16", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sra_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sra_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srai_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srai_epi16", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srai_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srai_epi32", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srav_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srav_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srl_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srl_epi16", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srl_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srl_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srl_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srl_epi64", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srli_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srli_epi16", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srli_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srli_epi32", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srli_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srli_epi64", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srli_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srli_si256", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srlv_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srlv_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srlv_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_srlv_epi64", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_stream_load_si256(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_stream_load_si256", (mem_addr))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sub_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sub_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sub_epi64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_sub_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_subs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_subs_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_subs_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_subs_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_subs_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_subs_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_subs_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_subs_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpackhi_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpackhi_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpackhi_epi64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpackhi_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpacklo_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpacklo_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpacklo_epi64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_unpacklo_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_xor_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_xor_si256", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sllv_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sllv_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn sllv_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sllv_epi64", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn srav_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srav_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn srlv_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srlv_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn srlv_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srlv_epi64", (a, count))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Avx2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Bmi1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Bmi1 =>
    "Unity.Burst.Intrinsics"."X86/Bmi1"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Bmi1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Bmi1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
impl crate::Unity::Burst::Intrinsics::X86_Bmi1 {
    pub fn andn_u32(a: u32, b: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("andn_u32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn andn_u64(a: u64, b: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("andn_u64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn bextr2_u32(a: u32, control: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bextr2_u32", (a, control))?;
        Ok(__cordl_ret.into())
    }
    pub fn bextr2_u64(a: u64, control: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bextr2_u64", (a, control))?;
        Ok(__cordl_ret.into())
    }
    pub fn bextr_u32(
        a: u32,
        start: u32,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bextr_u32", (a, start, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn bextr_u64(
        a: u64,
        start: u32,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bextr_u64", (a, start, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn blsi_u32(a: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blsi_u32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn blsi_u64(a: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blsi_u64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn blsmsk_u32(a: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blsmsk_u32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn blsmsk_u64(a: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blsmsk_u64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn blsr_u32(a: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blsr_u32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn blsr_u64(a: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blsr_u64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsBmi1Supported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsBmi1Supported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_u32(a: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt_u32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_u64(a: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt_u64", (a))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Bmi1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Bmi2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Bmi2 =>
    "Unity.Burst.Intrinsics"."X86/Bmi2"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Bmi2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Bmi2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
impl crate::Unity::Burst::Intrinsics::X86_Bmi2 {
    pub fn bzhi_u32(a: u32, index: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bzhi_u32", (a, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn bzhi_u64(a: u64, index: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bzhi_u64", (a, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsBmi2Supported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsBmi2Supported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn mulx_u32(
        a: u32,
        b: u32,
        hi: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mulx_u32", (a, b, hi))?;
        Ok(__cordl_ret.into())
    }
    pub fn mulx_u64(
        a: u64,
        b: u64,
        hi: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mulx_u64", (a, b, hi))?;
        Ok(__cordl_ret.into())
    }
    pub fn pdep_u32(a: u32, mask: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pdep_u32", (a, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn pdep_u64(a: u64, mask: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pdep_u64", (a, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn pext_u32(a: u32, mask: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pext_u32", (a, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn pext_u64(a: u64, mask: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pext_u64", (a, mask))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Bmi2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_F16C {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_F16C =>
    "Unity.Burst.Intrinsics"."X86/F16C"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_F16C {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_F16C {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
impl crate::Unity::Burst::Intrinsics::X86_F16C {
    pub fn FloatToHalf(f: u32, rounding: i32) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FloatToHalf", (f, rounding))?;
        Ok(__cordl_ret.into())
    }
    pub fn HalfToFloat(h: u16) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HalfToFloat", (h))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtph_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtph_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtps_ph(
        a: crate::Unity::Burst::Intrinsics::v128,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtps_ph", (a, rounding))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsF16CSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsF16CSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtph_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtph_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtps_ph(
        a: crate::Unity::Burst::Intrinsics::v256,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_cvtps_ph", (a, rounding))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_F16C {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Fma {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Fma =>
    "Unity.Burst.Intrinsics"."X86/Fma"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Fma {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Fma {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
impl crate::Unity::Burst::Intrinsics::X86_Fma {
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
    pub type Union = crate::Unity::Burst::Intrinsics::Fma_X86_Union;
    pub fn FmaHelper(a: f32, b: f32, c: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FmaHelper", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn FnmaHelper(a: f32, b: f32, c: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FnmaHelper", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmadd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmadd_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmadd_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmadd_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmadd_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmadd_sd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmadd_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmadd_ss", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmaddsub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmaddsub_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmaddsub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmaddsub_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmsub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmsub_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmsub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmsub_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmsub_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmsub_sd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmsub_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmsub_ss", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmsubadd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmsubadd_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmsubadd_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmsubadd_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fnmadd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fnmadd_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fnmadd_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fnmadd_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fnmadd_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fnmadd_sd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fnmadd_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fnmadd_ss", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fnmsub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fnmsub_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fnmsub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fnmsub_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fnmsub_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fnmsub_sd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn fnmsub_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fnmsub_ss", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFmaSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsFmaSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmadd_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fmadd_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmadd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fmadd_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmaddsub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fmaddsub_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmaddsub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fmaddsub_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmsub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fmsub_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmsub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fmsub_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmsubadd_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fmsubadd_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmsubadd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fmsubadd_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fnmadd_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fnmadd_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fnmadd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fnmadd_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fnmsub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fnmsub_pd", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fnmsub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mm256_fnmsub_ps", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Fma {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+MXCSRBits")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86_MXCSRBits {
    DenormalFlag = 2i32,
    DenormalOperationMask = 256i32,
    DenormalsAreZeroes = 64i32,
    DivideByZeroFlag = 4i32,
    DivideByZeroMask = 512i32,
    ExceptionMask = 8064i32,
    FlagMask = 63i32,
    FlushToZero = 32768i32,
    InvalidOperationFlag = 1i32,
    InvalidOperationMask = 128i32,
    OverflowFlag = 8i32,
    OverflowMask = 1024i32,
    PrecisionFlag = 32i32,
    PrecisionMask = 4096i32,
    RoundDown = 8192i32,
    RoundToNearest = 0i32,
    RoundTowardZero = 24576i32,
    RoundUp = 16384i32,
    UnderflowFlag = 16i32,
    UnderflowMask = 2048i32,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+MXCSRBits")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_MXCSRBits =>
    "Unity.Burst.Intrinsics"."X86/MXCSRBits"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Popcnt {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Popcnt =>
    "Unity.Burst.Intrinsics"."X86/Popcnt"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Popcnt {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Popcnt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
impl crate::Unity::Burst::Intrinsics::X86_Popcnt {
    pub fn get_IsPopcntSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsPopcntSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn popcnt_u32(v: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("popcnt_u32", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn popcnt_u64(v: u64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("popcnt_u64", (v))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Popcnt {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86_RoundingMode {
    FROUND_CEIL = 2i32,
    FROUND_CEIL_NOEXC = 10i32,
    FROUND_CUR_DIRECTION = 4i32,
    FROUND_FLOOR = 1i32,
    FROUND_FLOOR_NOEXC = 9i32,
    FROUND_NEARBYINT = 12i32,
    FROUND_NINT = 0i32,
    FROUND_NINT_NOEXC = 8i32,
    FROUND_TO_ZERO = 3i32,
    FROUND_TRUNC_NOEXC = 11i32,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_RoundingMode =>
    "Unity.Burst.Intrinsics"."X86/RoundingMode"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct X86_RoundingScope {
    pub OldBits: crate::Unity::Burst::Intrinsics::X86_MXCSRBits,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_RoundingScope =>
    "Unity.Burst.Intrinsics"."X86/RoundingScope"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::Intrinsics::X86_RoundingScope {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
impl crate::Unity::Burst::Intrinsics::X86_RoundingScope {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        roundingMode: crate::Unity::Burst::Intrinsics::X86_MXCSRBits,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (roundingMode),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
impl AsRef<crate::System::IDisposable>
for crate::Unity::Burst::Intrinsics::X86_RoundingScope {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
impl AsMut<crate::System::IDisposable>
for crate::Unity::Burst::Intrinsics::X86_RoundingScope {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Sse {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Sse =>
    "Unity.Burst.Intrinsics"."X86/Sse"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Sse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
impl crate::Unity::Burst::Intrinsics::X86_Sse {
    pub fn SHUFFLE(
        d: i32,
        c: i32,
        b: i32,
        a: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SHUFFLE", (d, c, b, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn TRANSPOSE4_PS(
        row0: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::Intrinsics::v128>,
        row1: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::Intrinsics::v128>,
        row2: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::Intrinsics::v128>,
        row3: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::Intrinsics::v128>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TRANSPOSE4_PS", (row0, row1, row2, row3))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn and_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("and_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn andnot_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("andnot_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpeq_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpeq_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpge_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpge_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpge_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpge_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpgt_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpgt_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmple_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmple_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmple_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmple_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmplt_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmplt_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpneq_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpneq_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpneq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpneq_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnge_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnge_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnge_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnge_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpngt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpngt_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpngt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpngt_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnle_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnle_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnle_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnle_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnlt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnlt_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnlt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnlt_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpord_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpord_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpord_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpord_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpunord_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpunord_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpunord_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpunord_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comieq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comieq_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comige_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comige_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comigt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comigt_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comile_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comile_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comilt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comilt_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comineq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comineq_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvt_ss2si(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvt_ss2si", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi32_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsi32_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi64_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsi64_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtss_f32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtss_f32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtss_si32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtss_si32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtss_si64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtss_si64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtt_ss2si(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtt_ss2si", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvttss_si32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvttss_si32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvttss_si64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvttss_si64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn div_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("div_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn div_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("div_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSseSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsSseSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn load_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("load_ps", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn loadu_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("loadu_ps", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn loadu_si16(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("loadu_si16", (mem_addr))?;
        Ok(__cordl_ret.into())
    }
    pub fn loadu_si64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("loadu_si64", (mem_addr))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn move_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("move_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn movehl_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("movehl_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn movelh_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("movelh_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn movemask_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("movemask_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn or_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("or_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn rcp_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rcp_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn rcp_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rcp_ss", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rsqrt_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rsqrt_ss", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn set1_ps(
        a: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set1_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ps(
        e3: f32,
        e2: f32,
        e1: f32,
        e0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ps", (e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ps1(
        a: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ps1", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ss(
        a: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ss", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn setr_ps(
        e3: f32,
        e2: f32,
        e1: f32,
        e0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("setr_ps", (e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn setzero_ps() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v128,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("setzero_ps", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle_ps", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt_ss", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn store_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("store_ps", (ptr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn storeu_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("storeu_ps", (ptr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn storeu_si16(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("storeu_si16", (mem_addr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn storeu_si64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("storeu_si64", (mem_addr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn stream_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("stream_ps", (mem_addr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn sub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sub_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sub_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sub_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomieq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomieq_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomige_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomige_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomigt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomigt_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomile_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomile_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomilt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomilt_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomineq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomineq_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpackhi_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpacklo_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn xor_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("xor_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Sse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Sse2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Sse2 =>
    "Unity.Burst.Intrinsics"."X86/Sse2"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Sse2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
impl crate::Unity::Burst::Intrinsics::X86_Sse2 {
    pub fn SHUFFLE2(x: i32, y: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SHUFFLE2", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_epi64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("add_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn adds_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("adds_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn adds_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("adds_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn adds_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("adds_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn adds_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("adds_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn and_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("and_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn and_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("and_si128", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn andnot_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("andnot_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn andnot_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("andnot_si128", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn avg_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("avg_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn avg_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("avg_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn bslli_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bslli_si128", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn bsrli_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bsrli_si128", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn clflush(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clflush", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpeq_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpeq_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpeq_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpeq_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpeq_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpge_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpge_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpge_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpge_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpgt_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpgt_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpgt_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpgt_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpgt_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmple_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmple_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmple_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmple_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmplt_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmplt_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmplt_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmplt_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmplt_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpneq_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpneq_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpneq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpneq_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnge_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnge_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnge_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnge_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpngt_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpngt_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpngt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpngt_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnle_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnle_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnle_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnle_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnlt_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnlt_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpnlt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpnlt_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpord_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpord_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpord_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpord_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpunord_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpunord_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpunord_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpunord_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comieq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comieq_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comige_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comige_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comigt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comigt_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comile_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comile_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comilt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comilt_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn comineq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("comineq_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi32_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepi32_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi32_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepi32_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtpd_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtpd_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtpd_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtpd_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtps_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtps_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtps_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtps_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsd_f64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsd_f64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsd_si32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsd_si32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsd_si64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsd_si64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsd_si64x(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsd_si64x", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsd_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsd_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi128_si32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsi128_si32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi128_si64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsi128_si64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi128_si64x(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsi128_si64x", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi32_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsi32_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi32_si128(
        a: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsi32_si128", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi64_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsi64_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi64_si128(
        a: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsi64_si128", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi64x_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsi64x_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi64x_si128(
        a: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtsi64x_si128", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtss_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtss_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvttpd_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvttpd_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvttps_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvttps_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvttsd_si32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvttsd_si32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvttsd_si64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvttsd_si64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvttsd_si64x(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvttsd_si64x", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn div_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("div_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn div_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("div_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn extract_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("extract_epi16", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSse2Supported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsSse2Supported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn insert_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        i: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("insert_epi16", (a, i, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn load_si128(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("load_si128", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn loadu_si128(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("loadu_si128", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn loadu_si32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("loadu_si32", (mem_addr))?;
        Ok(__cordl_ret.into())
    }
    pub fn madd_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("madd_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn move_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("move_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn move_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("move_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn movemask_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("movemask_epi8", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn movemask_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("movemask_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_epu32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul_epu32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mulhi_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mulhi_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mulhi_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mulhi_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mullo_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mullo_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn or_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("or_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn or_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("or_si128", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn packs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("packs_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn packs_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("packs_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn packus_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("packus_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sad_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sad_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn set1_epi16(
        a: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set1_epi16", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn set1_epi32(
        a: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set1_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn set1_epi64x(
        a: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set1_epi64x", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn set1_epi8(
        a: i8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set1_epi8", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn set1_pd(
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set1_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_epi16(
        e7: i16,
        e6: i16,
        e5: i16,
        e4: i16,
        e3: i16,
        e2: i16,
        e1: i16,
        e0: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_epi16", (e7, e6, e5, e4, e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_epi32(
        e3: i32,
        e2: i32,
        e1: i32,
        e0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_epi32", (e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_epi64x(
        e1: i64,
        e0: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_epi64x", (e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_epi8(
        e15_: i8,
        e14_: i8,
        e13_: i8,
        e12_: i8,
        e11_: i8,
        e10_: i8,
        e9_: i8,
        e8_: i8,
        e7_: i8,
        e6_: i8,
        e5_: i8,
        e4_: i8,
        e3_: i8,
        e2_: i8,
        e1_: i8,
        e0_: i8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "set_epi8",
                (
                    e15_,
                    e14_,
                    e13_,
                    e12_,
                    e11_,
                    e10_,
                    e9_,
                    e8_,
                    e7_,
                    e6_,
                    e5_,
                    e4_,
                    e3_,
                    e2_,
                    e1_,
                    e0_,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pd(
        e1: f64,
        e0: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_pd", (e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pd1(
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_pd1", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sd(
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_sd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn setr_epi16(
        e7: i16,
        e6: i16,
        e5: i16,
        e4: i16,
        e3: i16,
        e2: i16,
        e1: i16,
        e0: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("setr_epi16", (e7, e6, e5, e4, e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn setr_epi32(
        e3: i32,
        e2: i32,
        e1: i32,
        e0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("setr_epi32", (e3, e2, e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn setr_epi8(
        e15_: i8,
        e14_: i8,
        e13_: i8,
        e12_: i8,
        e11_: i8,
        e10_: i8,
        e9_: i8,
        e8_: i8,
        e7_: i8,
        e6_: i8,
        e5_: i8,
        e4_: i8,
        e3_: i8,
        e2_: i8,
        e1_: i8,
        e0_: i8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "setr_epi8",
                (
                    e15_,
                    e14_,
                    e13_,
                    e12_,
                    e11_,
                    e10_,
                    e9_,
                    e8_,
                    e7_,
                    e6_,
                    e5_,
                    e4_,
                    e3_,
                    e2_,
                    e1_,
                    e0_,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn setr_pd(
        e1: f64,
        e0: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("setr_pd", (e1, e0))?;
        Ok(__cordl_ret.into())
    }
    pub fn setzero_si128() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v128,
    > {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("setzero_si128", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle_epi32", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle_pd", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn shufflehi_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shufflehi_epi16", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn shufflelo_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shufflelo_epi16", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn sll_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sll_epi16", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn sll_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sll_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn sll_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sll_epi64", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn slli_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("slli_epi16", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn slli_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("slli_epi32", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn slli_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("slli_epi64", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn slli_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("slli_si128", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sra_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sra_epi16", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn sra_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sra_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn srai_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srai_epi16", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn srai_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srai_epi32", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn srl_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srl_epi16", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn srl_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srl_epi32", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn srl_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srl_epi64", (a, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn srli_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srli_epi16", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn srli_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srli_epi32", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn srli_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srli_epi64", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn srli_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srli_si128", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn store_si128(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("store_si128", (ptr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn storeu_si128(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("storeu_si128", (ptr, val))?;
        Ok(__cordl_ret.into())
    }
    pub fn storeu_si32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("storeu_si32", (mem_addr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn stream_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("stream_pd", (mem_addr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn stream_si128(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("stream_si128", (mem_addr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn stream_si32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("stream_si32", (mem_addr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn stream_si64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("stream_si64", (mem_addr, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn sub_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sub_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sub_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sub_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sub_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sub_epi64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sub_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sub_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sub_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sub_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sub_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn subs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("subs_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn subs_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("subs_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn subs_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("subs_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn subs_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("subs_epu8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomieq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomieq_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomige_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomige_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomigt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomigt_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomile_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomile_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomilt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomilt_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ucomineq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ucomineq_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpackhi_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpackhi_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpackhi_epi64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpackhi_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpackhi_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpacklo_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpacklo_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpacklo_epi64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpacklo_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpacklo_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn xor_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("xor_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn xor_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("xor_si128", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Sse2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Sse3 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Sse3 =>
    "Unity.Burst.Intrinsics"."X86/Sse3"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse3 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Sse3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
impl crate::Unity::Burst::Intrinsics::X86_Sse3 {
    pub fn addsub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("addsub_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn addsub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("addsub_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSse3Supported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsSse3Supported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn hadd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hadd_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn hadd_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hadd_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn hsub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hsub_pd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn hsub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hsub_ps", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn movedup_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("movedup_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn movehdup_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("movehdup_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn moveldup_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("moveldup_ps", (a))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Sse3 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Sse4_1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Sse4_1 =>
    "Unity.Burst.Intrinsics"."X86/Sse4_1"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse4_1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Sse4_1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
impl crate::Unity::Burst::Intrinsics::X86_Sse4_1 {
    pub fn MK_INSERTPS_NDX(
        srcField: i32,
        dstField: i32,
        zeroMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MK_INSERTPS_NDX", (srcField, dstField, zeroMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn RoundDImpl(d: f64, roundingMode: i32) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RoundDImpl", (d, roundingMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn blend_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blend_epi16", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn blend_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blend_pd", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn blend_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blend_ps", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn blendv_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blendv_epi8", (a, b, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn blendv_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blendv_pd", (a, b, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn blendv_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("blendv_ps", (a, b, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpeq_epi64", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi16_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepi16_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi16_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepi16_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi32_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepi32_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi8_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepi8_epi16", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi8_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepi8_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi8_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepi8_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu16_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepu16_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu16_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepu16_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu32_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepu32_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu8_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepu8_epi16", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu8_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepu8_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu8_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cvtepu8_epi64", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn dp_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dp_pd", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn dp_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dp_ps", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn extract_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("extract_epi32", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn extract_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("extract_epi64", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn extract_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("extract_epi8", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn extract_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("extract_ps", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn extractf_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("extractf_ps", (a, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor_pd", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor_ps", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor_sd", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor_ss", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSse41Supported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsSse41Supported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn insert_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        i: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("insert_epi32", (a, i, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn insert_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        i: i64,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("insert_epi64", (a, i, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn insert_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        i: u8,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("insert_epi8", (a, i, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn insert_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("insert_ps", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_epu32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max_epu32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min_epu16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_epu32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min_epu32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn minpos_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("minpos_epu16", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn mpsadbw_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mpsadbw_epu8", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mullo_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mullo_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn packus_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("packus_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round_pd", (a, rounding))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round_ps", (a, rounding))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round_sd", (a, b, rounding))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round_ss", (a, b, rounding))?;
        Ok(__cordl_ret.into())
    }
    pub fn stream_load_si128(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("stream_load_si128", (mem_addr))?;
        Ok(__cordl_ret.into())
    }
    pub fn test_all_ones(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("test_all_ones", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn test_all_zeros(
        a: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("test_all_zeros", (a, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn test_mix_ones_zeroes(
        a: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("test_mix_ones_zeroes", (a, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn testc_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("testc_si128", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn testnzc_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("testnzc_si128", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn testz_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("testz_si128", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Sse4_1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Sse4_2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Sse4_2 =>
    "Unity.Burst.Intrinsics"."X86/Sse4_2"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse4_2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Sse4_2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
impl crate::Unity::Burst::Intrinsics::X86_Sse4_2 {
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+SIDD")]
    pub type SIDD = crate::Unity::Burst::Intrinsics::Sse4_2_X86_SIDD;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
    pub type StrBoolArray = crate::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray;
    pub fn ComputeStrCmpIntRes2<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        alen: i32,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        blen: i32,
        len: i32,
        imm8: i32,
        allOnes: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeStrCmpIntRes2", (a, alen, b, blen, len, imm8, allOnes))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeStriOutput(
        len: i32,
        imm8: i32,
        intRes2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeStriOutput", (len, imm8, intRes2))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeStringLength<T>(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeStringLength", (ptr, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeStrmOutput<T>(
        len: i32,
        imm8: i32,
        allOnesT: T,
        intRes2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeStrmOutput", (len, imm8, allOnesT, intRes2))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpestra(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpestra", (a, la, b, lb, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpestrc(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpestrc", (a, la, b, lb, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpestri(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpestri", (a, la, b, lb, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpestri_emulation<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        alen: i32,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        blen: i32,
        len: i32,
        imm8: i32,
        allOnes: i32,
        allOnesT: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "cmpestri_emulation",
                (a, alen, b, blen, len, imm8, allOnes, allOnesT),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpestrm(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpestrm", (a, la, b, lb, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpestrm_emulation<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        alen: i32,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        blen: i32,
        len: i32,
        imm8: i32,
        allOnes: i32,
        allOnesT: T,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "cmpestrm_emulation",
                (a, alen, b, blen, len, imm8, allOnes, allOnesT),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpestro(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpestro", (a, la, b, lb, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpestrs(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpestrs", (a, la, b, lb, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpestrz(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpestrz", (a, la, b, lb, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_epi64(
        val1: crate::Unity::Burst::Intrinsics::v128,
        val2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpgt_epi64", (val1, val2))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpistra(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpistra", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpistrc(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpistrc", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpistri(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpistri", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpistri_emulation<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: i32,
        imm8: i32,
        allOnes: i32,
        allOnesT: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpistri_emulation", (a, b, len, imm8, allOnes, allOnesT))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpistrm(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpistrm", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpistrm_emulation<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: i32,
        imm8: i32,
        allOnes: i32,
        allOnesT: T,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpistrm_emulation", (a, b, len, imm8, allOnes, allOnesT))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpistro(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpistro", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpistrs(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpistrs", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmpistrz(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmpistrz", (a, b, imm8))?;
        Ok(__cordl_ret.into())
    }
    pub fn crc32_u16(crc: u32, v: u16) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("crc32_u16", (crc, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn crc32_u32(crc: u32, v: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("crc32_u32", (crc, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn crc32_u64_i64_0(crc_ul: u64, v: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("crc32_u64", (crc_ul, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn crc32_u64_u64_1(crc_ul: u64, v: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("crc32_u64", (crc_ul, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn crc32_u8(crc: u32, v: u8) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("crc32_u8", (crc, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSse42Supported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsSse42Supported", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Sse4_2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Ssse3 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Ssse3 =>
    "Unity.Burst.Intrinsics"."X86/Ssse3"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Ssse3 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Ssse3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
impl crate::Unity::Burst::Intrinsics::X86_Ssse3 {
    pub fn abs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs_epi16", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs_epi32", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs_epi8", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn alignr_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("alignr_epi8", (a, b, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSsse3Supported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsSsse3Supported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn hadd_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hadd_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn hadd_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hadd_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn hadds_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hadds_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn hsub_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hsub_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn hsub_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hsub_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn hsubs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hsubs_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn maddubs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("maddubs_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mulhrs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mulhrs_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sign_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sign_epi16", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sign_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sign_epi32", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn sign_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sign_epi8", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Ssse3 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
