#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct StreamCompressionModel {
    pub m_Initialized: u8,
    pub encodeTable: crate::Unity::Collections::StreamCompressionModel__encodeTable_e__FixedBuffer,
    pub decodeTable: crate::Unity::Collections::StreamCompressionModel__decodeTable_e__FixedBuffer,
    pub bucketSizes: crate::Unity::Collections::StreamCompressionModel__bucketSizes_e__FixedBuffer,
    pub bucketOffsets:
        crate::Unity::Collections::StreamCompressionModel__bucketOffsets_e__FixedBuffer,
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::StreamCompressionModel {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "StreamCompressionModel";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Collections::StreamCompressionModel {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Collections::StreamCompressionModel {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Collections::StreamCompressionModel {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Collections::StreamCompressionModel {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::StreamCompressionModel
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+StreamCompressionModel")]
impl crate::Unity::Collections::StreamCompressionModel {
    pub const k_AlphabetSize: i32 = 16i32;
    pub const k_MaxContexts: i32 = 1i32;
    pub const k_MaxHuffmanSymbolLength: i32 = 6i32;
    #[cfg(feature = "Unity+Collections+StreamCompressionModel+SharedStaticCompressionModel")]
    pub type SharedStaticCompressionModel =
        crate::Unity::Collections::StreamCompressionModel_SharedStaticCompressionModel;
    #[cfg(feature = "Unity+Collections+StreamCompressionModel+_bucketOffsets_e__FixedBuffer")]
    pub type _bucketOffsets_e__FixedBuffer =
        crate::Unity::Collections::StreamCompressionModel__bucketOffsets_e__FixedBuffer;
    #[cfg(feature = "Unity+Collections+StreamCompressionModel+_bucketSizes_e__FixedBuffer")]
    pub type _bucketSizes_e__FixedBuffer =
        crate::Unity::Collections::StreamCompressionModel__bucketSizes_e__FixedBuffer;
    #[cfg(feature = "Unity+Collections+StreamCompressionModel+_decodeTable_e__FixedBuffer")]
    pub type _decodeTable_e__FixedBuffer =
        crate::Unity::Collections::StreamCompressionModel__decodeTable_e__FixedBuffer;
    #[cfg(feature = "Unity+Collections+StreamCompressionModel+_encodeTable_e__FixedBuffer")]
    pub type _encodeTable_e__FixedBuffer =
        crate::Unity::Collections::StreamCompressionModel__encodeTable_e__FixedBuffer;
    pub fn CalculateBucket(&mut self, value: u32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), i32, 1usize>("CalculateBucket")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalculateBucket",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckAlphabetAndMaxCodeLength(
        alphabetSize: i32,
        maxCodeLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
                        "CheckAlphabetAndMaxCodeLength",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckAlphabetAndMaxCodeLength",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (alphabetSize, maxCodeLength))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckAlphabetSize(
        alphabetSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "CheckAlphabetSize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckAlphabetSize",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (alphabetSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckExceedMaxCodeLength(
        length: i32,
        maxCodeLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
                        "CheckExceedMaxCodeLength",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckExceedMaxCodeLength",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (length, maxCodeLength))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckSymbolLength(
        symbolLengths: crate::Unity::Collections::NativeArray_1<u8>,
        symbolLengthsOffset: i32,
        symbol: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Collections::NativeArray_1<u8>, i32, i32, i32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("CheckSymbolLength")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckSymbolLength", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (symbolLengths, symbolLengthsOffset, symbol, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateHuffmanCodes(
        symbolCodes: crate::Unity::Collections::NativeArray_1<u8>,
        symbolCodesOffset: i32,
        symbolLengths: crate::Unity::Collections::NativeArray_1<u8>,
        symbolLengthsOffset: i32,
        alphabetSize: i32,
        maxCodeLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::NativeArray_1<u8>,
                        i32,
                        crate::Unity::Collections::NativeArray_1<u8>,
                        i32,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "GenerateHuffmanCodes"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateHuffmanCodes",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    symbolCodes,
                    symbolCodesOffset,
                    symbolLengths,
                    symbolLengthsOffset,
                    alphabetSize,
                    maxCodeLength,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateHuffmanDecodeTable(
        decodeTable: crate::Unity::Collections::NativeArray_1<u16>,
        decodeTableOffset: i32,
        symbolLengths: crate::Unity::Collections::NativeArray_1<u8>,
        symbolCodes: crate::Unity::Collections::NativeArray_1<u8>,
        alphabetSize: i32,
        maxCodeLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Unity::Collections::NativeArray_1<u16>,
                        i32,
                        crate::Unity::Collections::NativeArray_1<u8>,
                        crate::Unity::Collections::NativeArray_1<u8>,
                        i32,
                        i32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "GenerateHuffmanDecodeTable"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateHuffmanDecodeTable",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    decodeTable,
                    decodeTableOffset,
                    symbolLengths,
                    symbolCodes,
                    alphabetSize,
                    maxCodeLength,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCompressedSizeInBits(&mut self, value: u32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u32), i32, 1usize>("GetCompressedSizeInBits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCompressedSizeInBits",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReverseBits(value: u32, num_bits: i32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32, i32), u32, 2usize>("ReverseBits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReverseBits",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u32 =
            unsafe { cordl_method_info.invoke_unchecked((), (value, num_bits))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Default(
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::StreamCompressionModel> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::Unity::Collections::StreamCompressionModel,
                        0usize,
                    >("get_Default")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Default", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::StreamCompressionModel =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+StreamCompressionModel+SharedStaticCompressionModel"
)]
#[repr(C)]
#[derive(Debug)]
pub struct StreamCompressionModel_SharedStaticCompressionModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_Unity+Collections+StreamCompressionModel+SharedStaticCompressionModel"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::StreamCompressionModel_SharedStaticCompressionModel
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "StreamCompressionModel/SharedStaticCompressionModel";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Collections+StreamCompressionModel+SharedStaticCompressionModel")]
impl std::ops::Deref
    for crate::Unity::Collections::StreamCompressionModel_SharedStaticCompressionModel
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+StreamCompressionModel+SharedStaticCompressionModel")]
impl std::ops::DerefMut
    for crate::Unity::Collections::StreamCompressionModel_SharedStaticCompressionModel
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Collections+StreamCompressionModel+SharedStaticCompressionModel")]
impl crate::Unity::Collections::StreamCompressionModel_SharedStaticCompressionModel {}
#[cfg(
    feature = "cordl_class_Unity+Collections+StreamCompressionModel+SharedStaticCompressionModel"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::Unity::Collections::StreamCompressionModel_SharedStaticCompressionModel
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketOffsets_e__FixedBuffer"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct StreamCompressionModel__bucketOffsets_e__FixedBuffer {
    pub FixedElementField: u32,
}
#[cfg(
    feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketOffsets_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::StreamCompressionModel__bucketOffsets_e__FixedBuffer
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "StreamCompressionModel/<bucketOffsets>e__FixedBuffer";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketOffsets_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Unity::Collections::StreamCompressionModel__bucketOffsets_e__FixedBuffer
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketOffsets_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Unity::Collections::StreamCompressionModel__bucketOffsets_e__FixedBuffer
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketOffsets_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Unity::Collections::StreamCompressionModel__bucketOffsets_e__FixedBuffer
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketOffsets_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Unity::Collections::StreamCompressionModel__bucketOffsets_e__FixedBuffer
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketOffsets_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::StreamCompressionModel__bucketOffsets_e__FixedBuffer
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+StreamCompressionModel+_bucketOffsets_e__FixedBuffer")]
impl crate::Unity::Collections::StreamCompressionModel__bucketOffsets_e__FixedBuffer {}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketSizes_e__FixedBuffer")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct StreamCompressionModel__bucketSizes_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketSizes_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::StreamCompressionModel__bucketSizes_e__FixedBuffer
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "StreamCompressionModel/<bucketSizes>e__FixedBuffer";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketSizes_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Unity::Collections::StreamCompressionModel__bucketSizes_e__FixedBuffer
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketSizes_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Unity::Collections::StreamCompressionModel__bucketSizes_e__FixedBuffer
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketSizes_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Unity::Collections::StreamCompressionModel__bucketSizes_e__FixedBuffer
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketSizes_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Unity::Collections::StreamCompressionModel__bucketSizes_e__FixedBuffer
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_bucketSizes_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::StreamCompressionModel__bucketSizes_e__FixedBuffer
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+StreamCompressionModel+_bucketSizes_e__FixedBuffer")]
impl crate::Unity::Collections::StreamCompressionModel__bucketSizes_e__FixedBuffer {}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_decodeTable_e__FixedBuffer")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct StreamCompressionModel__decodeTable_e__FixedBuffer {
    pub FixedElementField: u16,
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_decodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::StreamCompressionModel__decodeTable_e__FixedBuffer
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "StreamCompressionModel/<decodeTable>e__FixedBuffer";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_decodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Unity::Collections::StreamCompressionModel__decodeTable_e__FixedBuffer
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_decodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Unity::Collections::StreamCompressionModel__decodeTable_e__FixedBuffer
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_decodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Unity::Collections::StreamCompressionModel__decodeTable_e__FixedBuffer
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_decodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Unity::Collections::StreamCompressionModel__decodeTable_e__FixedBuffer
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_decodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::StreamCompressionModel__decodeTable_e__FixedBuffer
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+StreamCompressionModel+_decodeTable_e__FixedBuffer")]
impl crate::Unity::Collections::StreamCompressionModel__decodeTable_e__FixedBuffer {}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_encodeTable_e__FixedBuffer")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct StreamCompressionModel__encodeTable_e__FixedBuffer {
    pub FixedElementField: u16,
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_encodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Unity::Collections::StreamCompressionModel__encodeTable_e__FixedBuffer
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "StreamCompressionModel/<encodeTable>e__FixedBuffer";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_encodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Unity::Collections::StreamCompressionModel__encodeTable_e__FixedBuffer
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_encodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Unity::Collections::StreamCompressionModel__encodeTable_e__FixedBuffer
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_encodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Unity::Collections::StreamCompressionModel__encodeTable_e__FixedBuffer
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_encodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Unity::Collections::StreamCompressionModel__encodeTable_e__FixedBuffer
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+StreamCompressionModel+_encodeTable_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::StreamCompressionModel__encodeTable_e__FixedBuffer
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+StreamCompressionModel+_encodeTable_e__FixedBuffer")]
impl crate::Unity::Collections::StreamCompressionModel__encodeTable_e__FixedBuffer {}
