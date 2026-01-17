#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+SpatialAnchorCoreBuildingBlock")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct SpatialAnchorCoreBuildingBlock {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _onAnchorCreateCompleted: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Events::UnityEvent_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    >,
    pub _onAnchorsLoadCompleted: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Events::UnityEvent_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                >,
            >,
        >,
    >,
    pub _onAnchorsEraseAllCompleted: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Events::UnityEvent_1<
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    >,
    pub _onAnchorEraseCompleted: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Events::UnityEvent_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
        >,
    >,
    pub _Result_k__BackingField: crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+SpatialAnchorCoreBuildingBlock")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::BuildingBlocks::SpatialAnchorCoreBuildingBlock
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.BuildingBlocks";
    const CLASS_NAME: &'static str = "SpatialAnchorCoreBuildingBlock";
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
#[cfg(feature = "Meta+XR+BuildingBlocks+SpatialAnchorCoreBuildingBlock")]
impl std::ops::Deref for crate::Meta::XR::BuildingBlocks::SpatialAnchorCoreBuildingBlock {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+SpatialAnchorCoreBuildingBlock")]
impl std::ops::DerefMut for crate::Meta::XR::BuildingBlocks::SpatialAnchorCoreBuildingBlock {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+SpatialAnchorCoreBuildingBlock")]
impl crate::Meta::XR::BuildingBlocks::SpatialAnchorCoreBuildingBlock {
    pub fn EraseAllAnchors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("EraseAllAnchors")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EraseAllAnchors",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn EraseAnchorByUuid(
        &mut self,
        uuid: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::Guid), quest_hook::libil2cpp::Void, 1usize>(
                        "EraseAnchorByUuid",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EraseAnchorByUuid",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (uuid))? };
        Ok(__cordl_ret.into())
    }
    pub fn EraseAnchorByUuidAsync(
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
                    >("EraseAnchorByUuidAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EraseAnchorByUuidAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task> =
            unsafe { cordl_method_info.invoke_unchecked(self, (anchor))? };
        Ok(__cordl_ret.into())
    }
    pub fn EraseAnchorsAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("EraseAnchorsAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EraseAnchorsAsync",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFirstInstance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Meta::XR::BuildingBlocks::SpatialAnchorCoreBuildingBlock>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::BuildingBlocks::SpatialAnchorCoreBuildingBlock,
                    >, 0usize>("GetFirstInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFirstInstance",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::BuildingBlocks::SpatialAnchorCoreBuildingBlock,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InitSpatialAnchorAsync(
        &mut self,
        anchor: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRSpatialAnchor,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InitSpatialAnchorAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitSpatialAnchorAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
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
    pub fn LoadAnchorsAsync(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        uuids: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
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
                            crate::System::Collections::Generic::IEnumerable_1<crate::System::Guid>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("LoadAnchorsAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadAnchorsAsync",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (prefab, uuids))? };
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SaveAsync(
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
                    >("SaveAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SaveAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task> =
            unsafe { cordl_method_info.invoke_unchecked(self, (anchor))? };
        Ok(__cordl_ret.into())
    }
    pub fn WaitForInit(
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
                    >("WaitForInit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WaitForInit", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task> =
            unsafe { cordl_method_info.invoke_unchecked(self, (anchor))? };
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
    pub fn get_OnAnchorCreateCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
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
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        >,
                    >, 0usize>("get_OnAnchorCreateCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_OnAnchorCreateCompleted",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_OnAnchorEraseCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
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
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        >,
                    >, 0usize>("get_OnAnchorEraseCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_OnAnchorEraseCompleted",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_OnAnchorsEraseAllCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_1<
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
                        crate::UnityEngine::Events::UnityEvent_1<
                            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        >,
                    >, 0usize>("get_OnAnchorsEraseAllCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_OnAnchorsEraseAllCompleted",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_1<
                crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_OnAnchorsLoadCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Events::UnityEvent_1<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::OVRSpatialAnchor,
                                    >,
                                >,
                            >,
                        >,
                    >, 0usize>("get_OnAnchorsLoadCompleted")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_OnAnchorsLoadCompleted",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Result(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRSpatialAnchor_OperationResult>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        0usize,
                    >("get_Result")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Result", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRSpatialAnchor_OperationResult =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_OnAnchorCreateCompleted(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
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
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "set_OnAnchorCreateCompleted"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_OnAnchorCreateCompleted",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_OnAnchorEraseCompleted(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
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
                            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "set_OnAnchorEraseCompleted"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_OnAnchorEraseCompleted",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_OnAnchorsEraseAllCompleted(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_1<
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
                        crate::UnityEngine::Events::UnityEvent_1<
                            crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "set_OnAnchorsEraseAllCompleted"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_OnAnchorsEraseAllCompleted",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_OnAnchorsLoadCompleted(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSpatialAnchor>,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Events::UnityEvent_1<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::OVRSpatialAnchor,
                                    >,
                                >,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "set_OnAnchorsLoadCompleted"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_OnAnchorsLoadCompleted",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Result(
        &mut self,
        value: crate::GlobalNamespace::OVRSpatialAnchor_OperationResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRSpatialAnchor_OperationResult),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Result")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Result", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+SpatialAnchorCoreBuildingBlock")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::BuildingBlocks::SpatialAnchorCoreBuildingBlock
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
