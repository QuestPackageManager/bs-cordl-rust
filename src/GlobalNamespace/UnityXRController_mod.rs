#[cfg(feature = "cordl_class_UnityXRController")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityXRController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub positionAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    pub rotationAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    pub thumbstickAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    pub node: crate::UnityEngine::XR::XRNode,
    pub _hapticsHandler_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IUnityXRHapticsHandler>,
    pub _manufacturer_k__BackingField: crate::GlobalNamespace::VRControllerManufacturer,
}
#[cfg(feature = "cordl_class_UnityXRController")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::UnityXRController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UnityXRController";
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
#[cfg(feature = "UnityXRController")]
impl std::ops::Deref for crate::GlobalNamespace::UnityXRController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityXRController")]
impl std::ops::DerefMut for crate::GlobalNamespace::UnityXRController {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityXRController")]
impl crate::GlobalNamespace::UnityXRController {
    pub fn New(
        node: crate::UnityEngine::XR::XRNode,
        positionAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        rotationAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        thumbstickAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (node, positionAction, rotationAction, thumbstickAction),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn ResetManufacturerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ResetManufacturerName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResetManufacturerName",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupController(
        &mut self,
        device: crate::UnityEngine::XR::InputDevice,
        coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::XR::InputDevice,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                    ), bool, 2usize>("SetupController")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupController",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (device, coroutineRunner))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryToUpdateManufacturerName(
        &mut self,
        device: crate::UnityEngine::XR::InputDevice,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::XR::InputDevice), bool, 1usize>(
                        "TryToUpdateManufacturerName",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryToUpdateManufacturerName",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (device))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateHapticsHandler(
        &mut self,
        coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateHapticsHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateHapticsHandler", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (coroutineRunner))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        node: crate::UnityEngine::XR::XRNode,
        positionAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        rotationAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        thumbstickAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::XR::XRNode,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (node, positionAction, rotationAction, thumbstickAction),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_hapticsHandler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IUnityXRHapticsHandler>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_hapticsHandler", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IUnityXRHapticsHandler> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_manufacturer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::VRControllerManufacturer> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::VRControllerManufacturer, 0usize>(
                        "get_manufacturer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_manufacturer",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::VRControllerManufacturer =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_hapticsHandler(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IUnityXRHapticsHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IUnityXRHapticsHandler,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_hapticsHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_hapticsHandler", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_manufacturer(
        &mut self,
        value: crate::GlobalNamespace::VRControllerManufacturer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::VRControllerManufacturer),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_manufacturer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_manufacturer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityXRController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UnityXRController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
