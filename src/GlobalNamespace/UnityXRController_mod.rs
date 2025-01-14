#[cfg(feature = "UnityXRController")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityXRController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub positionAction: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub rotationAction: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub thumbstickAction: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub node: crate::UnityEngine::XR::XRNode,
    pub _manufacturerName_k__BackingField: crate::GlobalNamespace::UnityXRHelper_VRControllerManufacturerName,
    pub _hapticsHandler: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IUnityXRHapticsHandler,
    >,
}
#[cfg(feature = "UnityXRController")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::UnityXRController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UnityXRController";
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
#[cfg(feature = "UnityXRController")]
impl std::ops::Deref for crate::GlobalNamespace::UnityXRController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityXRController")]
impl std::ops::DerefMut for crate::GlobalNamespace::UnityXRController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityXRController")]
impl crate::GlobalNamespace::UnityXRController {
    #[cfg(feature = "UnityXRController+Configuration")]
    pub type Configuration = crate::GlobalNamespace::UnityXRController_Configuration;
    pub fn New(
        node: crate::UnityEngine::XR::XRNode,
        positionAction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        >,
        rotationAction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        >,
        thumbstickAction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (node, positionAction, rotationAction, thumbstickAction),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ResetManufacturerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ResetManufacturerName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ResetManufacturerName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupController(
        &mut self,
        device: crate::UnityEngine::XR::InputDevice,
        coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::XR::InputDevice,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                ),
                bool,
                2usize,
            >("SetupController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetupController", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (device, coroutineRunner))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryToUpdateManufacturerName(
        &mut self,
        device: crate::UnityEngine::XR::InputDevice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::XR::InputDevice),
                bool,
                1usize,
            >("TryToUpdateManufacturerName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryToUpdateManufacturerName", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (device)) };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateHapticsHandler(
        &mut self,
        coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UpdateHapticsHandler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateHapticsHandler", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (coroutineRunner))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        positionAction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        >,
        rotationAction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        >,
        thumbstickAction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::XR::XRNode,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputAction,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputAction,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::InputSystem::InputAction,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (node, positionAction, rotationAction, thumbstickAction),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_hapticsHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IUnityXRHapticsHandler>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IUnityXRHapticsHandler,
                >,
                0usize,
            >("get_hapticsHandler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_hapticsHandler", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IUnityXRHapticsHandler,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_manufacturerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::UnityXRHelper_VRControllerManufacturerName,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::UnityXRHelper_VRControllerManufacturerName,
                0usize,
            >("get_manufacturerName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_manufacturerName", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::UnityXRHelper_VRControllerManufacturerName = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_manufacturerName(
        &mut self,
        value: crate::GlobalNamespace::UnityXRHelper_VRControllerManufacturerName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::UnityXRHelper_VRControllerManufacturerName),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_manufacturerName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_manufacturerName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityXRController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UnityXRController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityXRController+Configuration")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityXRController_Configuration {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _positionActionReference_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionReference,
    >,
    pub _orientationActionReference_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionReference,
    >,
    pub _thumbstickActionReference_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionReference,
    >,
}
#[cfg(feature = "UnityXRController+Configuration")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::UnityXRController_Configuration {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UnityXRController/Configuration";
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
#[cfg(feature = "UnityXRController+Configuration")]
impl std::ops::Deref for crate::GlobalNamespace::UnityXRController_Configuration {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityXRController+Configuration")]
impl std::ops::DerefMut for crate::GlobalNamespace::UnityXRController_Configuration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityXRController+Configuration")]
impl crate::GlobalNamespace::UnityXRController_Configuration {
    pub fn CreateController(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UnityXRController>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::XR::XRNode),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UnityXRController>,
                1usize,
            >("CreateController")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateController", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::UnityXRController,
        > = unsafe { method.invoke_unchecked(self, (node)) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_orientationActionReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionReference,
                >,
                0usize,
            >("get_orientationActionReference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_orientationActionReference", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_positionActionReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionReference,
                >,
                0usize,
            >("get_positionActionReference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_positionActionReference", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_thumbstickActionReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionReference,
                >,
                0usize,
            >("get_thumbstickActionReference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_thumbstickActionReference", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_orientationActionReference(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionReference,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_orientationActionReference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_orientationActionReference", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_positionActionReference(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionReference,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_positionActionReference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_positionActionReference", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_thumbstickActionReference(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::InputSystem::InputActionReference,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_thumbstickActionReference")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_thumbstickActionReference", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityXRController+Configuration")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::UnityXRController_Configuration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
