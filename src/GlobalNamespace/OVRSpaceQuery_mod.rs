#[cfg(feature = "cordl_class_OVRSpaceQuery")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRSpaceQuery {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRSpaceQuery")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRSpaceQuery {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRSpaceQuery";
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
#[cfg(feature = "OVRSpaceQuery")]
impl std::ops::Deref for crate::GlobalNamespace::OVRSpaceQuery {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSpaceQuery")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRSpaceQuery {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSpaceQuery")]
impl crate::GlobalNamespace::OVRSpaceQuery {
    pub const DefaultTimeout: f64 = 0f64;
    pub const MaxResultsForAnchors: i32 = 1024i32;
    pub const MaxResultsForGroup: i32 = 1024i32;
    #[cfg(feature = "OVRSpaceQuery+Options")]
    pub type Options = crate::GlobalNamespace::OVRSpaceQuery_Options;
    #[cfg(feature = "OVRSpaceQuery+QueryInfoUnion")]
    pub type QueryInfoUnion = crate::GlobalNamespace::OVRSpaceQuery_QueryInfoUnion;
    pub fn AppendAnchors(
        query: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2>,
        anchorIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            crate::GlobalNamespace::OVRPlugin_Result,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                        >,
                    ), crate::System::ValueTuple_2<
                        crate::GlobalNamespace::OVRPlugin_Result,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >, 2usize>("AppendAnchors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AppendAnchors",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            crate::GlobalNamespace::OVRPlugin_Result,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (query, anchorIds))? };
        Ok(__cordl_ret.into())
    }
    pub fn ForAnchors(
        anchorIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
        query: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            crate::GlobalNamespace::OVRPlugin_Result,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2,
                        >,
                    ), crate::System::ValueTuple_2<
                        crate::GlobalNamespace::OVRPlugin_Result,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >, 2usize>("ForAnchors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ForAnchors",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            crate::GlobalNamespace::OVRPlugin_Result,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (anchorIds, query))? };
        Ok(__cordl_ret.into())
    }
    pub fn ForAnchorsThrow(
        anchorIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
        argName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2, 2usize>(
                        "ForAnchorsThrow"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ForAnchorsThrow",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2 =
            unsafe { cordl_method_info.invoke_unchecked((), (anchorIds, argName))? };
        Ok(__cordl_ret.into())
    }
    pub fn ForAnchorsUnchecked(
        anchorIds: crate::GlobalNamespace::OVREnumerable_1<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVREnumerable_1<crate::System::Guid>),
                        crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2,
                        1usize,
                    >("ForAnchorsUnchecked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ForAnchorsUnchecked", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2 =
            unsafe { cordl_method_info.invoke_unchecked((), (anchorIds))? };
        Ok(__cordl_ret.into())
    }
    pub fn ForComponent(
        _cordl_type: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        query: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            crate::GlobalNamespace::OVRPlugin_Result,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2,
                        >,
                    ), crate::System::ValueTuple_2<
                        crate::GlobalNamespace::OVRPlugin_Result,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >, 2usize>("ForComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ForComponent",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            crate::GlobalNamespace::OVRPlugin_Result,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type, query))? };
        Ok(__cordl_ret.into())
    }
    pub fn ForComponentThrow(
        _cordl_type: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        argName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2, 2usize>(
                        "ForComponentThrow",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ForComponentThrow",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2 =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type, argName))? };
        Ok(__cordl_ret.into())
    }
    pub fn ForComponentUnchecked(
        _cordl_type: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRPlugin_SpaceComponentType),
                        crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2,
                        1usize,
                    >("ForComponentUnchecked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ForComponentUnchecked", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2 =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn ForGroup(
        groupUuid: crate::System::Guid,
        query: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2>,
        anchorIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            crate::GlobalNamespace::OVRPlugin_Result,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Guid,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                        >,
                    ), crate::System::ValueTuple_2<
                        crate::GlobalNamespace::OVRPlugin_Result,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >, 3usize>("ForGroup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ForGroup",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            crate::GlobalNamespace::OVRPlugin_Result,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (groupUuid, query, anchorIds))? };
        Ok(__cordl_ret.into())
    }
    pub fn ForGroupThrow(
        groupUuid: crate::System::Guid,
        argName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        anchorIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Guid,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                        >,
                    ), crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2, 3usize>(
                        "ForGroupThrow"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ForGroupThrow",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2 =
            unsafe { cordl_method_info.invoke_unchecked((), (groupUuid, argName, anchorIds))? };
        Ok(__cordl_ret.into())
    }
    pub fn ForGroupUnchecked(
        groupUuid: crate::System::Guid,
        anchorIds: crate::GlobalNamespace::OVREnumerable_1<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::System::Guid,
                        crate::GlobalNamespace::OVREnumerable_1<crate::System::Guid>,
                    ), crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2, 2usize>(
                        "ForGroupUnchecked",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ForGroupUnchecked",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2 =
            unsafe { cordl_method_info.invoke_unchecked((), (groupUuid, anchorIds))? };
        Ok(__cordl_ret.into())
    }
    pub fn PostProcessQuery(
        query: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2>,
        result: crate::GlobalNamespace::OVRPlugin_Result,
        why: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            crate::GlobalNamespace::OVRPlugin_Result,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2,
                        >,
                        crate::GlobalNamespace::OVRPlugin_Result,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                    ), crate::System::ValueTuple_2<
                        crate::GlobalNamespace::OVRPlugin_Result,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >, 3usize>("PostProcessQuery")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PostProcessQuery",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            crate::GlobalNamespace::OVRPlugin_Result,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = unsafe { cordl_method_info.invoke_unchecked((), (query, result, why))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToV1(
        query2: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2,
                    >), crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo, 1usize>(
                        "ToV1"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToV1",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo =
            unsafe { cordl_method_info.invoke_unchecked((), (query2))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToV2(
        query1: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo,
                    >), crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2, 1usize>(
                        "ToV2"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToV2",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2 =
            unsafe { cordl_method_info.invoke_unchecked((), (query1))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRSpaceQuery")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRSpaceQuery {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRSpaceQuery+Options")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRSpaceQuery_Options {
    pub _MaxResults_k__BackingField: i32,
    pub _Timeout_k__BackingField: f64,
    pub _Location_k__BackingField: crate::GlobalNamespace::OVRSpace_StorageLocation,
    pub _QueryType_k__BackingField: crate::GlobalNamespace::OVRPlugin_SpaceQueryType,
    pub _ActionType_k__BackingField: crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType,
    pub _componentType: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    pub _uuidFilter: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
    >,
    pub _groupFilter: crate::System::Nullable_1<crate::System::Guid>,
}
#[cfg(feature = "cordl_class_OVRSpaceQuery+Options")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRSpaceQuery_Options {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRSpaceQuery/Options";
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
#[cfg(feature = "cordl_class_OVRSpaceQuery+Options")]
unsafe impl quest_hook::libil2cpp::Argument for crate::GlobalNamespace::OVRSpaceQuery_Options {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRSpaceQuery+Options")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::GlobalNamespace::OVRSpaceQuery_Options {
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
#[cfg(feature = "cordl_class_OVRSpaceQuery+Options")]
unsafe impl quest_hook::libil2cpp::Returned for crate::GlobalNamespace::OVRSpaceQuery_Options {
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
#[cfg(feature = "cordl_class_OVRSpaceQuery+Options")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRSpaceQuery_Options {
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
#[cfg(feature = "cordl_class_OVRSpaceQuery+Options")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::OVRSpaceQuery_Options {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSpaceQuery+Options")]
impl crate::GlobalNamespace::OVRSpaceQuery_Options {
    pub const MaxUuidCount: i32 = 1024i32;
    pub fn ToQueryInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo, 0usize>(
                        "ToQueryInfo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToQueryInfo",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ToQueryInfo2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2, 0usize>(
                        "ToQueryInfo2",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ToQueryInfo2",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryInfo2 =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn TryQuerySpaces(
        &mut self,
        requestId: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<u64>), bool, 1usize>(
                        "TryQuerySpaces",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryQuerySpaces",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (requestId))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateSingleFilter(
        uuidFilter: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
        componentFilter: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
        groupFilter: crate::System::Nullable_1<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                        >,
                        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                        crate::System::Nullable_1<crate::System::Guid>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "ValidateSingleFilter"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ValidateSingleFilter",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (uuidFilter, componentFilter, groupFilter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ActionType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType,
                        0usize,
                    >("get_ActionType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ActionType", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ComponentFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceComponentType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
                        0usize,
                    >("get_ComponentFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ComponentFilter", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceComponentType =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_GroupFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<crate::System::Guid>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::System::Nullable_1<crate::System::Guid>, 0usize>(
                        "get_GroupFilter",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_GroupFilter",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Nullable_1<crate::System::Guid> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Location(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRSpace_StorageLocation> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::OVRSpace_StorageLocation, 0usize>(
                        "get_Location",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Location",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRSpace_StorageLocation =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxResults(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_MaxResults")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_MaxResults",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_QueryType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_SpaceQueryType> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::OVRPlugin_SpaceQueryType, 0usize>(
                        "get_QueryType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_QueryType",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_SpaceQueryType =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Timeout(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f64, 0usize>("get_Timeout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_Timeout",
                            0usize
                        )
                    })
            });
        let __cordl_ret: f64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_UuidFilter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                    >, 0usize>("get_UuidFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_UuidFilter",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_ActionType(
        &mut self,
        value: crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRPlugin_SpaceQueryActionType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_ActionType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_ActionType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_ComponentFilter(
        &mut self,
        value: crate::GlobalNamespace::OVRPlugin_SpaceComponentType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRPlugin_SpaceComponentType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_ComponentFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_ComponentFilter", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_GroupFilter(
        &mut self,
        value: crate::System::Nullable_1<crate::System::Guid>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Nullable_1<crate::System::Guid>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_GroupFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_GroupFilter", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Location(
        &mut self,
        value: crate::GlobalNamespace::OVRSpace_StorageLocation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRSpace_StorageLocation),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Location")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Location", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_MaxResults(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_MaxResults")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_MaxResults",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_QueryType(
        &mut self,
        value: crate::GlobalNamespace::OVRPlugin_SpaceQueryType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRPlugin_SpaceQueryType),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_QueryType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_QueryType", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Timeout(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f64), quest_hook::libil2cpp::Void, 1usize>("set_Timeout")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_Timeout",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_UuidFilter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                    >), quest_hook::libil2cpp::Void, 1usize>("set_UuidFilter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_UuidFilter",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRSpaceQuery+QueryInfoUnion")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OVRSpaceQuery_QueryInfoUnion {
    padding: quest_hook::libil2cpp::ValueTypePadding<80usize>,
}
#[cfg(feature = "cordl_class_OVRSpaceQuery+QueryInfoUnion")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRSpaceQuery_QueryInfoUnion {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRSpaceQuery/QueryInfoUnion";
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
#[cfg(feature = "cordl_class_OVRSpaceQuery+QueryInfoUnion")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::OVRSpaceQuery_QueryInfoUnion
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRSpaceQuery+QueryInfoUnion")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::OVRSpaceQuery_QueryInfoUnion
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
#[cfg(feature = "cordl_class_OVRSpaceQuery+QueryInfoUnion")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::OVRSpaceQuery_QueryInfoUnion
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
#[cfg(feature = "cordl_class_OVRSpaceQuery+QueryInfoUnion")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::OVRSpaceQuery_QueryInfoUnion {
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
#[cfg(feature = "cordl_class_OVRSpaceQuery+QueryInfoUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::OVRSpaceQuery_QueryInfoUnion
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRSpaceQuery+QueryInfoUnion")]
impl crate::GlobalNamespace::OVRSpaceQuery_QueryInfoUnion {}
