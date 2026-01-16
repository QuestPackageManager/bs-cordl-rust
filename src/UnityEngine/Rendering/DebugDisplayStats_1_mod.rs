#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugDisplayStats_1<TProfileId: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_AccumulatedTiming: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    TProfileId,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::DebugDisplayStats_1_AccumulatedTiming<
                            TProfileId,
                        >,
                    >,
                >,
            >,
        >,
    >,
    pub m_TimeSinceLastAvgValue: f32,
    pub m_AccumulatedFrames: i32,
    pub m_HiddenProfileIds:
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::HashSet_1<TProfileId>>,
    pub averageProfilerTimingsOverASecond: bool,
    pub hideEmptyScopes: bool,
    __cordl_phantom_TProfileId: std::marker::PhantomData<TProfileId>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1")]
unsafe impl<TProfileId: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::DebugDisplayStats_1<TProfileId>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "DebugDisplayStats`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("UnityEngine.Rendering", "DebugDisplayStats`1")
                .unwrap()
                .make_generic::<(TProfileId)>()
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
#[cfg(feature = "UnityEngine+Rendering+DebugDisplayStats_1")]
impl<TProfileId: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::UnityEngine::Rendering::DebugDisplayStats_1<TProfileId>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplayStats_1")]
impl<TProfileId: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::UnityEngine::Rendering::DebugDisplayStats_1<TProfileId>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplayStats_1")]
impl<TProfileId: quest_hook::libil2cpp::Type>
    crate::UnityEngine::Rendering::DebugDisplayStats_1<TProfileId>
{
    pub const k_AccumulationTimeInSeconds: f32 = 1f32;
    #[cfg(feature = "UnityEngine+Rendering+DebugDisplayStats_1+AccumulatedTiming")]
    pub type AccumulatedTiming =
        crate::UnityEngine::Rendering::DebugDisplayStats_1_AccumulatedTiming<TProfileId>;
    #[cfg(feature = "UnityEngine+Rendering+DebugDisplayStats_1+DebugProfilingType")]
    pub type DebugProfilingType =
        crate::UnityEngine::Rendering::DebugDisplayStats_1_DebugProfilingType;
    pub fn BuildDetailedStatsList(
        &mut self,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        samplers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TProfileId>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    >
    where
        TProfileId: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<TProfileId>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        2usize,
                    >("BuildDetailedStatsList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildDetailedStatsList", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget> =
            unsafe { cordl_method_info.invoke_unchecked(self, (title, samplers))? };
        Ok(__cordl_ret.into())
    }
    pub fn BuildProfilingSamplerWidgetList(
        &mut self,
        samplers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<TProfileId>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ObservableList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
            >,
        >,
    >
    where
        TProfileId: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<TProfileId>,
                    >), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::ObservableList_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DebugUI_Widget,
                            >,
                        >,
                    >, 1usize>("BuildProfilingSamplerWidgetList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BuildProfilingSamplerWidgetList",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ObservableList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (samplers))? };
        Ok(__cordl_ret.into())
    }
    pub fn DisableProfilingRecorders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProfileId: quest_hook::libil2cpp::Type
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
                        "DisableProfilingRecorders",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisableProfilingRecorders",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn EnableProfilingRecorders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProfileId: quest_hook::libil2cpp::Type
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
                        "EnableProfilingRecorders",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnableProfilingRecorders",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetProfilerIdsToDisplay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<TProfileId>>,
    >
    where
        TProfileId: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<TProfileId>,
                    >, 0usize>("GetProfilerIdsToDisplay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetProfilerIdsToDisplay",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TProfileId>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSamplerTiming(
        &mut self,
        samplerId: TProfileId,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
        _cordl_type: crate::UnityEngine::Rendering::DebugDisplayStats_1_DebugProfilingType<
            TProfileId,
        >,
    ) -> quest_hook::libil2cpp::Result<f32>
    where
        TProfileId: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        TProfileId,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
                        crate::UnityEngine::Rendering::DebugDisplayStats_1_DebugProfilingType<
                            TProfileId,
                        >,
                    ), f32, 3usize>("GetSamplerTiming")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetSamplerTiming",
                            3usize
                        )
                    })
            });
        let __cordl_ret: f32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (samplerId, sampler, _cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TProfileId: quest_hook::libil2cpp::Type
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
    pub fn RegisterDebugUI(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProfileId: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DebugUI_Widget,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>("RegisterDebugUI")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RegisterDebugUI",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (list))? };
        Ok(__cordl_ret.into())
    }
    pub fn Update(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProfileId: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Update",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDetailedStats(
        &mut self,
        samplers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TProfileId>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProfileId: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<TProfileId>,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UpdateDetailedStats"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateDetailedStats",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (samplers))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateListOfAveragedProfilerTimings(
        &mut self,
        needUpdatingAverages: bool,
        samplers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TProfileId>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProfileId: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        bool,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<TProfileId>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UpdateListOfAveragedProfilerTimings"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateListOfAveragedProfilerTimings",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (needUpdatingAverages, samplers))? };
        Ok(__cordl_ret.into())
    }
    pub fn _BuildProfilingSamplerWidgetList_g__CreateWidgetForSampler_19_0(
        &mut self,
        samplerId: TProfileId,
        sampler: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
        _cordl_type: crate::UnityEngine::Rendering::DebugDisplayStats_1_DebugProfilingType<
            TProfileId,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Value>,
    >
    where
        TProfileId: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            TProfileId,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::ProfilingSampler,
                            >,
                            crate::UnityEngine::Rendering::DebugDisplayStats_1_DebugProfilingType<
                                TProfileId,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Value,
                        >,
                        3usize,
                    >("<BuildProfilingSamplerWidgetList>g__CreateWidgetForSampler|19_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<BuildProfilingSamplerWidgetList>g__CreateWidgetForSampler|19_0",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Value> =
            unsafe { cordl_method_info.invoke_unchecked(self, (samplerId, sampler, _cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProfileId: quest_hook::libil2cpp::Type
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1")]
impl<TProfileId: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::DebugDisplayStats_1<TProfileId>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1+AccumulatedTiming")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugDisplayStats_1_AccumulatedTiming<TProfileId: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub accumulatedValue: f32,
    pub lastAverage: f32,
    __cordl_phantom_TProfileId: std::marker::PhantomData<TProfileId>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1+AccumulatedTiming")]
unsafe impl<TProfileId: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::DebugDisplayStats_1_AccumulatedTiming<TProfileId>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "DebugDisplayStats`1/AccumulatedTiming";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "UnityEngine.Rendering",
                "DebugDisplayStats`1/AccumulatedTiming",
            )
            .unwrap()
            .make_generic::<(TProfileId)>()
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
#[cfg(feature = "UnityEngine+Rendering+DebugDisplayStats_1+AccumulatedTiming")]
impl<TProfileId: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::UnityEngine::Rendering::DebugDisplayStats_1_AccumulatedTiming<TProfileId>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplayStats_1+AccumulatedTiming")]
impl<TProfileId: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::UnityEngine::Rendering::DebugDisplayStats_1_AccumulatedTiming<TProfileId>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+DebugDisplayStats_1+AccumulatedTiming")]
impl<TProfileId: quest_hook::libil2cpp::Type>
    crate::UnityEngine::Rendering::DebugDisplayStats_1_AccumulatedTiming<TProfileId>
{
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TProfileId: quest_hook::libil2cpp::Type
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
    pub fn UpdateLastAverage(
        &mut self,
        frameCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProfileId: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("UpdateLastAverage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateLastAverage",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (frameCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TProfileId: quest_hook::libil2cpp::Type
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1+AccumulatedTiming")]
impl<TProfileId: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::DebugDisplayStats_1_AccumulatedTiming<TProfileId>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1+DebugProfilingType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum DebugDisplayStats_1_DebugProfilingType {
    #[default]
    CPU = 0i32,
    GPU = 2i32,
    InlineCPU = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1+DebugProfilingType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::DebugDisplayStats_1_DebugProfilingType
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "DebugDisplayStats`1/DebugProfilingType";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1+DebugProfilingType")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::DebugDisplayStats_1_DebugProfilingType
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1+DebugProfilingType")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::DebugDisplayStats_1_DebugProfilingType
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1+DebugProfilingType")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::DebugDisplayStats_1_DebugProfilingType
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+DebugDisplayStats_1+DebugProfilingType")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::DebugDisplayStats_1_DebugProfilingType
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
