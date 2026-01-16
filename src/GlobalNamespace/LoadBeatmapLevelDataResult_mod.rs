#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoadBeatmapLevelDataResult {
    pub errorType: crate::System::Nullable_1<
        crate::GlobalNamespace::LoadBeatmapLevelDataResult_ErrorType,
    >,
    pub beatmapLevelData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatmapLevelData,
    >,
}
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LoadBeatmapLevelDataResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LoadBeatmapLevelDataResult";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::LoadBeatmapLevelDataResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::LoadBeatmapLevelDataResult {
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
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::LoadBeatmapLevelDataResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::LoadBeatmapLevelDataResult {
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
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::LoadBeatmapLevelDataResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LoadBeatmapLevelDataResult")]
impl crate::GlobalNamespace::LoadBeatmapLevelDataResult {
    #[cfg(feature = "LoadBeatmapLevelDataResult+ErrorType")]
    pub type ErrorType = crate::GlobalNamespace::LoadBeatmapLevelDataResult_ErrorType;
    pub fn FromValue(
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::LoadBeatmapLevelDataResult,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IBeatmapLevelData,
                        >),
                        crate::GlobalNamespace::LoadBeatmapLevelDataResult,
                        1usize,
                    >("FromValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromValue", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::LoadBeatmapLevelDataResult = unsafe {
            cordl_method_info.invoke_unchecked((), (beatmapLevelData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Success(
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::LoadBeatmapLevelDataResult,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IBeatmapLevelData,
                        >),
                        crate::GlobalNamespace::LoadBeatmapLevelDataResult,
                        1usize,
                    >("Success")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Success",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::LoadBeatmapLevelDataResult = unsafe {
            cordl_method_info.invoke_unchecked((), (beatmapLevelData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        errorType: crate::System::Nullable_1<
            crate::GlobalNamespace::LoadBeatmapLevelDataResult_ErrorType,
        >,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Nullable_1<
                                crate::GlobalNamespace::LoadBeatmapLevelDataResult_ErrorType,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatmapLevelData,
                            >,
                        ),
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (errorType, beatmapLevelData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_errorMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_errorMessage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_errorMessage", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isError(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isError")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_isError", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult+ErrorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoadBeatmapLevelDataResult_ErrorType {
    #[default]
    BeatmapLevelDataNotFound = 2i32,
    BeatmapLevelNotFoundInRepository = 0i32,
    BeatmapLevelNotOwned = 1i32,
}
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult+ErrorType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LoadBeatmapLevelDataResult_ErrorType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LoadBeatmapLevelDataResult/ErrorType";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult+ErrorType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::LoadBeatmapLevelDataResult_ErrorType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult+ErrorType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::LoadBeatmapLevelDataResult_ErrorType {
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
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult+ErrorType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::LoadBeatmapLevelDataResult_ErrorType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_LoadBeatmapLevelDataResult+ErrorType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::LoadBeatmapLevelDataResult_ErrorType {
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
