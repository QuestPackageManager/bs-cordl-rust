#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProfilerRecorderHandle {
    padding: quest_hook::libil2cpp::ValueTypePadding<8usize>,
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Profiling.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "ProfilerRecorderHandle";
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
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
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
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
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
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
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
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerRecorderHandle")]
impl crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle {
    pub fn GetAvailable(
        outRecorderHandleList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GetAvailable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAvailable", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (outRecorderHandleList))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetByName(
        category: crate::Unity::Profiling::ProfilerCategory,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Profiling::ProfilerCategory,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
                        3usize,
                    >("GetByName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetByName", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle = unsafe {
            method.invoke_unchecked((), (category, name, nameLen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetByName_Unsafe(
        category: crate::Unity::Profiling::ProfilerCategory,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Profiling::ProfilerCategory,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
                        3usize,
                    >("GetByName_Unsafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetByName_Unsafe", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle = unsafe {
            method.invoke_unchecked((), (category, name, nameLen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetByName_Unsafe_Injected(
        category: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerCategory,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Profiling::ProfilerCategory,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("GetByName_Unsafe_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetByName_Unsafe_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (category, name, nameLen, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetByName__Unmanaged(
        category: crate::Unity::Profiling::ProfilerCategory,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Unity::Profiling::ProfilerCategory,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
                        3usize,
                    >("GetByName__Unmanaged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetByName__Unmanaged", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle = unsafe {
            method.invoke_unchecked((), (category, name, nameLen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetByName__Unmanaged_Injected(
        category: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::ProfilerCategory,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Profiling::ProfilerCategory,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("GetByName__Unmanaged_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetByName__Unmanaged_Injected", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (category, name, nameLen, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDescription(
        handle: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle),
                        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription,
                        1usize,
                    >("GetDescription")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDescription", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription = unsafe {
            method.invoke_unchecked((), (handle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDescriptionInternal(
        handle: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle),
                        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription,
                        1usize,
                    >("GetDescriptionInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDescriptionInternal", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription = unsafe {
            method.invoke_unchecked((), (handle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDescriptionInternal_Injected(
        handle: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
        >,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderHandle,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Profiling::LowLevel::Unsafe::ProfilerRecorderDescription,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("GetDescriptionInternal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDescriptionInternal_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (handle, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_Valid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Valid", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
