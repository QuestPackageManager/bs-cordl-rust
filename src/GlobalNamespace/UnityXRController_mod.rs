#[cfg(feature = "UnityXRController")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityXRController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub positionAction: *mut crate::UnityEngine::InputSystem::InputAction,
    pub rotationAction: *mut crate::UnityEngine::InputSystem::InputAction,
    pub thumbstickAction: *mut crate::UnityEngine::InputSystem::InputAction,
    pub node: crate::UnityEngine::XR::XRNode,
    pub _manufacturerName_k__BackingField: crate::GlobalNamespace::UnityXRHelper_VRControllerManufacturerName,
    pub _hapticsHandler: *mut crate::GlobalNamespace::IUnityXRHapticsHandler,
}
#[cfg(feature = "UnityXRController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UnityXRController => ""
    ."UnityXRController"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetManufacturerName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupController(
        &mut self,
        device: crate::UnityEngine::XR::InputDevice,
        coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetupController", (device, coroutineRunner))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryToUpdateManufacturerName(
        &mut self,
        device: crate::UnityEngine::XR::InputDevice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryToUpdateManufacturerName", (device))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateHapticsHandler(
        &mut self,
        coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateHapticsHandler", (coroutineRunner))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node, positionAction, rotationAction, thumbstickAction))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hapticsHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IUnityXRHapticsHandler>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IUnityXRHapticsHandler,
        > = __cordl_object.invoke("get_hapticsHandler", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_manufacturerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::UnityXRHelper_VRControllerManufacturerName,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::UnityXRHelper_VRControllerManufacturerName = __cordl_object
            .invoke("get_manufacturerName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_manufacturerName(
        &mut self,
        value: crate::GlobalNamespace::UnityXRHelper_VRControllerManufacturerName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_manufacturerName", (value))?;
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
    pub _positionActionReference_k__BackingField: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub _orientationActionReference_k__BackingField: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub _thumbstickActionReference_k__BackingField: *mut crate::UnityEngine::InputSystem::InputActionReference,
}
#[cfg(feature = "UnityXRController+Configuration")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UnityXRController_Configuration
    => ""."UnityXRController/Configuration"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::UnityXRController,
        > = __cordl_object.invoke("CreateController", (node))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_orientationActionReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        > = __cordl_object.invoke("get_orientationActionReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_positionActionReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        > = __cordl_object.invoke("get_positionActionReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_thumbstickActionReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        > = __cordl_object.invoke("get_thumbstickActionReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_orientationActionReference(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_orientationActionReference", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_positionActionReference(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_positionActionReference", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_thumbstickActionReference(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionReference,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_thumbstickActionReference", (value))?;
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
