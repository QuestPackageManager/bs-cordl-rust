#[cfg(feature = "cordl_class_OVRTask")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVRTask {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRTask")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTask {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTask";
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
    pub const HashModifier1: u64 = 3573116690164977347u64;
    pub const HashModifier2: u64 = 10871156337175269513u64;
    #[cfg(feature = "OVRTask+Builder")]
    pub type Builder = crate::GlobalNamespace::OVRTask_Builder;
    #[cfg(feature = "OVRTask+MultiTaskData_1")]
    pub type MultiTaskData_1<T: quest_hook::libil2cpp::Type> =
        crate::GlobalNamespace::OVRTask_MultiTaskData_1<T>;
    #[cfg(feature = "OVRTask+MultiTaskData_2")]
    pub type MultiTaskData_2<T1: quest_hook::libil2cpp::Type, T2: quest_hook::libil2cpp::Type> =
        crate::GlobalNamespace::OVRTask_MultiTaskData_2<T1, T2>;
    #[cfg(feature = "OVRTask+MultiTaskData_3")]
    pub type MultiTaskData_3<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::OVRTask_MultiTaskData_3<T1, T2, T3>;
    #[cfg(feature = "OVRTask+MultiTaskData_4")]
    pub type MultiTaskData_4<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::OVRTask_MultiTaskData_4<T1, T2, T3, T4>;
    #[cfg(feature = "OVRTask+MultiTaskData_5")]
    pub type MultiTaskData_5<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::OVRTask_MultiTaskData_5<T1, T2, T3, T4, T5>;
    #[cfg(feature = "OVRTask+MultiTaskData_6")]
    pub type MultiTaskData_6<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::OVRTask_MultiTaskData_6<T1, T2, T3, T4, T5, T6>;
    #[cfg(feature = "OVRTask+MultiTaskData_7")]
    pub type MultiTaskData_7<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::OVRTask_MultiTaskData_7<T1, T2, T3, T4, T5, T6, T7>;
    #[cfg(feature = "OVRTask+MultiTaskData_8")]
    pub type MultiTaskData_8<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
        T8: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::OVRTask_MultiTaskData_8<T1, T2, T3, T4, T5, T6, T7, T8>;
    pub fn Build_OVRPlugin_Result1(
        result: crate::GlobalNamespace::OVRPlugin_Result,
        requestId: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_Builder> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_Result, u64),
                        crate::GlobalNamespace::OVRTask_Builder,
                        2usize,
                    >("Build")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Build",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_Builder =
            unsafe { cordl_method_info.invoke_unchecked((), (result, requestId))? };
        Ok(__cordl_ret.into())
    }
    pub fn Build_OVRPlugin_Result_OVRPlugin_EventType2(
        result: crate::GlobalNamespace::OVRPlugin_Result,
        requestId: u64,
        eventType: crate::GlobalNamespace::OVRPlugin_EventType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_Builder> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRPlugin_Result,
                        u64,
                        crate::GlobalNamespace::OVRPlugin_EventType,
                    ), crate::GlobalNamespace::OVRTask_Builder, 3usize>("Build")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Build",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_Builder =
            unsafe { cordl_method_info.invoke_unchecked((), (result, requestId, eventType))? };
        Ok(__cordl_ret.into())
    }
    pub fn Build__cordl_bool0(
        success: bool,
        requestId: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_Builder> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool, u64),
                        crate::GlobalNamespace::OVRTask_Builder,
                        2usize,
                    >("Build")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Build",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_Builder =
            unsafe { cordl_method_info.invoke_unchecked((), (success, requestId))? };
        Ok(__cordl_ret.into())
    }
    pub fn Create<TResult>(
        taskId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Guid),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> =
            unsafe { cordl_method_info.invoke_unchecked((), (taskId))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromGuid<TResult>(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Guid),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("FromGuid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromGuid", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> =
            unsafe { cordl_method_info.invoke_unchecked((), (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromRequest_OVRPlugin_EventType1<TResult>(
        id: u64,
        eventType: crate::GlobalNamespace::OVRPlugin_EventType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64, crate::GlobalNamespace::OVRPlugin_EventType),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        2usize,
                    >("FromRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromRequest", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> =
            unsafe { cordl_method_info.invoke_unchecked((), (id, eventType))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromRequest_u64_0<TResult>(
        id: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("FromRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromRequest", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> =
            unsafe { cordl_method_info.invoke_unchecked((), (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromResult<TResult>(
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (TResult),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("FromResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FromResult", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> =
            unsafe { cordl_method_info.invoke_unchecked((), (result))? };
        Ok(__cordl_ret.into())
    }
    pub fn Get<TResult>(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Guid),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Get",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> =
            unsafe { cordl_method_info.invoke_unchecked((), (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExisting_Guid0<TResult>(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Guid),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("GetExisting")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetExisting", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> =
            unsafe { cordl_method_info.invoke_unchecked((), (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExisting_u64_1<TResult>(
        id: u64,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64),
                        crate::GlobalNamespace::OVRTask_1<TResult>,
                        1usize,
                    >("GetExisting")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetExisting", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> =
            unsafe { cordl_method_info.invoke_unchecked((), (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetIdParts(
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<crate::System::ValueTuple_2<u64, u64>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Guid),
                        crate::System::ValueTuple_2<u64, u64>,
                        1usize,
                    >("GetIdParts")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetIdParts", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<u64, u64> =
            unsafe { cordl_method_info.invoke_unchecked((), (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetId_Guid3(value: crate::System::Guid) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::System::Guid), u64, 1usize>("GetId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetId",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetId_u64_2(value: u64) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64), crate::System::Guid, 1usize>("GetId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetId",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Guid =
            unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetId_u64_OVRPlugin_EventType1(
        handle: u64,
        eventType: crate::GlobalNamespace::OVRPlugin_EventType,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u64, crate::GlobalNamespace::OVRPlugin_EventType),
                        crate::System::Guid,
                        2usize,
                    >("GetId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "GetId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Guid =
            unsafe { cordl_method_info.invoke_unchecked((), (handle, eventType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetId_u64_u64_0(
        part1: u64,
        part2: u64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64, u64), crate::System::Guid, 2usize>("GetId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Guid =
            unsafe { cordl_method_info.invoke_unchecked((), (part1, part2))? };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterType<TResult>() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("RegisterType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RegisterType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetResult_Guid0<TResult>(
        id: crate::System::Guid,
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::Guid, TResult),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetResult", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (id, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetResult_u64_1<TResult>(
        id: u64,
        result: TResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u64, TResult), quest_hook::libil2cpp::Void, 2usize>(
                        "SetResult",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetResult",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (id, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPendingTask_Guid0<TResult>(
        id: crate::System::Guid,
        task: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRTask_1<TResult>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Guid,
                        quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRTask_1<TResult>>,
                    ), bool, 2usize>("TryGetPendingTask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetPendingTask",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (id, task))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetPendingTask_u64_1<TResult>(
        id: u64,
        task: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRTask_1<TResult>>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u64,
                        quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRTask_1<TResult>>,
                    ), bool, 2usize>("TryGetPendingTask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetPendingTask",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (id, task))? };
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_IEnumerable_1_0<TResult>(
        tasks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::OVRTask_1<TResult>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TResult>>,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            crate::GlobalNamespace::OVRTask_1<TResult>,
                        >,
                    >), crate::GlobalNamespace::OVRTask_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TResult>>,
                    >, 1usize>("WhenAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WhenAll",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TResult>>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (tasks))? };
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_IEnumerable_1_List_1_1<TResult>(
        tasks: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::OVRTask_1<TResult>,
            >,
        >,
        results: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<TResult>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<TResult>>,
        >,
    >
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                crate::GlobalNamespace::OVRTask_1<TResult>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<TResult>,
                        >,
                    ), crate::GlobalNamespace::OVRTask_1<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<TResult>,
                        >,
                    >, 2usize>("WhenAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WhenAll",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<TResult>>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (tasks, results))? };
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_OVRTask_1_OVRTask_1_2<T1, T2>(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_2<T1, T2>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRTask_1<T1>,
                            crate::GlobalNamespace::OVRTask_1<T2>,
                        ),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::System::ValueTuple_2<T1, T2>,
                        >,
                        2usize,
                    >("WhenAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "WhenAll",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_2<T1, T2>> =
            unsafe { cordl_method_info.invoke_unchecked((), (task1, task2))? };
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_OVRTask_1_OVRTask_1_OVRTask_1_3<T1, T2, T3>(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_3<T1, T2, T3>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRTask_1<T1>,
                            crate::GlobalNamespace::OVRTask_1<T2>,
                            crate::GlobalNamespace::OVRTask_1<T3>,
                        ),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::System::ValueTuple_3<T1, T2, T3>,
                        >,
                        3usize,
                    >("WhenAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "WhenAll",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_3<T1, T2, T3>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (task1, task2, task3))? };
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_4<T1, T2, T3, T4>(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
        task4: crate::GlobalNamespace::OVRTask_1<T4>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_4<T1, T2, T3, T4>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRTask_1<T1>,
                            crate::GlobalNamespace::OVRTask_1<T2>,
                            crate::GlobalNamespace::OVRTask_1<T3>,
                            crate::GlobalNamespace::OVRTask_1<T4>,
                        ),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::System::ValueTuple_4<T1, T2, T3, T4>,
                        >,
                        4usize,
                    >("WhenAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "WhenAll",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_4<T1, T2, T3, T4>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (task1, task2, task3, task4))? };
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_5<T1, T2, T3, T4, T5>(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
        task4: crate::GlobalNamespace::OVRTask_1<T4>,
        task5: crate::GlobalNamespace::OVRTask_1<T5>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_5<T1, T2, T3, T4, T5>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTask_1<T1>,
                        crate::GlobalNamespace::OVRTask_1<T2>,
                        crate::GlobalNamespace::OVRTask_1<T3>,
                        crate::GlobalNamespace::OVRTask_1<T4>,
                        crate::GlobalNamespace::OVRTask_1<T5>,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::System::ValueTuple_5<T1, T2, T3, T4, T5>,
                    >, 5usize>("WhenAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WhenAll",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_5<T1, T2, T3, T4, T5>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (task1, task2, task3, task4, task5))? };
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_6<
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
    >(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
        task4: crate::GlobalNamespace::OVRTask_1<T4>,
        task5: crate::GlobalNamespace::OVRTask_1<T5>,
        task6: crate::GlobalNamespace::OVRTask_1<T6>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_6<T1, T2, T3, T4, T5, T6>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTask_1<T1>,
                        crate::GlobalNamespace::OVRTask_1<T2>,
                        crate::GlobalNamespace::OVRTask_1<T3>,
                        crate::GlobalNamespace::OVRTask_1<T4>,
                        crate::GlobalNamespace::OVRTask_1<T5>,
                        crate::GlobalNamespace::OVRTask_1<T6>,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::System::ValueTuple_6<T1, T2, T3, T4, T5, T6>,
                    >, 6usize>("WhenAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WhenAll",
                            6usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_6<T1, T2, T3, T4, T5, T6>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (task1, task2, task3, task4, task5, task6))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_7<
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
    >(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
        task4: crate::GlobalNamespace::OVRTask_1<T4>,
        task5: crate::GlobalNamespace::OVRTask_1<T5>,
        task6: crate::GlobalNamespace::OVRTask_1<T6>,
        task7: crate::GlobalNamespace::OVRTask_1<T7>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_7<T1, T2, T3, T4, T5, T6, T7>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTask_1<T1>,
                        crate::GlobalNamespace::OVRTask_1<T2>,
                        crate::GlobalNamespace::OVRTask_1<T3>,
                        crate::GlobalNamespace::OVRTask_1<T4>,
                        crate::GlobalNamespace::OVRTask_1<T5>,
                        crate::GlobalNamespace::OVRTask_1<T6>,
                        crate::GlobalNamespace::OVRTask_1<T7>,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::System::ValueTuple_7<T1, T2, T3, T4, T5, T6, T7>,
                    >, 7usize>("WhenAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WhenAll",
                            7usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_7<T1, T2, T3, T4, T5, T6, T7>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (task1, task2, task3, task4, task5, task6, task7))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WhenAll_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_OVRTask_1_8<
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
    >(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
        task4: crate::GlobalNamespace::OVRTask_1<T4>,
        task5: crate::GlobalNamespace::OVRTask_1<T5>,
        task6: crate::GlobalNamespace::OVRTask_1<T6>,
        task7: crate::GlobalNamespace::OVRTask_1<T7>,
        task8: crate::GlobalNamespace::OVRTask_1<T8>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_8<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                crate::System::ValueTuple_1<T8>,
            >,
        >,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T8: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTask_1<T1>,
                        crate::GlobalNamespace::OVRTask_1<T2>,
                        crate::GlobalNamespace::OVRTask_1<T3>,
                        crate::GlobalNamespace::OVRTask_1<T4>,
                        crate::GlobalNamespace::OVRTask_1<T5>,
                        crate::GlobalNamespace::OVRTask_1<T6>,
                        crate::GlobalNamespace::OVRTask_1<T7>,
                        crate::GlobalNamespace::OVRTask_1<T8>,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::System::ValueTuple_8<
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            crate::System::ValueTuple_1<T8>,
                        >,
                    >, 8usize>("WhenAll")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "WhenAll",
                            8usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_8<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                crate::System::ValueTuple_1<T8>,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (task1, task2, task3, task4, task5, task6, task7, task8))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRTask")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTask+Builder")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct OVRTask_Builder {
    pub _synchronousResult: crate::GlobalNamespace::OVRPlugin_Result,
    pub _taskId: crate::System::Guid,
}
#[cfg(feature = "cordl_class_OVRTask+Builder")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTask_Builder {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTask/Builder";
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
#[cfg(feature = "cordl_class_OVRTask+Builder")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRTask_Builder {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRTask+Builder")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRTask_Builder {
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
#[cfg(feature = "cordl_class_OVRTask+Builder")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRTask_Builder {
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
#[cfg(feature = "cordl_class_OVRTask+Builder")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRTask_Builder {
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
#[cfg(feature = "cordl_class_OVRTask+Builder")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRTask_Builder {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRTask+Builder")]
impl crate::GlobalNamespace::OVRTask_Builder {
    pub fn CastResult<TResult>(&mut self) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), TResult, 0usize>("CastResult")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CastResult",
                            0usize
                        )
                    })
            });
        let __cordl_ret: TResult = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToResultTask<TStatus>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRResult_1<TStatus>>,
    >
    where
        TStatus: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRResult_1<TStatus>,
                        >,
                        0usize,
                    >("ToResultTask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToResultTask", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_1<TStatus>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToTask_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRPlugin_Result>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::GlobalNamespace::OVRPlugin_Result,
                        >,
                        0usize,
                    >("ToTask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToTask",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRPlugin_Result,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToTask_1<TStatus>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TStatus>>
    where
        TStatus: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::OVRTask_1<TStatus>, 0usize>("ToTask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToTask",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TStatus> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToTask_3<TValue, TStatus>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::GlobalNamespace::OVRResult_2<TValue, TStatus>>,
    >
    where
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TStatus: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::OVRTask_1<
                        crate::GlobalNamespace::OVRResult_2<TValue, TStatus>,
                    >, 0usize>("ToTask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToTask",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::GlobalNamespace::OVRResult_2<TValue, TStatus>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToTask_TResult2<TResult>(
        &mut self,
        failureValue: TResult,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTask_1<TResult>>
    where
        TResult: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(TResult), crate::GlobalNamespace::OVRTask_1<TResult>, 1usize>(
                        "ToTask",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToTask",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<TResult> =
            unsafe { cordl_method_info.invoke_unchecked(self, (failureValue))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        synchronousResult: crate::GlobalNamespace::OVRPlugin_Result,
        taskId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::OVRPlugin_Result,
                        crate::System::Guid,
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
            unsafe { cordl_method_info.invoke_unchecked(self, (synchronousResult, taskId))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_1")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVRTask_MultiTaskData_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub CombinedTask: crate::GlobalNamespace::OVRTask_1<T>,
    pub Result: T,
    pub Remaining: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<crate::System::Guid>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRTask_MultiTaskData_1<T>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTask/MultiTaskData`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "OVRTask/MultiTaskData`1")
                .unwrap()
                .make_generic::<(T)>()
                .unwrap()
                .unwrap()
        })
    }
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
#[cfg(feature = "OVRTask+MultiTaskData_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::GlobalNamespace::OVRTask_MultiTaskData_1<T>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::GlobalNamespace::OVRTask_MultiTaskData_1<T>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_1")]
impl<T: quest_hook::libil2cpp::Type> crate::GlobalNamespace::OVRTask_MultiTaskData_1<T> {
    pub fn AddTask(
        &mut self,
        id: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::Guid), quest_hook::libil2cpp::Void, 1usize>(
                        "AddTask",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddTask",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (id))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OVRObjectPool_IPoolObject_OnGet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "OVRObjectPool.IPoolObject.OnGet",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OVRObjectPool.IPoolObject.OnGet",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OVRObjectPool_IPoolObject_OnReturn(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "OVRObjectPool.IPoolObject.OnReturn",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OVRObjectPool.IPoolObject.OnReturn",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnResult(
        &mut self,
        taskId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::Guid), quest_hook::libil2cpp::Void, 1usize>(
                        "OnResult",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnResult",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (taskId))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRTask_MultiTaskData_1<T>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::GlobalNamespace::OVRObjectPool_IPoolObject>
    for crate::GlobalNamespace::OVRTask_MultiTaskData_1<T>
{
    fn as_ref(&self) -> &crate::GlobalNamespace::OVRObjectPool_IPoolObject {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::GlobalNamespace::OVRObjectPool_IPoolObject>
    for crate::GlobalNamespace::OVRTask_MultiTaskData_1<T>
{
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::OVRObjectPool_IPoolObject {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_2")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVRTask_MultiTaskData_2<T1: quest_hook::libil2cpp::Type, T2: quest_hook::libil2cpp::Type>
{
    __cordl_parent:
        crate::GlobalNamespace::OVRTask_MultiTaskData_1<crate::System::ValueTuple_2<T1, T2>>,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
    __cordl_phantom_T2: std::marker::PhantomData<T2>,
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_2")]
unsafe impl<T1: quest_hook::libil2cpp::Type, T2: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTask_MultiTaskData_2<T1, T2>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTask/MultiTaskData`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "OVRTask/MultiTaskData`2")
                .unwrap()
                .make_generic::<(T1, T2)>()
                .unwrap()
                .unwrap()
        })
    }
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
#[cfg(feature = "OVRTask+MultiTaskData_2")]
impl<T1: quest_hook::libil2cpp::Type, T2: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::GlobalNamespace::OVRTask_MultiTaskData_2<T1, T2>
{
    type Target =
        crate::GlobalNamespace::OVRTask_MultiTaskData_1<crate::System::ValueTuple_2<T1, T2>>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_2")]
impl<T1: quest_hook::libil2cpp::Type, T2: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::GlobalNamespace::OVRTask_MultiTaskData_2<T1, T2>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_2")]
impl<T1: quest_hook::libil2cpp::Type, T2: quest_hook::libil2cpp::Type>
    crate::GlobalNamespace::OVRTask_MultiTaskData_2<T1, T2>
{
    pub fn Get(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_2<T1, T2>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRTask_1<T1>,
                            crate::GlobalNamespace::OVRTask_1<T2>,
                        ),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::System::ValueTuple_2<T1, T2>,
                        >,
                        2usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Get",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_2<T1, T2>> =
            unsafe { cordl_method_info.invoke_unchecked((), (task1, task2))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_2")]
impl<T1: quest_hook::libil2cpp::Type, T2: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRTask_MultiTaskData_2<T1, T2>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_3")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVRTask_MultiTaskData_3<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
    T3: quest_hook::libil2cpp::Type,
> {
    __cordl_parent:
        crate::GlobalNamespace::OVRTask_MultiTaskData_1<crate::System::ValueTuple_3<T1, T2, T3>>,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
    __cordl_phantom_T2: std::marker::PhantomData<T2>,
    __cordl_phantom_T3: std::marker::PhantomData<T3>,
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_3")]
unsafe impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRTask_MultiTaskData_3<T1, T2, T3>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTask/MultiTaskData`3";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "OVRTask/MultiTaskData`3")
                .unwrap()
                .make_generic::<(T1, T2, T3)>()
                .unwrap()
                .unwrap()
        })
    }
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
#[cfg(feature = "OVRTask+MultiTaskData_3")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
    > std::ops::Deref for crate::GlobalNamespace::OVRTask_MultiTaskData_3<T1, T2, T3>
{
    type Target =
        crate::GlobalNamespace::OVRTask_MultiTaskData_1<crate::System::ValueTuple_3<T1, T2, T3>>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_3")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut for crate::GlobalNamespace::OVRTask_MultiTaskData_3<T1, T2, T3>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_3")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
    > crate::GlobalNamespace::OVRTask_MultiTaskData_3<T1, T2, T3>
{
    pub fn Get(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_3<T1, T2, T3>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRTask_1<T1>,
                            crate::GlobalNamespace::OVRTask_1<T2>,
                            crate::GlobalNamespace::OVRTask_1<T3>,
                        ),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::System::ValueTuple_3<T1, T2, T3>,
                        >,
                        3usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Get",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_3<T1, T2, T3>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (task1, task2, task3))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_3")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRTask_MultiTaskData_3<T1, T2, T3>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_4")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVRTask_MultiTaskData_4<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
    T3: quest_hook::libil2cpp::Type,
    T4: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::OVRTask_MultiTaskData_1<
        crate::System::ValueTuple_4<T1, T2, T3, T4>,
    >,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
    __cordl_phantom_T2: std::marker::PhantomData<T2>,
    __cordl_phantom_T3: std::marker::PhantomData<T3>,
    __cordl_phantom_T4: std::marker::PhantomData<T4>,
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_4")]
unsafe impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRTask_MultiTaskData_4<T1, T2, T3, T4>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTask/MultiTaskData`4";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "OVRTask/MultiTaskData`4")
                .unwrap()
                .make_generic::<(T1, T2, T3, T4)>()
                .unwrap()
                .unwrap()
        })
    }
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
#[cfg(feature = "OVRTask+MultiTaskData_4")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
    > std::ops::Deref for crate::GlobalNamespace::OVRTask_MultiTaskData_4<T1, T2, T3, T4>
{
    type Target = crate::GlobalNamespace::OVRTask_MultiTaskData_1<
        crate::System::ValueTuple_4<T1, T2, T3, T4>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_4")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut for crate::GlobalNamespace::OVRTask_MultiTaskData_4<T1, T2, T3, T4>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_4")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
    > crate::GlobalNamespace::OVRTask_MultiTaskData_4<T1, T2, T3, T4>
{
    pub fn Get(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
        task4: crate::GlobalNamespace::OVRTask_1<T4>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_4<T1, T2, T3, T4>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::OVRTask_1<T1>,
                            crate::GlobalNamespace::OVRTask_1<T2>,
                            crate::GlobalNamespace::OVRTask_1<T3>,
                            crate::GlobalNamespace::OVRTask_1<T4>,
                        ),
                        crate::GlobalNamespace::OVRTask_1<
                            crate::System::ValueTuple_4<T1, T2, T3, T4>,
                        >,
                        4usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Get",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_4<T1, T2, T3, T4>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (task1, task2, task3, task4))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_4")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRTask_MultiTaskData_4<T1, T2, T3, T4>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_5")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVRTask_MultiTaskData_5<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
    T3: quest_hook::libil2cpp::Type,
    T4: quest_hook::libil2cpp::Type,
    T5: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::OVRTask_MultiTaskData_1<
        crate::System::ValueTuple_5<T1, T2, T3, T4, T5>,
    >,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
    __cordl_phantom_T2: std::marker::PhantomData<T2>,
    __cordl_phantom_T3: std::marker::PhantomData<T3>,
    __cordl_phantom_T4: std::marker::PhantomData<T4>,
    __cordl_phantom_T5: std::marker::PhantomData<T5>,
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_5")]
unsafe impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRTask_MultiTaskData_5<T1, T2, T3, T4, T5>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTask/MultiTaskData`5";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "OVRTask/MultiTaskData`5")
                .unwrap()
                .make_generic::<(T1, T2, T3, T4, T5)>()
                .unwrap()
                .unwrap()
        })
    }
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
#[cfg(feature = "OVRTask+MultiTaskData_5")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
    > std::ops::Deref for crate::GlobalNamespace::OVRTask_MultiTaskData_5<T1, T2, T3, T4, T5>
{
    type Target = crate::GlobalNamespace::OVRTask_MultiTaskData_1<
        crate::System::ValueTuple_5<T1, T2, T3, T4, T5>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_5")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut for crate::GlobalNamespace::OVRTask_MultiTaskData_5<T1, T2, T3, T4, T5>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_5")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
    > crate::GlobalNamespace::OVRTask_MultiTaskData_5<T1, T2, T3, T4, T5>
{
    pub fn Get(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
        task4: crate::GlobalNamespace::OVRTask_1<T4>,
        task5: crate::GlobalNamespace::OVRTask_1<T5>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_5<T1, T2, T3, T4, T5>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTask_1<T1>,
                        crate::GlobalNamespace::OVRTask_1<T2>,
                        crate::GlobalNamespace::OVRTask_1<T3>,
                        crate::GlobalNamespace::OVRTask_1<T4>,
                        crate::GlobalNamespace::OVRTask_1<T5>,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::System::ValueTuple_5<T1, T2, T3, T4, T5>,
                    >, 5usize>("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Get",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_5<T1, T2, T3, T4, T5>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (task1, task2, task3, task4, task5))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_5")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRTask_MultiTaskData_5<T1, T2, T3, T4, T5>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_6")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVRTask_MultiTaskData_6<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
    T3: quest_hook::libil2cpp::Type,
    T4: quest_hook::libil2cpp::Type,
    T5: quest_hook::libil2cpp::Type,
    T6: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::OVRTask_MultiTaskData_1<
        crate::System::ValueTuple_6<T1, T2, T3, T4, T5, T6>,
    >,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
    __cordl_phantom_T2: std::marker::PhantomData<T2>,
    __cordl_phantom_T3: std::marker::PhantomData<T3>,
    __cordl_phantom_T4: std::marker::PhantomData<T4>,
    __cordl_phantom_T5: std::marker::PhantomData<T5>,
    __cordl_phantom_T6: std::marker::PhantomData<T6>,
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_6")]
unsafe impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRTask_MultiTaskData_6<T1, T2, T3, T4, T5, T6>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTask/MultiTaskData`6";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "OVRTask/MultiTaskData`6")
                .unwrap()
                .make_generic::<(T1, T2, T3, T4, T5, T6)>()
                .unwrap()
                .unwrap()
        })
    }
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
#[cfg(feature = "OVRTask+MultiTaskData_6")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
    > std::ops::Deref for crate::GlobalNamespace::OVRTask_MultiTaskData_6<T1, T2, T3, T4, T5, T6>
{
    type Target = crate::GlobalNamespace::OVRTask_MultiTaskData_1<
        crate::System::ValueTuple_6<T1, T2, T3, T4, T5, T6>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_6")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut
    for crate::GlobalNamespace::OVRTask_MultiTaskData_6<T1, T2, T3, T4, T5, T6>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_6")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
    > crate::GlobalNamespace::OVRTask_MultiTaskData_6<T1, T2, T3, T4, T5, T6>
{
    pub fn Get(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
        task4: crate::GlobalNamespace::OVRTask_1<T4>,
        task5: crate::GlobalNamespace::OVRTask_1<T5>,
        task6: crate::GlobalNamespace::OVRTask_1<T6>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_6<T1, T2, T3, T4, T5, T6>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTask_1<T1>,
                        crate::GlobalNamespace::OVRTask_1<T2>,
                        crate::GlobalNamespace::OVRTask_1<T3>,
                        crate::GlobalNamespace::OVRTask_1<T4>,
                        crate::GlobalNamespace::OVRTask_1<T5>,
                        crate::GlobalNamespace::OVRTask_1<T6>,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::System::ValueTuple_6<T1, T2, T3, T4, T5, T6>,
                    >, 6usize>("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Get",
                            6usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_6<T1, T2, T3, T4, T5, T6>,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (task1, task2, task3, task4, task5, task6))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_6")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRTask_MultiTaskData_6<T1, T2, T3, T4, T5, T6>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_7")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVRTask_MultiTaskData_7<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
    T3: quest_hook::libil2cpp::Type,
    T4: quest_hook::libil2cpp::Type,
    T5: quest_hook::libil2cpp::Type,
    T6: quest_hook::libil2cpp::Type,
    T7: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::OVRTask_MultiTaskData_1<
        crate::System::ValueTuple_7<T1, T2, T3, T4, T5, T6, T7>,
    >,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
    __cordl_phantom_T2: std::marker::PhantomData<T2>,
    __cordl_phantom_T3: std::marker::PhantomData<T3>,
    __cordl_phantom_T4: std::marker::PhantomData<T4>,
    __cordl_phantom_T5: std::marker::PhantomData<T5>,
    __cordl_phantom_T6: std::marker::PhantomData<T6>,
    __cordl_phantom_T7: std::marker::PhantomData<T7>,
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_7")]
unsafe impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRTask_MultiTaskData_7<T1, T2, T3, T4, T5, T6, T7>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTask/MultiTaskData`7";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "OVRTask/MultiTaskData`7")
                .unwrap()
                .make_generic::<(T1, T2, T3, T4, T5, T6, T7)>()
                .unwrap()
                .unwrap()
        })
    }
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
#[cfg(feature = "OVRTask+MultiTaskData_7")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
    > std::ops::Deref
    for crate::GlobalNamespace::OVRTask_MultiTaskData_7<T1, T2, T3, T4, T5, T6, T7>
{
    type Target = crate::GlobalNamespace::OVRTask_MultiTaskData_1<
        crate::System::ValueTuple_7<T1, T2, T3, T4, T5, T6, T7>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_7")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut
    for crate::GlobalNamespace::OVRTask_MultiTaskData_7<T1, T2, T3, T4, T5, T6, T7>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_7")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
    > crate::GlobalNamespace::OVRTask_MultiTaskData_7<T1, T2, T3, T4, T5, T6, T7>
{
    pub fn Get(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
        task4: crate::GlobalNamespace::OVRTask_1<T4>,
        task5: crate::GlobalNamespace::OVRTask_1<T5>,
        task6: crate::GlobalNamespace::OVRTask_1<T6>,
        task7: crate::GlobalNamespace::OVRTask_1<T7>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<crate::System::ValueTuple_7<T1, T2, T3, T4, T5, T6, T7>>,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTask_1<T1>,
                        crate::GlobalNamespace::OVRTask_1<T2>,
                        crate::GlobalNamespace::OVRTask_1<T3>,
                        crate::GlobalNamespace::OVRTask_1<T4>,
                        crate::GlobalNamespace::OVRTask_1<T5>,
                        crate::GlobalNamespace::OVRTask_1<T6>,
                        crate::GlobalNamespace::OVRTask_1<T7>,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::System::ValueTuple_7<T1, T2, T3, T4, T5, T6, T7>,
                    >, 7usize>("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Get",
                            7usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_7<T1, T2, T3, T4, T5, T6, T7>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (task1, task2, task3, task4, task5, task6, task7))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_7")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRTask_MultiTaskData_7<T1, T2, T3, T4, T5, T6, T7>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_8")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVRTask_MultiTaskData_8<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
    T3: quest_hook::libil2cpp::Type,
    T4: quest_hook::libil2cpp::Type,
    T5: quest_hook::libil2cpp::Type,
    T6: quest_hook::libil2cpp::Type,
    T7: quest_hook::libil2cpp::Type,
    T8: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::OVRTask_MultiTaskData_1<
        crate::System::ValueTuple_8<T1, T2, T3, T4, T5, T6, T7, crate::System::ValueTuple_1<T8>>,
    >,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
    __cordl_phantom_T2: std::marker::PhantomData<T2>,
    __cordl_phantom_T3: std::marker::PhantomData<T3>,
    __cordl_phantom_T4: std::marker::PhantomData<T4>,
    __cordl_phantom_T5: std::marker::PhantomData<T5>,
    __cordl_phantom_T6: std::marker::PhantomData<T6>,
    __cordl_phantom_T7: std::marker::PhantomData<T7>,
    __cordl_phantom_T8: std::marker::PhantomData<T8>,
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_8")]
unsafe impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
        T8: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRTask_MultiTaskData_8<T1, T2, T3, T4, T5, T6, T7, T8>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRTask/MultiTaskData`8";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "OVRTask/MultiTaskData`8")
                .unwrap()
                .make_generic::<(T1, T2, T3, T4, T5, T6, T7, T8)>()
                .unwrap()
                .unwrap()
        })
    }
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
#[cfg(feature = "OVRTask+MultiTaskData_8")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
        T8: quest_hook::libil2cpp::Type,
    > std::ops::Deref
    for crate::GlobalNamespace::OVRTask_MultiTaskData_8<T1, T2, T3, T4, T5, T6, T7, T8>
{
    type Target = crate::GlobalNamespace::OVRTask_MultiTaskData_1<
        crate::System::ValueTuple_8<T1, T2, T3, T4, T5, T6, T7, crate::System::ValueTuple_1<T8>>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_8")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
        T8: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut
    for crate::GlobalNamespace::OVRTask_MultiTaskData_8<T1, T2, T3, T4, T5, T6, T7, T8>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRTask+MultiTaskData_8")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
        T8: quest_hook::libil2cpp::Type,
    > crate::GlobalNamespace::OVRTask_MultiTaskData_8<T1, T2, T3, T4, T5, T6, T7, T8>
{
    pub fn Get(
        task1: crate::GlobalNamespace::OVRTask_1<T1>,
        task2: crate::GlobalNamespace::OVRTask_1<T2>,
        task3: crate::GlobalNamespace::OVRTask_1<T3>,
        task4: crate::GlobalNamespace::OVRTask_1<T4>,
        task5: crate::GlobalNamespace::OVRTask_1<T5>,
        task6: crate::GlobalNamespace::OVRTask_1<T6>,
        task7: crate::GlobalNamespace::OVRTask_1<T7>,
        task8: crate::GlobalNamespace::OVRTask_1<T8>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_8<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                crate::System::ValueTuple_1<T8>,
            >,
        >,
    >
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T8: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTask_1<T1>,
                        crate::GlobalNamespace::OVRTask_1<T2>,
                        crate::GlobalNamespace::OVRTask_1<T3>,
                        crate::GlobalNamespace::OVRTask_1<T4>,
                        crate::GlobalNamespace::OVRTask_1<T5>,
                        crate::GlobalNamespace::OVRTask_1<T6>,
                        crate::GlobalNamespace::OVRTask_1<T7>,
                        crate::GlobalNamespace::OVRTask_1<T8>,
                    ), crate::GlobalNamespace::OVRTask_1<
                        crate::System::ValueTuple_8<
                            T1,
                            T2,
                            T3,
                            T4,
                            T5,
                            T6,
                            T7,
                            crate::System::ValueTuple_1<T8>,
                        >,
                    >, 8usize>("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Get",
                            8usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTask_1<
            crate::System::ValueTuple_8<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                crate::System::ValueTuple_1<T8>,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked((), (task1, task2, task3, task4, task5, task6, task7, task8))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T8: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T5: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T6: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T7: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T8: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRTask+MultiTaskData_8")]
impl<
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
        T4: quest_hook::libil2cpp::Type,
        T5: quest_hook::libil2cpp::Type,
        T6: quest_hook::libil2cpp::Type,
        T7: quest_hook::libil2cpp::Type,
        T8: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::OVRTask_MultiTaskData_8<T1, T2, T3, T4, T5, T6, T7, T8>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
