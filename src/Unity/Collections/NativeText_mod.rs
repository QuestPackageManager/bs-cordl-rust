#[cfg(feature = "cordl_class_Unity+Collections+NativeText")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct NativeText {
    pub m_Data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeText")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::NativeText {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "NativeText";
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Collections::NativeText {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeText")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Collections::NativeText {
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Collections::NativeText {
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Collections::NativeText {
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Collections::NativeText {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl crate::Unity::Collections::NativeText {
    #[cfg(feature = "Unity+Collections+NativeText+Enumerator")]
    pub type Enumerator = crate::Unity::Collections::NativeText_Enumerator;
    #[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
    pub type ReadOnly = crate::Unity::Collections::NativeText_ReadOnly;
    pub fn Add(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<u8>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Add")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Add",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn AsReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText_ReadOnly> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::NativeText_ReadOnly, 0usize>(
                        "AsReadOnly",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AsReadOnly",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText_ReadOnly =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckIndexInRange(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("CheckIndexInRange")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckIndexInRange",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckNull(
        dataPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckNull")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckNull", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (dataPtr))? };
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
    pub fn CheckWrite(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CheckWrite")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckWrite",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckWriteAndBumpSecondaryVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "CheckWriteAndBumpSecondaryVersion",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CheckWriteAndBumpSecondaryVersion",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_FixedString128Bytes5(
        &mut self,
        other: crate::Unity::Collections::FixedString128Bytes,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString128Bytes), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_FixedString32Bytes3(
        &mut self,
        other: crate::Unity::Collections::FixedString32Bytes,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString32Bytes), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_FixedString4096Bytes7(
        &mut self,
        other: crate::Unity::Collections::FixedString4096Bytes,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString4096Bytes), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_FixedString512Bytes6(
        &mut self,
        other: crate::Unity::Collections::FixedString512Bytes,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString512Bytes), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_FixedString64Bytes4(
        &mut self,
        other: crate::Unity::Collections::FixedString64Bytes,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString64Bytes), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Il2CppString2(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        i32,
                        1usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_NativeText0(
        &mut self,
        other: crate::Unity::Collections::NativeText,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::NativeText), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_NativeText_ReadOnly1(
        &mut self,
        other: crate::Unity::Collections::NativeText_ReadOnly,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::NativeText_ReadOnly), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_JobHandle1(
        &mut self,
        inputDeps: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Jobs::JobHandle),
                        crate::Unity::Jobs::JobHandle,
                        1usize,
                    >("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (inputDeps))? };
        Ok(__cordl_ret.into())
    }
    pub fn ElementAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<u8>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::ByRefMut<u8>, 1usize>("ElementAt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ElementAt",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<u8> =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_FixedString128Bytes5(
        &mut self,
        other: crate::Unity::Collections::FixedString128Bytes,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString128Bytes), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_FixedString32Bytes3(
        &mut self,
        other: crate::Unity::Collections::FixedString32Bytes,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString32Bytes), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_FixedString4096Bytes7(
        &mut self,
        other: crate::Unity::Collections::FixedString4096Bytes,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString4096Bytes), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_FixedString512Bytes6(
        &mut self,
        other: crate::Unity::Collections::FixedString512Bytes,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString512Bytes), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_FixedString64Bytes4(
        &mut self,
        other: crate::Unity::Collections::FixedString64Bytes,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString64Bytes), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject8(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppString2(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_NativeText0(
        &mut self,
        other: crate::Unity::Collections::NativeText,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::NativeText), bool, 1usize>("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_NativeText_ReadOnly1(
        &mut self,
        other: crate::Unity::Collections::NativeText_ReadOnly,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::NativeText_ReadOnly), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText_Enumerator> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::NativeText_Enumerator, 0usize>(
                        "GetEnumerator",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetEnumerator",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText_Enumerator =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafePtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("GetUnsafePtr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUnsafePtr", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ThrowCopyError(
        &mut self,
        error: crate::Unity::Collections::CopyError,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::CopyError,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), quest_hook::libil2cpp::Void, 2usize>("ThrowCopyError")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ThrowCopyError",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (error, source))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TryResize(
        &mut self,
        newLength: i32,
        clearOptions: crate::Unity::Collections::NativeArrayOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::Unity::Collections::NativeArrayOptions),
                        bool,
                        2usize,
                    >("TryResize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryResize", 2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (newLength, clearOptions))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Allocator4(
        &mut self,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::Allocator),
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
            unsafe { cordl_method_info.invoke_unchecked(self, (allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AllocatorManager_AllocatorHandle5(
        &mut self,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::AllocatorManager_AllocatorHandle),
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
            unsafe { cordl_method_info.invoke_unchecked(self, (allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_Allocator11(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString128Bytes>,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString128Bytes,
                        >,
                        crate::Unity::Collections::Allocator,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_Allocator13(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString512Bytes>,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString512Bytes,
                        >,
                        crate::Unity::Collections::Allocator,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_Allocator15(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString4096Bytes>,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString4096Bytes,
                        >,
                        crate::Unity::Collections::Allocator,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_Allocator7(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString32Bytes>,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString32Bytes,
                        >,
                        crate::Unity::Collections::Allocator,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_Allocator9(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString64Bytes>,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString64Bytes,
                        >,
                        crate::Unity::Collections::Allocator,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_AllocatorManager_AllocatorHandle10(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString128Bytes>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString128Bytes,
                        >,
                        crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_AllocatorManager_AllocatorHandle12(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString512Bytes>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString512Bytes,
                        >,
                        crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_AllocatorManager_AllocatorHandle14(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString4096Bytes>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString4096Bytes,
                        >,
                        crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_AllocatorManager_AllocatorHandle6(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString32Bytes>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString32Bytes,
                        >,
                        crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ByRefMut_AllocatorManager_AllocatorHandle8(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString64Bytes>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString64Bytes,
                        >,
                        crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Allocator0(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::Unity::Collections::Allocator,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_AllocatorManager_AllocatorHandle1(
        &mut self,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (source, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_Allocator2(
        &mut self,
        capacity: i32,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::Unity::Collections::Allocator),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (capacity, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_AllocatorManager_AllocatorHandle3(
        &mut self,
        capacity: i32,
        allocator: crate::Unity::Collections::AllocatorManager_AllocatorHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::Unity::Collections::AllocatorManager_AllocatorHandle,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (capacity, allocator))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Capacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_Capacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Capacity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsEmpty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_IsEmpty",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<u8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), u8, 1usize>("get_Item")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Item",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u8 = unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
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
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_Value")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Value", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_ByRefMut_ByRefMut0(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString32Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString32Bytes,
                        >,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_ByRefMut_ByRefMut1(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString64Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString64Bytes,
                        >,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_ByRefMut_ByRefMut2(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString128Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString128Bytes,
                        >,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_ByRefMut_ByRefMut3(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString512Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString512Bytes,
                        >,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_ByRefMut_ByRefMut4(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString4096Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString4096Bytes,
                        >,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_ByRefMut_ByRefMut0(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString32Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString32Bytes,
                        >,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_ByRefMut_ByRefMut1(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString64Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString64Bytes,
                        >,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_ByRefMut_ByRefMut2(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString128Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString128Bytes,
                        >,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_ByRefMut_ByRefMut3(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString512Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString512Bytes,
                        >,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_ByRefMut_ByRefMut4(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString4096Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString4096Bytes,
                        >,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Capacity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_Capacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_Capacity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, u8), quest_hook::libil2cpp::Void, 2usize>("set_Item")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_Item",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Length(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_Length")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_Length",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::FixedString128Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::Unity::Collections::FixedString128Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::FixedString128Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::FixedString128Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::FixedString32Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::Unity::Collections::FixedString32Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::FixedString32Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::FixedString32Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::FixedString4096Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::Unity::Collections::FixedString4096Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::FixedString4096Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::FixedString4096Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::FixedString512Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::Unity::Collections::FixedString512Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::FixedString512Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::FixedString512Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::FixedString64Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::Unity::Collections::FixedString64Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::FixedString64Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::FixedString64Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::NativeText>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(&self) -> &crate::System::IComparable_1<crate::Unity::Collections::NativeText> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::NativeText>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::NativeText> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl
    AsRef<
        crate::System::IComparable_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl
    AsMut<
        crate::System::IComparable_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IDisposable> for crate::Unity::Collections::NativeText {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IDisposable> for crate::Unity::Collections::NativeText {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::FixedString128Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Collections::FixedString128Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::FixedString128Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::FixedString128Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::FixedString32Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Collections::FixedString32Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::FixedString32Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::FixedString32Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::FixedString4096Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Collections::FixedString4096Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::FixedString4096Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::FixedString4096Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::FixedString512Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Collections::FixedString512Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::FixedString512Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::FixedString512Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::FixedString64Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Collections::FixedString64Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::FixedString64Bytes>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::FixedString64Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::NativeText>>
    for crate::Unity::Collections::NativeText
{
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::Unity::Collections::NativeText> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::NativeText>>
    for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::NativeText> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl
    AsRef<
        crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>,
    > for crate::Unity::Collections::NativeText
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl
    AsMut<
        crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>,
    > for crate::Unity::Collections::NativeText
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::Unity::Collections::IIndexable_1<u8>> for crate::Unity::Collections::NativeText {
    fn as_ref(&self) -> &crate::Unity::Collections::IIndexable_1<u8> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::Unity::Collections::IIndexable_1<u8>> for crate::Unity::Collections::NativeText {
    fn as_mut(&mut self) -> &mut crate::Unity::Collections::IIndexable_1<u8> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::Unity::Collections::INativeDisposable> for crate::Unity::Collections::NativeText {
    fn as_ref(&self) -> &crate::Unity::Collections::INativeDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::Unity::Collections::INativeDisposable> for crate::Unity::Collections::NativeText {
    fn as_mut(&mut self) -> &mut crate::Unity::Collections::INativeDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::Unity::Collections::INativeList_1<u8>> for crate::Unity::Collections::NativeText {
    fn as_ref(&self) -> &crate::Unity::Collections::INativeList_1<u8> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::Unity::Collections::INativeList_1<u8>> for crate::Unity::Collections::NativeText {
    fn as_mut(&mut self) -> &mut crate::Unity::Collections::INativeList_1<u8> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsRef<crate::Unity::Collections::IUTF8Bytes> for crate::Unity::Collections::NativeText {
    fn as_ref(&self) -> &crate::Unity::Collections::IUTF8Bytes {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText")]
impl AsMut<crate::Unity::Collections::IUTF8Bytes> for crate::Unity::Collections::NativeText {
    fn as_mut(&mut self) -> &mut crate::Unity::Collections::IUTF8Bytes {
        todo!()
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+Enumerator")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct NativeText_Enumerator {
    pub target: crate::Unity::Collections::NativeText_ReadOnly,
    pub offset: i32,
    pub current: crate::Unity::Collections::Unicode_Rune,
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+Enumerator")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::NativeText_Enumerator {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "NativeText/Enumerator";
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+Enumerator")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Collections::NativeText_Enumerator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+Enumerator")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Collections::NativeText_Enumerator {
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+Enumerator")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Collections::NativeText_Enumerator {
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+Enumerator")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Collections::NativeText_Enumerator {
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+Enumerator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Unity::Collections::NativeText_Enumerator
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+NativeText+Enumerator")]
impl crate::Unity::Collections::NativeText_Enumerator {
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MoveNext",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("System.Collections.IEnumerator.get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerator.get_Current", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_NativeText0(
        &mut self,
        source: crate::Unity::Collections::NativeText,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeText),
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
            unsafe { cordl_method_info.invoke_unchecked(self, (source))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_NativeText_ReadOnly1(
        &mut self,
        source: crate::Unity::Collections::NativeText_ReadOnly,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeText_ReadOnly),
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
            unsafe { cordl_method_info.invoke_unchecked(self, (source))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::Unicode_Rune> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::Unicode_Rune, 0usize>(
                        "get_Current",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Current",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::Unicode_Rune =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Collections+NativeText+Enumerator")]
impl
    AsRef<
        crate::System::Collections::Generic::IEnumerator_1<crate::Unity::Collections::Unicode_Rune>,
    > for crate::Unity::Collections::NativeText_Enumerator
{
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerator_1<crate::Unity::Collections::Unicode_Rune>
    {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+Enumerator")]
impl
    AsMut<
        crate::System::Collections::Generic::IEnumerator_1<crate::Unity::Collections::Unicode_Rune>,
    > for crate::Unity::Collections::NativeText_Enumerator
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<
        crate::Unity::Collections::Unicode_Rune,
    > {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+Enumerator")]
impl AsRef<crate::System::Collections::IEnumerator>
    for crate::Unity::Collections::NativeText_Enumerator
{
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+Enumerator")]
impl AsMut<crate::System::Collections::IEnumerator>
    for crate::Unity::Collections::NativeText_Enumerator
{
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+Enumerator")]
impl AsRef<crate::System::IDisposable> for crate::Unity::Collections::NativeText_Enumerator {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+Enumerator")]
impl AsMut<crate::System::IDisposable> for crate::Unity::Collections::NativeText_Enumerator {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+ReadOnly")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct NativeText_ReadOnly {
    pub m_Data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Collections::NativeText_ReadOnly {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Collections";
    const CLASS_NAME: &'static str = "NativeText/ReadOnly";
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Collections::NativeText_ReadOnly {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Collections::NativeText_ReadOnly {
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Collections::NativeText_ReadOnly {
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+ReadOnly")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Collections::NativeText_ReadOnly {
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
#[cfg(feature = "cordl_class_Unity+Collections+NativeText+ReadOnly")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Collections::NativeText_ReadOnly {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl crate::Unity::Collections::NativeText_ReadOnly {
    pub fn CheckNull(
        dataPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("CheckNull")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckNull", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (dataPtr))? };
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
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_FixedString128Bytes5(
        &mut self,
        other: crate::Unity::Collections::FixedString128Bytes,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString128Bytes), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_FixedString32Bytes3(
        &mut self,
        other: crate::Unity::Collections::FixedString32Bytes,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString32Bytes), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_FixedString4096Bytes7(
        &mut self,
        other: crate::Unity::Collections::FixedString4096Bytes,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString4096Bytes), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_FixedString512Bytes6(
        &mut self,
        other: crate::Unity::Collections::FixedString512Bytes,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString512Bytes), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_FixedString64Bytes4(
        &mut self,
        other: crate::Unity::Collections::FixedString64Bytes,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString64Bytes), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Il2CppString0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        i32,
                        1usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_NativeText2(
        &mut self,
        other: crate::Unity::Collections::NativeText,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::NativeText), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_NativeText_ReadOnly1(
        &mut self,
        other: crate::Unity::Collections::NativeText_ReadOnly,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::NativeText_ReadOnly), i32, 1usize>(
                        "CompareTo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CompareTo",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ElementAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<u8>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::ByRefMut<u8>, 1usize>("ElementAt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ElementAt",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<u8> =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_FixedString128Bytes5(
        &mut self,
        other: crate::Unity::Collections::FixedString128Bytes,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString128Bytes), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_FixedString32Bytes3(
        &mut self,
        other: crate::Unity::Collections::FixedString32Bytes,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString32Bytes), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_FixedString4096Bytes7(
        &mut self,
        other: crate::Unity::Collections::FixedString4096Bytes,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString4096Bytes), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_FixedString512Bytes6(
        &mut self,
        other: crate::Unity::Collections::FixedString512Bytes,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString512Bytes), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_FixedString64Bytes4(
        &mut self,
        other: crate::Unity::Collections::FixedString64Bytes,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::FixedString64Bytes), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject8(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppString0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_NativeText2(
        &mut self,
        other: crate::Unity::Collections::NativeText,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::NativeText), bool, 1usize>("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_NativeText_ReadOnly1(
        &mut self,
        other: crate::Unity::Collections::NativeText_ReadOnly,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::NativeText_ReadOnly), bool, 1usize>(
                        "Equals",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn ErrorWrite(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ErrorWrite")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ErrorWrite",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeText_Enumerator> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::NativeText_Enumerator, 0usize>(
                        "GetEnumerator",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetEnumerator",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeText_Enumerator =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetHashCode",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetUnsafePtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("GetUnsafePtr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetUnsafePtr", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TryResize(
        &mut self,
        newLength: i32,
        clearOptions: crate::Unity::Collections::NativeArrayOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::Unity::Collections::NativeArrayOptions),
                        bool,
                        2usize,
                    >("TryResize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryResize", 2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (newLength, clearOptions))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
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
            unsafe { cordl_method_info.invoke_unchecked(self, (text))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Capacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_Capacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Capacity",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsEmpty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_IsEmpty",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<u8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), u8, 1usize>("get_Item")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Item",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u8 = unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
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
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_Value")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Value", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_ByRefMut_ByRefMut0(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText_ReadOnly>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString32Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeText_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString32Bytes,
                        >,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_ByRefMut_ByRefMut1(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText_ReadOnly>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString64Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeText_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString64Bytes,
                        >,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_ByRefMut_ByRefMut2(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText_ReadOnly>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString128Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeText_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString128Bytes,
                        >,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_ByRefMut_ByRefMut3(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText_ReadOnly>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString512Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeText_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString512Bytes,
                        >,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_ByRefMut_ByRefMut4(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText_ReadOnly>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString4096Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeText_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString4096Bytes,
                        >,
                    ), bool, 2usize>("op_Equality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Equality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_ByRefMut_ByRefMut0(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText_ReadOnly>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString32Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeText_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString32Bytes,
                        >,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_ByRefMut_ByRefMut1(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText_ReadOnly>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString64Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeText_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString64Bytes,
                        >,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_ByRefMut_ByRefMut2(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText_ReadOnly>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString128Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeText_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString128Bytes,
                        >,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_ByRefMut_ByRefMut3(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText_ReadOnly>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString512Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeText_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString512Bytes,
                        >,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_ByRefMut_ByRefMut4(
        a: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeText_ReadOnly>,
        b: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::FixedString4096Bytes>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeText_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::FixedString4096Bytes,
                        >,
                    ), bool, 2usize>("op_Inequality")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "op_Inequality",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Capacity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_Capacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_Capacity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_IsEmpty(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("set_IsEmpty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_IsEmpty",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, u8), quest_hook::libil2cpp::Void, 2usize>("set_Item")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_Item",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Length(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_Length")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_Length",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::FixedString128Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::Unity::Collections::FixedString128Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::FixedString128Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::FixedString128Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::FixedString32Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::Unity::Collections::FixedString32Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::FixedString32Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::FixedString32Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::FixedString4096Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::Unity::Collections::FixedString4096Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::FixedString4096Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::FixedString4096Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::FixedString512Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::Unity::Collections::FixedString512Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::FixedString512Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::FixedString512Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::FixedString64Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::Unity::Collections::FixedString64Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::FixedString64Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::FixedString64Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IComparable_1<crate::Unity::Collections::NativeText>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(&self) -> &crate::System::IComparable_1<crate::Unity::Collections::NativeText> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IComparable_1<crate::Unity::Collections::NativeText>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::Unity::Collections::NativeText> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl
    AsRef<
        crate::System::IComparable_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl
    AsMut<
        crate::System::IComparable_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::FixedString128Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Collections::FixedString128Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::FixedString128Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::FixedString128Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::FixedString32Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Collections::FixedString32Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::FixedString32Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::FixedString32Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::FixedString4096Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Collections::FixedString4096Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::FixedString4096Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::FixedString4096Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::FixedString512Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Collections::FixedString512Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::FixedString512Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::FixedString512Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::FixedString64Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Collections::FixedString64Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::FixedString64Bytes>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::FixedString64Bytes> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Collections::NativeText>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::Unity::Collections::NativeText> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Collections::NativeText>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Collections::NativeText> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl
    AsRef<
        crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>,
    > for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl
    AsMut<
        crate::System::IEquatable_1<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>,
    > for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::Unity::Collections::IIndexable_1<u8>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(&self) -> &crate::Unity::Collections::IIndexable_1<u8> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::Unity::Collections::IIndexable_1<u8>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(&mut self) -> &mut crate::Unity::Collections::IIndexable_1<u8> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::Unity::Collections::INativeList_1<u8>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(&self) -> &crate::Unity::Collections::INativeList_1<u8> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::Unity::Collections::INativeList_1<u8>>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(&mut self) -> &mut crate::Unity::Collections::INativeList_1<u8> {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsRef<crate::Unity::Collections::IUTF8Bytes>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_ref(&self) -> &crate::Unity::Collections::IUTF8Bytes {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeText+ReadOnly")]
impl AsMut<crate::Unity::Collections::IUTF8Bytes>
    for crate::Unity::Collections::NativeText_ReadOnly
{
    fn as_mut(&mut self) -> &mut crate::Unity::Collections::IUTF8Bytes {
        todo!()
    }
}
