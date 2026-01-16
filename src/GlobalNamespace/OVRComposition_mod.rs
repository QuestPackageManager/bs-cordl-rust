#[cfg(feature = "cordl_class_OVRComposition")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRComposition {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub cameraInTrackingSpace: bool,
    pub cameraRig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCameraRig>,
    pub usingLastAttachedNodePose: bool,
    pub lastAttachedNodePose: crate::GlobalNamespace::OVRPose,
}
#[cfg(feature = "cordl_class_OVRComposition")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRComposition {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRComposition";
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
#[cfg(feature = "OVRComposition")]
impl std::ops::Deref for crate::GlobalNamespace::OVRComposition {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRComposition")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRComposition {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRComposition")]
impl crate::GlobalNamespace::OVRComposition {
    pub fn Cleanup(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Cleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Cleanup",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CompositionMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRManager_CompositionMethod> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::OVRManager_CompositionMethod,
                        0usize,
                    >("CompositionMethod")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompositionMethod", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRManager_CompositionMethod =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeCameraTrackingSpacePose(
        &mut self,
        extrinsics: crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::OVRPlugin_CameraExtrinsics),
                        crate::GlobalNamespace::OVRPose,
                        1usize,
                    >("ComputeCameraTrackingSpacePose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeCameraTrackingSpacePose", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose =
            unsafe { cordl_method_info.invoke_unchecked(self, (extrinsics))? };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeCameraWorldSpacePose(
        &mut self,
        extrinsics: crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPose> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::OVRPlugin_CameraExtrinsics,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), crate::GlobalNamespace::OVRPose, 2usize>(
                        "ComputeCameraWorldSpacePose"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ComputeCameraWorldSpacePose",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPose =
            unsafe { cordl_method_info.invoke_unchecked(self, (extrinsics, mainCamera))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parentObject, mainCamera, configuration))?;
        Ok(__cordl_object.into())
    }
    pub fn RecenterPose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("RecenterPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RecenterPose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn RefreshCameraRig(
        &mut self,
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    ), quest_hook::libil2cpp::Void, 2usize>("RefreshCameraRig")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RefreshCameraRig",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (parentObject, mainCamera))? };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
        trackingOrigin: crate::GlobalNamespace::OVRManager_TrackingOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
                        >,
                        crate::GlobalNamespace::OVRManager_TrackingOrigin,
                    ), quest_hook::libil2cpp::Void, 4usize>("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Update",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (gameObject, mainCamera, configuration, trackingOrigin),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        parentObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        mainCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        configuration: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::OVRMixedRealityCaptureConfiguration,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (parentObject, mainCamera, configuration))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRComposition")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRComposition {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
