#[cfg(feature = "LoadBeatmapLevelDataResult")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoadBeatmapLevelDataResult {
    pub isError: bool,
    pub beatmapLevelData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatmapLevelData,
    >,
}
#[cfg(feature = "LoadBeatmapLevelDataResult")]
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
#[cfg(feature = "LoadBeatmapLevelDataResult")]
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
#[cfg(feature = "LoadBeatmapLevelDataResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::LoadBeatmapLevelDataResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "LoadBeatmapLevelDataResult")]
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
#[cfg(feature = "LoadBeatmapLevelDataResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::LoadBeatmapLevelDataResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "LoadBeatmapLevelDataResult")]
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
    pub fn FromValue(
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::LoadBeatmapLevelDataResult,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                            Self::class(), "FromValue", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::LoadBeatmapLevelDataResult = unsafe {
            method.invoke_unchecked((), (beatmapLevelData))?
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
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                            Self::class(), "Success", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::LoadBeatmapLevelDataResult = unsafe {
            method.invoke_unchecked((), (beatmapLevelData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        isError: bool,
        beatmapLevelData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            bool,
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
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (isError, beatmapLevelData))?
        };
        Ok(__cordl_ret.into())
    }
}
