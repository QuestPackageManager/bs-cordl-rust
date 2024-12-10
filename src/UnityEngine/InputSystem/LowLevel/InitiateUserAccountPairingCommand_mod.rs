#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InitiateUserAccountPairingCommand")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InitiateUserAccountPairingCommand {
    padding: [u8; 8usize],
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
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+InitiateUserAccountPairingCommand+Result"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitiateUserAccountPairingCommand_Result {
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
