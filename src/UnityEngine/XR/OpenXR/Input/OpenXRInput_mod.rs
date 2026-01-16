#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput")]
#[repr(C)]
#[derive(Debug)]
pub struct OpenXRInput {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.Input";
    const CLASS_NAME: &'static str = "OpenXRInput";
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
#[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput")]
impl std::ops::Deref for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput")]
impl std::ops::DerefMut for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput")]
impl crate::UnityEngine::XR::OpenXR::Input::OpenXRInput {
    pub const Library: &'static str = "UnityOpenXR";
    pub const s_devicePoseActionName: &'static str = "devicepose";
    pub const s_pointerActionName: &'static str = "pointer";
    #[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput+GetInternalDeviceIdCommand")]
    pub type GetInternalDeviceIdCommand =
        crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand;
    #[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput+InputSourceNameFlags")]
    pub type InputSourceNameFlags =
        crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_InputSourceNameFlags;
    #[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedBinding")]
    pub type SerializedBinding =
        crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding;
    #[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedGuid")]
    pub type SerializedGuid = crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid;
    pub fn AttachActionSets() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "AttachActionSets",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AttachActionSets",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateActions(
        actionMaps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::XR::OpenXR::Features::OpenXRInteractionFeature_ActionMapConfig,
                >,
            >,
        >,
        interactionProfiles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::XR::OpenXR::Features::OpenXRInteractionFeature_ActionMapConfig,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Collections::Generic::List_1<
                                            crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        bool,
                        2usize,
                    >("CreateActions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateActions", 2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (actionMaps, interactionProfiles))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionHandleName(
        control: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputControl>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputControl,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("GetActionHandleName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetActionHandleName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (control))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionHandle_InputAction_InputDevice2(
        inputAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        inputDevice: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), u64, 2usize>("GetActionHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetActionHandle",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u64 =
            unsafe { cordl_method_info.invoke_unchecked((), (inputAction, inputDevice))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionHandle_InputDevice_Il2CppString1(
        device: crate::UnityEngine::XR::InputDevice,
        usageName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::InputDevice,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), u64, 2usize>("GetActionHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetActionHandle",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u64 =
            unsafe { cordl_method_info.invoke_unchecked((), (device, usageName))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionHandle_InputDevice_InputFeatureUsage0(
        device: crate::UnityEngine::XR::InputDevice,
        usage: crate::UnityEngine::XR::InputFeatureUsage,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::InputDevice,
                        crate::UnityEngine::XR::InputFeatureUsage,
                    ), u64, 2usize>("GetActionHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetActionHandle",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked((), (device, usage))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionIsActive_InputAction0(
        inputAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputAction,
                        >),
                        bool,
                        1usize,
                    >("GetActionIsActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetActionIsActive", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (inputAction))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionIsActive_InputDevice_Il2CppString2(
        device: crate::UnityEngine::XR::InputDevice,
        usageName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::InputDevice,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 2usize>("GetActionIsActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetActionIsActive",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (device, usageName))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetActionIsActive_InputDevice_InputFeatureUsage1(
        device: crate::UnityEngine::XR::InputDevice,
        usage: crate::UnityEngine::XR::InputFeatureUsage,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::InputDevice,
                        crate::UnityEngine::XR::InputFeatureUsage,
                    ), bool, 2usize>("GetActionIsActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetActionIsActive",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (device, usage))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDeviceId_InputDevice0(
        inputDevice: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputDevice,
                        >),
                        u32,
                        1usize,
                    >("GetDeviceId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDeviceId", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (inputDevice))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetDeviceId_InputDevice1(
        inputDevice: crate::UnityEngine::XR::InputDevice,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::XR::InputDevice), u32, 1usize>(
                        "GetDeviceId",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetDeviceId",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (inputDevice))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_AttachActionSets() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("Internal_AttachActionSets")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_AttachActionSets",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CreateAction(
        actionSetId: u64,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localizedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        actionType: u32,
        guid: crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid,
        userPaths: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        userPathCount: u32,
        isAdditive: bool,
        usages: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        usageCount: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u64,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        u32,
                        crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                            >,
                        >,
                        u32,
                        bool,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                            >,
                        >,
                        u32,
                    ), u64, 10usize>("Internal_CreateAction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_CreateAction",
                            10usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    actionSetId,
                    name,
                    localizedName,
                    actionType,
                    guid,
                    userPaths,
                    userPathCount,
                    isAdditive,
                    usages,
                    usageCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_CreateActionSet(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localizedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        guid: crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid,
                    ), u64, 3usize>("Internal_CreateActionSet")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_CreateActionSet",
                            3usize
                        )
                    })
            });
        let __cordl_ret: u64 =
            unsafe { cordl_method_info.invoke_unchecked((), (name, localizedName, guid))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetActionId(
        deviceId: u32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), u64, 2usize>("Internal_GetActionId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_GetActionId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked((), (deviceId, name))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetActionIdNoISX(
        deviceId: u32,
        usageName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), u64, 2usize>("Internal_GetActionIdNoISX")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_GetActionIdNoISX",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u64 =
            unsafe { cordl_method_info.invoke_unchecked((), (deviceId, usageName))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetActionIsActive(
        deviceId: u32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 2usize>("Internal_GetActionIsActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_GetActionIsActive",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (deviceId, name))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetActionIsActiveNoISX(
        deviceId: u32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 2usize>("Internal_GetActionIsActiveNoISX")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_GetActionIsActiveNoISX",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (deviceId, name))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetDeviceId(
        characteristics: crate::UnityEngine::XR::InputDeviceCharacteristics,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::InputDeviceCharacteristics,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), u32, 2usize>("Internal_GetDeviceId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_GetDeviceId",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u32 =
            unsafe { cordl_method_info.invoke_unchecked((), (characteristics, name))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_RegisterDeviceDefinition(
        userPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        interactionProfile: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isAdditive: bool,
        characteristics: u32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        manufacturer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serialNumber: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                        u32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), u64, 7usize>("Internal_RegisterDeviceDefinition")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_RegisterDeviceDefinition",
                            7usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    userPath,
                    interactionProfile,
                    isAdditive,
                    characteristics,
                    name,
                    manufacturer,
                    serialNumber,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SendHapticImpulse(
        deviceId: u32,
        actionId: u64,
        amplitude: f32,
        frequency: f32,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u32, u64, f32, f32, f32),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("Internal_SendHapticImpulse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SendHapticImpulse", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (deviceId, actionId, amplitude, frequency, duration))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SendHapticImpulseNoISX(
        deviceId: u32,
        amplitude: f32,
        frequency: f32,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (u32, f32, f32, f32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Internal_SendHapticImpulseNoISX")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SendHapticImpulseNoISX", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (deviceId, amplitude, frequency, duration))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetDpadBindingCustomValues(
        isLeft: bool,
        forceThreshold: f32,
        forceThresholdReleased: f32,
        centerRegion: f32,
        wedgeAngle: f32,
        isSticky: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool, f32, f32, f32, f32, bool),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("Internal_SetDpadBindingCustomValues")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SetDpadBindingCustomValues", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    isLeft,
                    forceThreshold,
                    forceThresholdReleased,
                    centerRegion,
                    wedgeAngle,
                    isSticky,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_StopHaptics(
        deviceId: u32,
        actionId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32, u64), quest_hook::libil2cpp::Void, 2usize>(
                        "Internal_StopHaptics",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_StopHaptics",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (deviceId, actionId))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_StopHapticsNoISX(
        deviceId: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32), quest_hook::libil2cpp::Void, 1usize>(
                        "Internal_StopHapticsNoISX",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_StopHapticsNoISX",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (deviceId))? };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SuggestBindings(
        interactionProfile: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        serializedBindings: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding,
            >,
        >,
        serializedBindingCount: u32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding,
                                >,
                            >,
                            u32,
                        ),
                        bool,
                        3usize,
                    >("Internal_SuggestBindings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Internal_SuggestBindings", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    interactionProfile,
                    serializedBindings,
                    serializedBindingCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_TryGetInputSourceName(
        deviceId: u32,
        actionId: u64,
        index: u32,
        flags: u32,
        outName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        u64,
                        u32,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                    ), bool, 5usize>("Internal_TryGetInputSourceName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_TryGetInputSourceName",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (deviceId, actionId, index, flags, outName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_TryGetInputSourceNamePtr(
        deviceId: u32,
        actionId: u64,
        index: u32,
        flags: u32,
        outName: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        u64,
                        u32,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                    ), bool, 5usize>("Internal_TryGetInputSourceNamePtr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_TryGetInputSourceNamePtr",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (deviceId, actionId, index, flags, outName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Internal_TrySetControllerLateLatchAction(
        deviceId: u32,
        actionId: u64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32, u64), bool, 2usize>(
                        "Internal_TrySetControllerLateLatchAction",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Internal_TrySetControllerLateLatchAction",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (deviceId, actionId))? };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDevices(
        actionMaps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::XR::OpenXR::Features::OpenXRInteractionFeature_ActionMapConfig,
                >,
            >,
        >,
        isAdditive: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::XR::OpenXR::Features::OpenXRInteractionFeature_ActionMapConfig,
                                    >,
                                >,
                            >,
                            bool,
                        ),
                        bool,
                        2usize,
                    >("RegisterDevices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterDevices", 2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (actionMaps, isAdditive))? };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterLayouts() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "RegisterLayouts",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RegisterLayouts",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SanitizeCharForOpenXRPath(c: char) -> quest_hook::libil2cpp::Result<char> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(char), char, 1usize>("SanitizeCharForOpenXRPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SanitizeCharForOpenXRPath",
                            1usize
                        )
                    })
            });
        let __cordl_ret: char = unsafe { cordl_method_info.invoke_unchecked((), (c))? };
        Ok(__cordl_ret.into())
    }
    pub fn SanitizeStringForOpenXRPath(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("SanitizeStringForOpenXRPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SanitizeStringForOpenXRPath", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendHapticImpulse_InputActionReference_InputDevice0(
        actionRef: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
        amplitude: f32,
        duration: f32,
        inputDevice: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputActionReference,
                        >,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), quest_hook::libil2cpp::Void, 4usize>("SendHapticImpulse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendHapticImpulse",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (actionRef, amplitude, duration, inputDevice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendHapticImpulse_InputActionReference_f32_InputDevice1(
        actionRef: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
        amplitude: f32,
        frequency: f32,
        duration: f32,
        inputDevice: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputActionReference,
                        >,
                        f32,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), quest_hook::libil2cpp::Void, 5usize>("SendHapticImpulse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendHapticImpulse",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (actionRef, amplitude, frequency, duration, inputDevice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendHapticImpulse_InputAction_InputDevice2(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        amplitude: f32,
        duration: f32,
        inputDevice: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), quest_hook::libil2cpp::Void, 4usize>("SendHapticImpulse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendHapticImpulse",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (action, amplitude, duration, inputDevice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendHapticImpulse_InputAction_f32_InputDevice3(
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        amplitude: f32,
        frequency: f32,
        duration: f32,
        inputDevice: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
                        f32,
                        f32,
                        f32,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), quest_hook::libil2cpp::Void, 5usize>("SendHapticImpulse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendHapticImpulse",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (action, amplitude, frequency, duration, inputDevice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendHapticImpulse_InputDevice_f32_4(
        device: crate::UnityEngine::XR::InputDevice,
        amplitude: f32,
        frequency: f32,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::XR::InputDevice, f32, f32, f32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("SendHapticImpulse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendHapticImpulse", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (device, amplitude, frequency, duration))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDpadBindingCustomValues() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "SetDpadBindingCustomValues",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetDpadBindingCustomValues",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn StopHapticImpulse(
        device: crate::UnityEngine::XR::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::XR::InputDevice),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("StopHapticImpulse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StopHapticImpulse", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (device))? };
        Ok(__cordl_ret.into())
    }
    pub fn StopHaptics_InputAction1(
        inputAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        inputDevice: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), quest_hook::libil2cpp::Void, 2usize>("StopHaptics")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "StopHaptics",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (inputAction, inputDevice))? };
        Ok(__cordl_ret.into())
    }
    pub fn StopHaptics_InputActionReference0(
        actionRef: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
        inputDevice: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputActionReference,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), quest_hook::libil2cpp::Void, 2usize>("StopHaptics")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "StopHaptics",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (actionRef, inputDevice))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInputSourceName(
        inputAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
        index: i32,
        name: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        flags: crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_InputSourceNameFlags,
        inputDevice: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                        crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_InputSourceNameFlags,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
                    ), bool, 5usize>("TryGetInputSourceName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetInputSourceName",
                            5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (inputAction, index, name, flags, inputDevice))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TrySetControllerLateLatchAction_InputAction0(
        inputAction: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputSystem::InputAction,
                        >),
                        bool,
                        1usize,
                    >("TrySetControllerLateLatchAction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TrySetControllerLateLatchAction", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (inputAction))? };
        Ok(__cordl_ret.into())
    }
    pub fn TrySetControllerLateLatchAction_InputDevice_Il2CppString2(
        device: crate::UnityEngine::XR::InputDevice,
        usageName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::InputDevice,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 2usize>("TrySetControllerLateLatchAction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TrySetControllerLateLatchAction",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (device, usageName))? };
        Ok(__cordl_ret.into())
    }
    pub fn TrySetControllerLateLatchAction_InputDevice_InputFeatureUsage1(
        device: crate::UnityEngine::XR::InputDevice,
        usage: crate::UnityEngine::XR::InputFeatureUsage,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::XR::InputDevice,
                        crate::UnityEngine::XR::InputFeatureUsage,
                    ), bool, 2usize>("TrySetControllerLateLatchAction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TrySetControllerLateLatchAction",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (device, usage))? };
        Ok(__cordl_ret.into())
    }
    pub fn UserPathToDeviceName(
        userPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("UserPathToDeviceName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UserPathToDeviceName", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (userPath))? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateActionMapConfig(
        interactionFeature: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::XR::OpenXR::Features::OpenXRInteractionFeature,
        >,
        actionMapConfig: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::XR::OpenXR::Features::OpenXRInteractionFeature_ActionMapConfig,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::XR::OpenXR::Features::OpenXRInteractionFeature,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::XR::OpenXR::Features::OpenXRInteractionFeature_ActionMapConfig,
                            >,
                        ),
                        bool,
                        2usize,
                    >("ValidateActionMapConfig")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateActionMapConfig", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (interactionFeature, actionMapConfig))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+GetInternalDeviceIdCommand")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OpenXRInput_GetInternalDeviceIdCommand {
    padding: quest_hook::libil2cpp::ValueTypePadding<12usize>,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+GetInternalDeviceIdCommand")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.Input";
    const CLASS_NAME: &'static str = "OpenXRInput/GetInternalDeviceIdCommand";
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+GetInternalDeviceIdCommand")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+GetInternalDeviceIdCommand")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+GetInternalDeviceIdCommand")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+GetInternalDeviceIdCommand")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+GetInternalDeviceIdCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput+GetInternalDeviceIdCommand")]
impl crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand {
    pub const k_BaseCommandSizeSize: i32 = 8i32;
    pub const k_Size: i32 = 12i32;
    pub fn Create() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand,
                        0usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::FourCC> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::InputSystem::Utilities::FourCC,
                        0usize,
                    >("get_Type")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Type", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_typeStatic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::Utilities::FourCC> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::InputSystem::Utilities::FourCC, 0usize>(
                        "get_typeStatic",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_typeStatic",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput+GetInternalDeviceIdCommand")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand
{
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput+GetInternalDeviceIdCommand")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_GetInternalDeviceIdCommand
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+InputSourceNameFlags")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum OpenXRInput_InputSourceNameFlags {
    #[default]
    All = 7i32,
    Component = 4i32,
    InteractionProfile = 2i32,
    UserPath = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+InputSourceNameFlags")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_InputSourceNameFlags
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.Input";
    const CLASS_NAME: &'static str = "OpenXRInput/InputSourceNameFlags";
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+InputSourceNameFlags")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_InputSourceNameFlags
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+InputSourceNameFlags")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_InputSourceNameFlags
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+InputSourceNameFlags")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_InputSourceNameFlags
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+InputSourceNameFlags")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_InputSourceNameFlags
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedBinding")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OpenXRInput_SerializedBinding {
    pub actionId: u64,
    pub path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedBinding")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.Input";
    const CLASS_NAME: &'static str = "OpenXRInput/SerializedBinding";
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedBinding")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedBinding")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedBinding")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedBinding")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedBinding")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedBinding")]
impl crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedBinding {}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedGuid")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OpenXRInput_SerializedGuid {
    padding: quest_hook::libil2cpp::ValueTypePadding<16usize>,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedGuid")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.Input";
    const CLASS_NAME: &'static str = "OpenXRInput/SerializedGuid";
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedGuid")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedGuid")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedGuid")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedGuid")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedGuid")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+OpenXR+Input+OpenXRInput+SerializedGuid")]
impl crate::UnityEngine::XR::OpenXR::Input::OpenXRInput_SerializedGuid {}
