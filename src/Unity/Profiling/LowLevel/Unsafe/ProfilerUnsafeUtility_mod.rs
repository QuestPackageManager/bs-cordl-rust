#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct ProfilerUnsafeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerUnsafeUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Profiling.LowLevel.Unsafe";
    const CLASS_NAME: &'static str = "ProfilerUnsafeUtility";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
impl std::ops::Deref
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerUnsafeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
impl std::ops::DerefMut
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerUnsafeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
impl crate::Unity::Profiling::LowLevel::Unsafe::ProfilerUnsafeUtility {
    pub fn BeginSample(
        markerPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("BeginSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "BeginSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (markerPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateCategory__Unmanaged(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        colorIndex: crate::Unity::Profiling::ProfilerCategoryColor,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            crate::Unity::Profiling::ProfilerCategoryColor,
                        ),
                        u16,
                        3usize,
                    >("CreateCategory__Unmanaged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateCategory__Unmanaged", 3usize
                        )
                    })
            });
        let __cordl_ret: u16 = unsafe {
            method.invoke_unchecked((), (name, nameLen, colorIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateCounterValue__Unmanaged(
        counterPtr: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        categoryId: u16,
        flags: crate::Unity::Profiling::LowLevel::MarkerFlags,
        dataType: u8,
        dataUnit: u8,
        dataSize: i32,
        counterOptions: crate::Unity::Profiling::ProfilerCounterOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            u16,
                            crate::Unity::Profiling::LowLevel::MarkerFlags,
                            u8,
                            u8,
                            i32,
                            crate::Unity::Profiling::ProfilerCounterOptions,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        9usize,
                    >("CreateCounterValue__Unmanaged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateCounterValue__Unmanaged", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        counterPtr,
                        name,
                        nameLen,
                        categoryId,
                        flags,
                        dataType,
                        dataUnit,
                        dataSize,
                        counterOptions,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateMarker(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        categoryId: u16,
        flags: crate::Unity::Profiling::LowLevel::MarkerFlags,
        metadataCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            u16,
                            crate::Unity::Profiling::LowLevel::MarkerFlags,
                            i32,
                        ),
                        crate::System::IntPtr,
                        4usize,
                    >("CreateMarker")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateMarker", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), (name, categoryId, flags, metadataCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateMarker__Unmanaged(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        categoryId: u16,
        flags: crate::Unity::Profiling::LowLevel::MarkerFlags,
        metadataCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            u16,
                            crate::Unity::Profiling::LowLevel::MarkerFlags,
                            i32,
                        ),
                        crate::System::IntPtr,
                        5usize,
                    >("CreateMarker__Unmanaged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateMarker__Unmanaged", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method
                .invoke_unchecked((), (name, nameLen, categoryId, flags, metadataCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndSample(
        markerPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndSample")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "EndSample", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (markerPtr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCategoryDescription(
        categoryId: u16,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerCategoryDescription,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (u16),
                        crate::Unity::Profiling::LowLevel::Unsafe::ProfilerCategoryDescription,
                        1usize,
                    >("GetCategoryDescription")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetCategoryDescription", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Profiling::LowLevel::Unsafe::ProfilerCategoryDescription = unsafe {
            method.invoke_unchecked((), (categoryId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCategoryDescription_Injected(
        categoryId: u16,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Profiling::LowLevel::Unsafe::ProfilerCategoryDescription,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            u16,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Profiling::LowLevel::Unsafe::ProfilerCategoryDescription,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("GetCategoryDescription_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetCategoryDescription_Injected", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (categoryId, ret))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetMarkerMetadata__Unmanaged(
        markerPtr: crate::System::IntPtr,
        index: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nameLen: i32,
        _cordl_type: u8,
        unit: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::System::IntPtr,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            u8,
                            u8,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SetMarkerMetadata__Unmanaged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetMarkerMetadata__Unmanaged", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (markerPtr, index, name, nameLen, _cordl_type, unit),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Utf8ToString(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        charsLen: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        2usize,
                    >("Utf8ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Utf8ToString", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (chars, charsLen))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Profiling+LowLevel+Unsafe+ProfilerUnsafeUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Profiling::LowLevel::Unsafe::ProfilerUnsafeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
