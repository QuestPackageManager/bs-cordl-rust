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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86 =>
    "Unity.Burst.Intrinsics"."X86"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86 {
    type Target = crate::System::Object;
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Avx =>
    "Unity.Burst.Intrinsics"."X86/Avx"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Avx {
    type Target = crate::System::Object;
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Avx2 =>
    "Unity.Burst.Intrinsics"."X86/Avx2"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Avx2 {
    type Target = crate::System::Object;
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
impl crate::Unity::Burst::Intrinsics::X86_Avx2 {}
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Bmi1 =>
    "Unity.Burst.Intrinsics"."X86/Bmi1"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Bmi1 {
    type Target = crate::System::Object;
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
impl crate::Unity::Burst::Intrinsics::X86_Bmi1 {}
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Bmi2 =>
    "Unity.Burst.Intrinsics"."X86/Bmi2"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Bmi2 {
    type Target = crate::System::Object;
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
impl crate::Unity::Burst::Intrinsics::X86_Bmi2 {}
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_F16C =>
    "Unity.Burst.Intrinsics"."X86/F16C"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_F16C {
    type Target = crate::System::Object;
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
impl crate::Unity::Burst::Intrinsics::X86_F16C {}
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Fma =>
    "Unity.Burst.Intrinsics"."X86/Fma"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Fma {
    type Target = crate::System::Object;
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Popcnt =>
    "Unity.Burst.Intrinsics"."X86/Popcnt"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Popcnt {
    type Target = crate::System::Object;
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
impl crate::Unity::Burst::Intrinsics::X86_Popcnt {}
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Sse {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Sse =>
    "Unity.Burst.Intrinsics"."X86/Sse"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse {
    type Target = crate::System::Object;
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
impl crate::Unity::Burst::Intrinsics::X86_Sse {}
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Sse2 =>
    "Unity.Burst.Intrinsics"."X86/Sse2"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse2 {
    type Target = crate::System::Object;
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
impl crate::Unity::Burst::Intrinsics::X86_Sse2 {}
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Sse3 =>
    "Unity.Burst.Intrinsics"."X86/Sse3"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse3 {
    type Target = crate::System::Object;
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
impl crate::Unity::Burst::Intrinsics::X86_Sse3 {}
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Sse4_1 =>
    "Unity.Burst.Intrinsics"."X86/Sse4_1"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse4_1 {
    type Target = crate::System::Object;
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
impl crate::Unity::Burst::Intrinsics::X86_Sse4_1 {}
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Sse4_2 =>
    "Unity.Burst.Intrinsics"."X86/Sse4_2"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse4_2 {
    type Target = crate::System::Object;
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
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::Intrinsics::X86_Ssse3 =>
    "Unity.Burst.Intrinsics"."X86/Ssse3"
);
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Ssse3 {
    type Target = crate::System::Object;
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
impl crate::Unity::Burst::Intrinsics::X86_Ssse3 {}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Ssse3 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
