#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct DataStreamReader {
    pub m_BufferPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_Context: crate::Unity::Collections::DataStreamReader_Context,
    pub m_Length: i32,
}
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::DataStreamReader {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "DataStreamReader";
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
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Collections::DataStreamReader {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Collections::DataStreamReader {
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
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Collections::DataStreamReader {
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
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Collections::DataStreamReader {
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
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Collections::DataStreamReader {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+DataStreamReader")]
impl crate::Unity::Collections::DataStreamReader {
    #[cfg(feature = "Unity+Collections+DataStreamReader+Context")]
    pub type Context = crate::Unity::Collections::DataStreamReader_Context;
    pub fn ByteSwap_i16_0(val: i16) -> quest_hook::libil2cpp::Result<i16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i16), i16, 1usize>("ByteSwap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ByteSwap",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i16 = unsafe { cordl_method_info.invoke_unchecked((), (val))? };
        Ok(__cordl_ret.into())
    }
    pub fn ByteSwap_i32_1(val: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), i32, 1usize>("ByteSwap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ByteSwap",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (val))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckBits(numBits: i32) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("CheckBits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckBits",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (numBits))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckRead(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CheckRead")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckRead",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FillBitBuffer(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("FillBitBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FillBitBuffer",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Flush(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Flush")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Flush",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBitsRead(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetBitsRead")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetBitsRead",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBytesRead(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetBytesRead")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetBytesRead",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        _cordl_self: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::DataStreamReader>,
        array: crate::Unity::Collections::NativeArray_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::DataStreamReader,
                        >,
                        crate::Unity::Collections::NativeArray_1<u8>,
                    ), quest_hook::libil2cpp::Void, 2usize>("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Initialize",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_self, array))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadByte(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u8, 0usize>("ReadByte")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadByte",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u8 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadBytesInternal(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("ReadBytesInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadBytesInternal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (data, length))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadBytes_NativeArray_1_0(
        &mut self,
        array: crate::Unity::Collections::NativeArray_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<u8>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReadBytes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadBytes", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (array))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadBytes_Span_1_1(
        &mut self,
        span: crate::System::Span_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Span_1<u8>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReadBytes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadBytes", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (span))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadDouble(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f64, 0usize>("ReadDouble")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadDouble",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFixedString(
        &mut self,
        array: crate::Unity::Collections::NativeArray_1<u8>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::NativeArray_1<u8>), u16, 1usize>(
                        "ReadFixedString",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadFixedString",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u16 = unsafe { cordl_method_info.invoke_unchecked(self, (array))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFixedString128(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::FixedString128Bytes, 0usize>(
                        "ReadFixedString128",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadFixedString128",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFixedString32(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString32Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::FixedString32Bytes, 0usize>(
                        "ReadFixedString32",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadFixedString32",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString32Bytes =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFixedString4096(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString4096Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::FixedString4096Bytes, 0usize>(
                        "ReadFixedString4096",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadFixedString4096",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString4096Bytes =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFixedString512(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::FixedString512Bytes, 0usize>(
                        "ReadFixedString512",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadFixedString512",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFixedString64(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString64Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::FixedString64Bytes, 0usize>(
                        "ReadFixedString64",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadFixedString64",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString64Bytes =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFixedStringInternal(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        maxLength: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                    ), u16, 2usize>("ReadFixedStringInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadFixedStringInternal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u16 =
            unsafe { cordl_method_info.invoke_unchecked(self, (data, maxLength))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFloat(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("ReadFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadFloat",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadInt(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("ReadInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadInt",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadIntNetworkByteOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("ReadIntNetworkByteOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadIntNetworkByteOrder",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadLong(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i64, 0usize>("ReadLong")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadLong",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedDouble(
        &mut self,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Collections::StreamCompressionModel,
                    >), f64, 1usize>("ReadPackedDouble")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedDouble",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { cordl_method_info.invoke_unchecked(self, (model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedDoubleDelta(
        &mut self,
        baseline: f64,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        f64,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), f64, 2usize>("ReadPackedDoubleDelta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedDoubleDelta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f64 =
            unsafe { cordl_method_info.invoke_unchecked(self, (baseline, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedFixedString128Delta(
        &mut self,
        baseline: crate::Unity::Collections::FixedString128Bytes,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString128Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::FixedString128Bytes,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), crate::Unity::Collections::FixedString128Bytes, 2usize>(
                        "ReadPackedFixedString128Delta",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedFixedString128Delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString128Bytes =
            unsafe { cordl_method_info.invoke_unchecked(self, (baseline, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedFixedString32Delta(
        &mut self,
        baseline: crate::Unity::Collections::FixedString32Bytes,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString32Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::FixedString32Bytes,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), crate::Unity::Collections::FixedString32Bytes, 2usize>(
                        "ReadPackedFixedString32Delta",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedFixedString32Delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString32Bytes =
            unsafe { cordl_method_info.invoke_unchecked(self, (baseline, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedFixedString4096Delta(
        &mut self,
        baseline: crate::Unity::Collections::FixedString4096Bytes,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString4096Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::FixedString4096Bytes,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), crate::Unity::Collections::FixedString4096Bytes, 2usize>(
                        "ReadPackedFixedString4096Delta",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedFixedString4096Delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString4096Bytes =
            unsafe { cordl_method_info.invoke_unchecked(self, (baseline, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedFixedString512Delta(
        &mut self,
        baseline: crate::Unity::Collections::FixedString512Bytes,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString512Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::FixedString512Bytes,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), crate::Unity::Collections::FixedString512Bytes, 2usize>(
                        "ReadPackedFixedString512Delta",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedFixedString512Delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString512Bytes =
            unsafe { cordl_method_info.invoke_unchecked(self, (baseline, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedFixedString64Delta(
        &mut self,
        baseline: crate::Unity::Collections::FixedString64Bytes,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::FixedString64Bytes> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::FixedString64Bytes,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), crate::Unity::Collections::FixedString64Bytes, 2usize>(
                        "ReadPackedFixedString64Delta",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedFixedString64Delta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::FixedString64Bytes =
            unsafe { cordl_method_info.invoke_unchecked(self, (baseline, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedFixedStringDelta(
        &mut self,
        data: crate::Unity::Collections::NativeArray_1<u8>,
        baseData: crate::Unity::Collections::NativeArray_1<u8>,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<u8>,
                        crate::Unity::Collections::NativeArray_1<u8>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), u16, 3usize>("ReadPackedFixedStringDelta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedFixedStringDelta",
                            3usize
                        )
                    })
            });
        let __cordl_ret: u16 =
            unsafe { cordl_method_info.invoke_unchecked(self, (data, baseData, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedFixedStringDeltaInternal(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        maxLength: i32,
        baseData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        baseLength: u16,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        i32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        u16,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), u16, 5usize>("ReadPackedFixedStringDeltaInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedFixedStringDeltaInternal",
                            5usize
                        )
                    })
            });
        let __cordl_ret: u16 = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (data, maxLength, baseData, baseLength, model))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedFloat(
        &mut self,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Collections::StreamCompressionModel,
                    >), f32, 1usize>("ReadPackedFloat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedFloat",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, (model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedFloatDelta(
        &mut self,
        baseline: f32,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        f32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), f32, 2usize>("ReadPackedFloatDelta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedFloatDelta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (baseline, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedInt(
        &mut self,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Collections::StreamCompressionModel,
                    >), i32, 1usize>("ReadPackedInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedInt",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedIntDelta(
        &mut self,
        baseline: i32,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), i32, 2usize>("ReadPackedIntDelta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedIntDelta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (baseline, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedLong(
        &mut self,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Collections::StreamCompressionModel,
                    >), i64, 1usize>("ReadPackedLong")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedLong",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { cordl_method_info.invoke_unchecked(self, (model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedLongDelta(
        &mut self,
        baseline: i64,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i64,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), i64, 2usize>("ReadPackedLongDelta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedLongDelta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i64 =
            unsafe { cordl_method_info.invoke_unchecked(self, (baseline, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedUInt(
        &mut self,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Collections::StreamCompressionModel,
                    >), u32, 1usize>("ReadPackedUInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedUInt",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, (model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedUIntDelta(
        &mut self,
        baseline: u32,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), u32, 2usize>("ReadPackedUIntDelta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedUIntDelta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (baseline, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedUIntInternal(
        &mut self,
        maxSymbolLength: i32,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), u32, 2usize>("ReadPackedUIntInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedUIntInternal",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (maxSymbolLength, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedULong(
        &mut self,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Collections::StreamCompressionModel,
                    >), u64, 1usize>("ReadPackedULong")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedULong",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked(self, (model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPackedULongDelta(
        &mut self,
        baseline: u64,
        model: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::StreamCompressionModel>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        u64,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::StreamCompressionModel,
                        >,
                    ), u64, 2usize>("ReadPackedULongDelta")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadPackedULongDelta",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u64 =
            unsafe { cordl_method_info.invoke_unchecked(self, (baseline, model))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadRawBits(&mut self, numbits: i32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), u32, 1usize>("ReadRawBits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadRawBits",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, (numbits))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadRawBitsInternal(&mut self, numbits: i32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), u32, 1usize>("ReadRawBitsInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadRawBitsInternal",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, (numbits))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadShort(&mut self) -> quest_hook::libil2cpp::Result<i16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i16, 0usize>("ReadShort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadShort",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i16 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadShortNetworkByteOrder(&mut self) -> quest_hook::libil2cpp::Result<i16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i16, 0usize>("ReadShortNetworkByteOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadShortNetworkByteOrder",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i16 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUInt(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("ReadUInt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadUInt",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUIntNetworkByteOrder(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("ReadUIntNetworkByteOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadUIntNetworkByteOrder",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadULong(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u64, 0usize>("ReadULong")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadULong",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUShort(&mut self) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u16, 0usize>("ReadUShort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadUShort",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u16 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadUShortNetworkByteOrder(&mut self) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u16, 0usize>("ReadUShortNetworkByteOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReadUShortNetworkByteOrder",
                            0usize
                        )
                    })
            });
        let __cordl_ret: u16 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SeekSet(
        &mut self,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("SeekSet")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SeekSet",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pos))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        array: crate::Unity::Collections::NativeArray_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<u8>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (array))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_HasFailedReads(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_HasFailedReads")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_HasFailedReads",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCreated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsCreated")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_IsCreated",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsLittleEndian() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_IsLittleEndian")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_IsLittleEndian",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_Length")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Length",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader+Context")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct DataStreamReader_Context {
    pub m_ReadByteIndex: i32,
    pub m_BitIndex: i32,
    pub m_BitBuffer: u64,
    pub m_FailedReads: i32,
}
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader+Context")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::DataStreamReader_Context {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "DataStreamReader/Context";
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
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader+Context")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Unity::Collections::DataStreamReader_Context
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader+Context")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Unity::Collections::DataStreamReader_Context
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
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader+Context")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Unity::Collections::DataStreamReader_Context
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
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader+Context")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Collections::DataStreamReader_Context {
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
#[cfg(feature = "cordl_class_Unity+Collections+DataStreamReader+Context")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::DataStreamReader_Context
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+DataStreamReader+Context")]
impl crate::Unity::Collections::DataStreamReader_Context {}
