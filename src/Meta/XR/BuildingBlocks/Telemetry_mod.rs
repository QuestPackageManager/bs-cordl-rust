#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+Telemetry")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct Telemetry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+Telemetry")]
unsafe impl quest_hook::libil2cpp::Type for crate::Meta::XR::BuildingBlocks::Telemetry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.BuildingBlocks";
    const CLASS_NAME: &'static str = "Telemetry";
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
#[cfg(feature = "Meta+XR+BuildingBlocks+Telemetry")]
impl std::ops::Deref for crate::Meta::XR::BuildingBlocks::Telemetry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+Telemetry")]
impl std::ops::DerefMut for crate::Meta::XR::BuildingBlocks::Telemetry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+Telemetry")]
impl crate::Meta::XR::BuildingBlocks::Telemetry {
    pub fn AddBlockInfo(
        marker: crate::GlobalNamespace::OVRTelemetryMarker,
        block: quest_hook::libil2cpp::Gc<crate::Meta::XR::BuildingBlocks::BuildingBlock>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTelemetryMarker,
                        quest_hook::libil2cpp::Gc<crate::Meta::XR::BuildingBlocks::BuildingBlock>,
                    ), crate::GlobalNamespace::OVRTelemetryMarker, 2usize>(
                        "AddBlockInfo"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddBlockInfo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker =
            unsafe { cordl_method_info.invoke_unchecked((), (marker, block))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddBlockVariantInfo(
        marker: crate::GlobalNamespace::OVRTelemetryMarker,
        block: quest_hook::libil2cpp::Gc<crate::Meta::XR::BuildingBlocks::BuildingBlock>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTelemetryMarker,
                        quest_hook::libil2cpp::Gc<crate::Meta::XR::BuildingBlocks::BuildingBlock>,
                    ), crate::GlobalNamespace::OVRTelemetryMarker, 2usize>(
                        "AddBlockVariantInfo"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddBlockVariantInfo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker =
            unsafe { cordl_method_info.invoke_unchecked((), (marker, block))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddInstallationRoutineInfo(
        marker: crate::GlobalNamespace::OVRTelemetryMarker,
        checkpoint: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::BuildingBlocks::InstallationRoutineCheckpoint,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTelemetryMarker,
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::BuildingBlocks::InstallationRoutineCheckpoint,
                        >,
                    ), crate::GlobalNamespace::OVRTelemetryMarker, 2usize>(
                        "AddInstallationRoutineInfo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddInstallationRoutineInfo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker =
            unsafe { cordl_method_info.invoke_unchecked((), (marker, checkpoint))? };
        Ok(__cordl_ret.into())
    }
    pub fn AddSceneInfo(
        marker: crate::GlobalNamespace::OVRTelemetryMarker,
        scene: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRTelemetryMarker> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRTelemetryMarker,
                        crate::UnityEngine::SceneManagement::Scene,
                    ), crate::GlobalNamespace::OVRTelemetryMarker, 2usize>(
                        "AddSceneInfo"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddSceneInfo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRTelemetryMarker =
            unsafe { cordl_method_info.invoke_unchecked((), (marker, scene))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+Telemetry")]
impl quest_hook::libil2cpp::ObjectType for crate::Meta::XR::BuildingBlocks::Telemetry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
