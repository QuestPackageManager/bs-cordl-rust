#[cfg(feature = "Unity+Burst+BurstString+tBigInt+_m_blocks_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct tBigInt__m_blocks_e__FixedBuffer {
    pub FixedElementField: u32,
}
#[cfg(feature = "Unity+Burst+BurstString+tBigInt+_m_blocks_e__FixedBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::tBigInt__m_blocks_e__FixedBuffer
    => "Unity.Burst"."BurstString/tBigInt/<m_blocks>e__FixedBuffer"
);
#[cfg(feature = "Unity+Burst+BurstString+tBigInt+_m_blocks_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::tBigInt__m_blocks_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+BurstString+tBigInt+_m_blocks_e__FixedBuffer")]
impl crate::Unity::Burst::tBigInt__m_blocks_e__FixedBuffer {}
#[cfg(feature = "Unity+Burst+BurstString")]
#[repr(C)]
#[derive(Debug)]
pub struct BurstString {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+Burst+BurstString")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::BurstString => "Unity.Burst"
    ."BurstString"
);
#[cfg(feature = "Unity+Burst+BurstString")]
impl std::ops::Deref for crate::Unity::Burst::BurstString {
    type Target = crate::System::Object;
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
#[derive(Debug, Clone)]
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
        Ok(__cordl_ret)
    }
    pub fn GetBase(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBase",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_Uppercase(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Uppercase",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Burst+BurstString+NumberBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BurstString_NumberBuffer {
    pub _buffer: *mut quest_hook::libil2cpp::Il2CppObject,
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
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDigitsPointer",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        kind: crate::Unity::Burst::BurstString_NumberBufferKind,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        digitsCount: i32,
        scale: i32,
        isNegative: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (kind, buffer, digitsCount, scale, isNegative),
        )?;
        Ok(__cordl_ret)
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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
#[derive(Debug, Clone)]
pub struct BurstString_tBigInt {
    pub m_length: i32,
    pub m_blocks: crate::Unity::Burst::tBigInt__m_blocks_e__FixedBuffer,
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
    pub type _m_blocks_e__FixedBuffer = crate::Unity::Burst::tBigInt__m_blocks_e__FixedBuffer;
    pub fn GetBlock(&mut self, idx: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBlock",
            (idx),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLength",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetU32(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetU32",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsZero(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsZero",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SetZero(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetZero",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Burst+BurstString+tFloatUnion32")]
#[repr(C)]
#[derive(Debug, Clone)]
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
        Ok(__cordl_ret)
    }
    pub fn GetMantissa(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetMantissa",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsNegative(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNegative",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Burst+BurstString+tFloatUnion64")]
#[repr(C)]
#[derive(Debug, Clone)]
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
        Ok(__cordl_ret)
    }
    pub fn GetMantissa(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetMantissa",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsNegative(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNegative",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
