#[cfg(feature = "FirstPersonFlyingController")]
#[repr(C)]
#[derive(Debug)]
pub struct FirstPersonFlyingController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _moveSensitivity: f32,
    pub _transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _cameraFov: f32,
    pub _centerAdjust: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRCenterAdjust>,
    pub _controller0: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
    pub _controller1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
    pub _controllerModels: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    >,
    pub _mouseLook: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MouseLook>,
    pub _trackedPoseDriver: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::SpatialTracking::TrackedPoseDriver,
    >,
    pub _cameraTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _originalStereoTargetEyeMask: crate::UnityEngine::StereoTargetEyeMask,
    pub _originalCameraFov: f32,
    pub _overrideDisableMovement: bool,
}
#[cfg(feature = "FirstPersonFlyingController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FirstPersonFlyingController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FirstPersonFlyingController";
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
#[cfg(feature = "FirstPersonFlyingController")]
impl std::ops::Deref for crate::GlobalNamespace::FirstPersonFlyingController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FirstPersonFlyingController")]
impl std::ops::DerefMut for crate::GlobalNamespace::FirstPersonFlyingController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FirstPersonFlyingController")]
impl crate::GlobalNamespace::FirstPersonFlyingController {
    pub fn Inject(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        centerAdjust: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRCenterAdjust>,
        controller0: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
        controller1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
        trackedPoseDriver: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::SpatialTracking::TrackedPoseDriver,
        >,
        overrideDisableMovement: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::FirstPersonFlyingController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRCenterAdjust>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::SpatialTracking::TrackedPoseDriver,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("Inject")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::FirstPersonFlyingController as
                    quest_hook::libil2cpp::Type > ::class(), "Inject", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        camera,
                        centerAdjust,
                        controller0,
                        controller1,
                        trackedPoseDriver,
                        overrideDisableMovement,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::FirstPersonFlyingController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::FirstPersonFlyingController as
                    quest_hook::libil2cpp::Type > ::class(), "OnDisable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::FirstPersonFlyingController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::FirstPersonFlyingController as
                    quest_hook::libil2cpp::Type > ::class(), "OnEnable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::FirstPersonFlyingController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::FirstPersonFlyingController as
                    quest_hook::libil2cpp::Type > ::class(), "Update", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::FirstPersonFlyingController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::FirstPersonFlyingController as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FirstPersonFlyingController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FirstPersonFlyingController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
