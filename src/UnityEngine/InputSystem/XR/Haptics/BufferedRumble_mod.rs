#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BufferedRumble {
    pub _capabilities_k__BackingField: crate::UnityEngine::InputSystem::XR::Haptics::HapticCapabilities,
    pub _device_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputDevice,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::XR::Haptics::BufferedRumble {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.XR.Haptics";
    const CLASS_NAME: &'static str = "BufferedRumble";
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
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::XR::Haptics::BufferedRumble {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::XR::Haptics::BufferedRumble {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::XR::Haptics::BufferedRumble {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::XR::Haptics::BufferedRumble {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::Haptics::BufferedRumble {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+BufferedRumble")]
impl crate::UnityEngine::InputSystem::XR::Haptics::BufferedRumble {
    pub fn EnqueueRumble(
        &mut self,
        samples: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "EnqueueRumble",
            (samples),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        device: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (device),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_capabilities(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::XR::Haptics::HapticCapabilities,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::XR::Haptics::HapticCapabilities = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_capabilities",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_device(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputDevice,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_device", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_capabilities(
        &mut self,
        value: crate::UnityEngine::InputSystem::XR::Haptics::HapticCapabilities,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_capabilities",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_device(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_device",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
