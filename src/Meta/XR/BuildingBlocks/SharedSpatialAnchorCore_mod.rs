#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+SharedSpatialAnchorCore")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct SharedSpatialAnchorCore {
    __cordl_parent: crate::Meta::XR::BuildingBlocks::SpatialAnchorCoreBuildingBlock,
    pub _onSpatialAnchorsShareCompleted: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Events::UnityEvent_2<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                >,
            >,
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    >,
    pub _onSpatialAnchorsShareToGroupCompleted: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Events::UnityEvent_2<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                >,
            >,
            crate::GlobalNamespace::OVRAnchor_ShareResult,
        >,
    >,
    pub _onSharedSpatialAnchorsLoadCompleted: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Events::UnityEvent_2<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                >,
            >,
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    >,
    pub _onShareCompleted: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IEnumerable_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                >,
            >,
        >,
    >,
    pub _onShareToGroupCompleted: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IEnumerable_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+SharedSpatialAnchorCore")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::BuildingBlocks::SharedSpatialAnchorCore
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.BuildingBlocks";
    const CLASS_NAME: &'static str = "SharedSpatialAnchorCore";
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
#[cfg(feature = "Meta+XR+BuildingBlocks+SharedSpatialAnchorCore")]
impl std::ops::Deref for crate::Meta::XR::BuildingBlocks::SharedSpatialAnchorCore {
    type Target = crate::Meta::XR::BuildingBlocks::SpatialAnchorCoreBuildingBlock;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+SharedSpatialAnchorCore")]
impl std::ops::DerefMut for crate::Meta::XR::BuildingBlocks::SharedSpatialAnchorCore {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+SharedSpatialAnchorCore")]
impl crate::Meta::XR::BuildingBlocks::SharedSpatialAnchorCore {
    pub fn InitSpatialAnchor(
        &mut self,
        anchor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRSpatialAnchor,
                        >),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        1usize,
                    >("InitSpatialAnchor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitSpatialAnchor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task> =
            unsafe { cordl_method_info.invoke_unchecked(self, (anchor))? };
        Ok(__cordl_ret.into())
    }
    pub fn InstantiateSpatialAnchor(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        crate::UnityEngine::Vector3,
                        crate::UnityEngine::Quaternion,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "InstantiateSpatialAnchor"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InstantiateSpatialAnchor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (prefab, position, rotation))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadAndInstantiateAnchors(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        uuids: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::System::Guid>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<crate::System::Guid>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "LoadAndInstantiateAnchors"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadAndInstantiateAnchors",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (prefab, uuids))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadAndInstantiateAnchorsFromGroup(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        groupUuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        crate::System::Guid,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "LoadAndInstantiateAnchorsFromGroup"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadAndInstantiateAnchorsFromGroup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (prefab, groupUuid))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSharedSpatialAnchorsRoutine(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        result: crate::GlobalNamespace::OVRResult_2<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::GlobalNamespace::OVRSpatialAnchor_UnboundAnchor,
                >,
            >,
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        crate::GlobalNamespace::OVRResult_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::GlobalNamespace::OVRSpatialAnchor_UnboundAnchor,
                                >,
                            >,
                            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "LoadSharedSpatialAnchorsRoutine"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadSharedSpatialAnchorsRoutine",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (prefab, result))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnDestroy",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnShareCompleted(
        &mut self,
        result: crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("OnShareCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnShareCompleted",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (result, anchors))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnShareToGroupCompleted(
        &mut self,
        result: crate::GlobalNamespace::OVRResult_1<crate::GlobalNamespace::OVRAnchor_ShareResult>,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::OVRResult_1<
                            crate::GlobalNamespace::OVRAnchor_ShareResult,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "OnShareToGroupCompleted"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnShareToGroupCompleted",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (result, anchors))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShareSpatialAnchors_Guid1(
        &mut self,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            >,
        >,
        groupUuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                            >,
                        >,
                        crate::System::Guid,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "ShareSpatialAnchors"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShareSpatialAnchors",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (anchors, groupUuid))? };
        Ok(__cordl_ret.into())
    }
    pub fn ShareSpatialAnchors_List_1_0(
        &mut self,
        anchors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            >,
        >,
        users: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::GlobalNamespace::OVRSpaceUser>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::GlobalNamespace::OVRSpaceUser,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "ShareSpatialAnchors"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ShareSpatialAnchors",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (anchors, users))? };
        Ok(__cordl_ret.into())
    }
    pub fn Start(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Start",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
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
    pub fn get_OnSharedSpatialAnchorsLoadCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Events::UnityEvent_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::OVRSpatialAnchor,
                                    >,
                                >,
                            >,
                            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        >,
                    >, 0usize>("get_OnSharedSpatialAnchorsLoadCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_OnSharedSpatialAnchorsLoadCompleted",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_OnSpatialAnchorsShareCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Events::UnityEvent_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::OVRSpatialAnchor,
                                    >,
                                >,
                            >,
                            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        >,
                    >, 0usize>("get_OnSpatialAnchorsShareCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_OnSpatialAnchorsShareCompleted",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_OnSpatialAnchorsShareToGroupCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
                crate::GlobalNamespace::OVRAnchor_ShareResult,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Events::UnityEvent_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::OVRSpatialAnchor,
                                    >,
                                >,
                            >,
                            crate::GlobalNamespace::OVRAnchor_ShareResult,
                        >,
                    >, 0usize>("get_OnSpatialAnchorsShareToGroupCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_OnSpatialAnchorsShareToGroupCompleted",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
                crate::GlobalNamespace::OVRAnchor_ShareResult,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_OnSharedSpatialAnchorsLoadCompleted(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Events::UnityEvent_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::OVRSpatialAnchor,
                                    >,
                                >,
                            >,
                            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "set_OnSharedSpatialAnchorsLoadCompleted",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_OnSharedSpatialAnchorsLoadCompleted",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_OnSpatialAnchorsShareCompleted(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Events::UnityEvent_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::OVRSpatialAnchor,
                                    >,
                                >,
                            >,
                            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "set_OnSpatialAnchorsShareCompleted"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_OnSpatialAnchorsShareCompleted",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_OnSpatialAnchorsShareToGroupCompleted(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
                crate::GlobalNamespace::OVRAnchor_ShareResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Events::UnityEvent_2<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::OVRSpatialAnchor,
                                    >,
                                >,
                            >,
                            crate::GlobalNamespace::OVRAnchor_ShareResult,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "set_OnSpatialAnchorsShareToGroupCompleted",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_OnSpatialAnchorsShareToGroupCompleted",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+SharedSpatialAnchorCore")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::BuildingBlocks::SharedSpatialAnchorCore
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
