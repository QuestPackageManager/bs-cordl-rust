#[cfg(feature = "OVRTask")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRTask {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRTask")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTask {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTask";
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
#[cfg(feature = "OVRTask")]
impl std::ops::Deref for crate::GlobalNamespace::OVRTask {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRTask {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask")]
impl crate::GlobalNamespace::OVRTask {
    pub fn Create<TResult>(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Guid),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Create", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = unsafe {
            method.invoke_unchecked((), (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromGuid<TResult>(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Guid),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("FromGuid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FromGuid", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = unsafe {
            method.invoke_unchecked((), (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromRequest<TResult>(
        id: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (u64),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("FromRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FromRequest", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = unsafe {
            method.invoke_unchecked((), (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromResult<TResult>(
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (TResult),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("FromResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FromResult", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = unsafe {
            method.invoke_unchecked((), (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Get<TResult>(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Guid),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Get", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = unsafe {
            method.invoke_unchecked((), (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetExisting_Guid0<TResult>(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Guid),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("GetExisting")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetExisting", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = unsafe {
            method.invoke_unchecked((), (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetExisting_u64_1<TResult>(
        id: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (u64),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("GetExisting")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetExisting", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> = unsafe {
            method.invoke_unchecked((), (id))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetId(value: u64) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(u64), crate::System::Guid, 1usize>("GetId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetId", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Guid = unsafe {
            method.invoke_unchecked((), (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetResult_Guid0<TResult>(
        id: crate::System::Guid,
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (crate::System::Guid, TResult),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetResult", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (id, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetResult_u64_1<TResult>(
        id: u64,
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (u64, TResult),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetResult", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (id, result))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRTask")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
