#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProfilerRecorder {
    pub handle: u64,
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Profiling::ProfilerRecorder {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Profiling";
    const CLASS_NAME: &'static str = "ProfilerRecorder";
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
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Profiling::ProfilerRecorder {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Profiling::ProfilerRecorder {
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
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Profiling::ProfilerRecorder {
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
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Profiling::ProfilerRecorder {
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
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::ProfilerRecorder {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
impl crate::Unity::Profiling::ProfilerRecorder {
    #[cfg(feature = "Unity+Profiling+ProfilerRecorder+ControlOptions")]
    pub type ControlOptions = crate::Unity::Profiling::ProfilerRecorder_ControlOptions;
    pub fn CheckInitializedAndThrow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CheckInitializedAndThrow")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckInitializedAndThrow", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Control(
        handle: crate::Unity::Profiling::ProfilerRecorder,
        options: crate::Unity::Profiling::ProfilerRecorder_ControlOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Profiling::ProfilerRecorder,
                            crate::Unity::Profiling::ProfilerRecorder_ControlOptions,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Control")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Control", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Control_Injected(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerRecorder,
        >,
        options: crate::Unity::Profiling::ProfilerRecorder_ControlOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Profiling::ProfilerRecorder,
                            >,
                            crate::Unity::Profiling::ProfilerRecorder_ControlOptions,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Control_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Control_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        statHandle: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
        maxSampleCount: i32,
        options: crate::Unity::Profiling::ProfilerRecorderOptions,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerRecorder> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
                            i32,
                            crate::Unity::Profiling::ProfilerRecorderOptions,
                        ),
                        crate::Unity::Profiling::ProfilerRecorder,
                        3usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Create", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::ProfilerRecorder = unsafe {
            method.invoke_unchecked((), (statHandle, maxSampleCount, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Create_Injected(
        statHandle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
        >,
        maxSampleCount: i32,
        options: crate::Unity::Profiling::ProfilerRecorderOptions,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Profiling::ProfilerRecorder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
                            >,
                            i32,
                            crate::Unity::Profiling::ProfilerRecorderOptions,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Profiling::ProfilerRecorder,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Create_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Create_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (statHandle, maxSampleCount, options, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Dispose", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLastValue(
        handle: crate::Unity::Profiling::ProfilerRecorder,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Profiling::ProfilerRecorder),
                        i64,
                        1usize,
                    >("GetLastValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetLastValue", 1usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLastValue_Injected(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerRecorder,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Profiling::ProfilerRecorder,
                        >),
                        i64,
                        1usize,
                    >("GetLastValue_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetLastValue_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetValid(
        handle: crate::Unity::Profiling::ProfilerRecorder,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Profiling::ProfilerRecorder),
                        bool,
                        1usize,
                    >("GetValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetValid", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetValid_Injected(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerRecorder,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Profiling::ProfilerRecorder,
                        >),
                        bool,
                        1usize,
                    >("GetValid_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetValid_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetValueUnitType(
        handle: crate::Unity::Profiling::ProfilerRecorder,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerMarkerDataUnit> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::Unity::Profiling::ProfilerRecorder),
                        crate::Unity::Profiling::ProfilerMarkerDataUnit,
                        1usize,
                    >("GetValueUnitType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetValueUnitType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::ProfilerMarkerDataUnit = unsafe {
            method.invoke_unchecked((), (handle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetValueUnitType_Injected(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerRecorder,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerMarkerDataUnit> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Profiling::ProfilerRecorder,
                        >),
                        crate::Unity::Profiling::ProfilerMarkerDataUnit,
                        1usize,
                    >("GetValueUnitType_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetValueUnitType_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::ProfilerMarkerDataUnit = unsafe {
            method.invoke_unchecked((), (handle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartNew(
        category: crate::Unity::Profiling::ProfilerCategory,
        statName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        capacity: i32,
        options: crate::Unity::Profiling::ProfilerRecorderOptions,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerRecorder> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Unity::Profiling::ProfilerCategory,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            crate::Unity::Profiling::ProfilerRecorderOptions,
                        ),
                        crate::Unity::Profiling::ProfilerRecorder,
                        4usize,
                    >("StartNew")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "StartNew", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::ProfilerRecorder = unsafe {
            method.invoke_unchecked((), (category, statName, capacity, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        category: crate::Unity::Profiling::ProfilerCategory,
        statName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        statNameLen: i32,
        capacity: i32,
        options: crate::Unity::Profiling::ProfilerRecorderOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Unity::Profiling::ProfilerCategory,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            i32,
                            crate::Unity::Profiling::ProfilerRecorderOptions,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (category, statName, statNameLen, capacity, options),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_LastValue(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i64, 0usize>("get_LastValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_LastValue", 0usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UnitType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Profiling::ProfilerMarkerDataUnit> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::Unity::Profiling::ProfilerMarkerDataUnit,
                        0usize,
                    >("get_UnitType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_UnitType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::ProfilerMarkerDataUnit = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), bool, 0usize>("get_Valid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Valid", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
impl AsRef<crate::System::IDisposable> for crate::Unity::Profiling::ProfilerRecorder {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder")]
impl AsMut<crate::System::IDisposable> for crate::Unity::Profiling::ProfilerRecorder {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder+ControlOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProfilerRecorder_ControlOptions {
    #[default]
    Release = 4i32,
    Reset = 2i32,
    SetFilterToCurrentThread = 5i32,
    SetToCollectFromAllThreads = 6i32,
    Start = 0i32,
    Stop = 1i32,
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder+ControlOptions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Profiling::ProfilerRecorder_ControlOptions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Profiling";
    const CLASS_NAME: &'static str = "ProfilerRecorder/ControlOptions";
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
#[cfg(feature = "Unity+Profiling+ProfilerRecorder+ControlOptions")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Profiling::ProfilerRecorder_ControlOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorder+ControlOptions")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Profiling::ProfilerRecorder_ControlOptions {
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
#[cfg(feature = "Unity+Profiling+ProfilerRecorder+ControlOptions")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Profiling::ProfilerRecorder_ControlOptions {
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
#[cfg(feature = "Unity+Profiling+ProfilerRecorder+ControlOptions")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Profiling::ProfilerRecorder_ControlOptions {
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
