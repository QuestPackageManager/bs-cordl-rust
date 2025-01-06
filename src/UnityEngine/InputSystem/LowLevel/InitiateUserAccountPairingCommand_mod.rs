#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InitiateUserAccountPairingCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InitiateUserAccountPairingCommand {
    padding: quest_hook::libil2cpp::ValueTypePadding<8usize>,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InitiateUserAccountPairingCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InitiateUserAccountPairingCommand =>
    "UnityEngine.InputSystem.LowLevel"."InitiateUserAccountPairingCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InitiateUserAccountPairingCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InitiateUserAccountPairingCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InitiateUserAccountPairingCommand")]
impl crate::UnityEngine::InputSystem::LowLevel::InitiateUserAccountPairingCommand {
    pub const kSize: i32 = 8i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+InitiateUserAccountPairingCommand+Result"
    )]
    pub type Result = crate::UnityEngine::InputSystem::LowLevel::InitiateUserAccountPairingCommand_Result;
    pub fn Create() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InitiateUserAccountPairingCommand,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InitiateUserAccountPairingCommand = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_typeStatic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_typeStatic",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InitiateUserAccountPairingCommand")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
for crate::UnityEngine::InputSystem::LowLevel::InitiateUserAccountPairingCommand {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InitiateUserAccountPairingCommand")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
for crate::UnityEngine::InputSystem::LowLevel::InitiateUserAccountPairingCommand {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InitiateUserAccountPairingCommand+Result"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InitiateUserAccountPairingCommand_Result {
    #[default]
    ErrorAlreadyInProgress = -2i32,
    ErrorNotSupported = -1i32,
    SuccessfullyInitiated = 1i32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InitiateUserAccountPairingCommand+Result"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InitiateUserAccountPairingCommand_Result =>
    "UnityEngine.InputSystem.LowLevel"."InitiateUserAccountPairingCommand/Result"
);
