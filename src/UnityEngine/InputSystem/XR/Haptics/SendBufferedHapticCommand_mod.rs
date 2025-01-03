#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendBufferedHapticCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SendBufferedHapticCommand {
    padding: [u8; 1040usize],
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendBufferedHapticCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XR::Haptics::SendBufferedHapticCommand =>
    "UnityEngine.InputSystem.XR.Haptics"."SendBufferedHapticCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendBufferedHapticCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::Haptics::SendBufferedHapticCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendBufferedHapticCommand")]
impl crate::UnityEngine::InputSystem::XR::Haptics::SendBufferedHapticCommand {
    pub const kMaxHapticBufferSize: i32 = 1024i32;
    pub const kSize: i32 = 1040i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+XR+Haptics+SendBufferedHapticCommand+_buffer_e__FixedBuffer"
    )]
    pub type _buffer_e__FixedBuffer = crate::UnityEngine::InputSystem::XR::Haptics::SendBufferedHapticCommand__buffer_e__FixedBuffer;
    pub fn Create(
        rumbleBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::XR::Haptics::SendBufferedHapticCommand,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::XR::Haptics::SendBufferedHapticCommand = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (rumbleBuffer))?;
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
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendBufferedHapticCommand")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
for crate::UnityEngine::InputSystem::XR::Haptics::SendBufferedHapticCommand {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+Haptics+SendBufferedHapticCommand")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
for crate::UnityEngine::InputSystem::XR::Haptics::SendBufferedHapticCommand {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+XR+Haptics+SendBufferedHapticCommand+_buffer_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SendBufferedHapticCommand__buffer_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+XR+Haptics+SendBufferedHapticCommand+_buffer_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XR::Haptics::SendBufferedHapticCommand__buffer_e__FixedBuffer
    => "UnityEngine.InputSystem.XR.Haptics"
    ."SendBufferedHapticCommand/<buffer>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+XR+Haptics+SendBufferedHapticCommand+_buffer_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::Haptics::SendBufferedHapticCommand__buffer_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+XR+Haptics+SendBufferedHapticCommand+_buffer_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::XR::Haptics::SendBufferedHapticCommand__buffer_e__FixedBuffer {}
