#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryUserIdCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct QueryUserIdCommand {
    padding: [u8; 520usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryUserIdCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::QueryUserIdCommand =>
    "UnityEngine.InputSystem.LowLevel"."QueryUserIdCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryUserIdCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::QueryUserIdCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryUserIdCommand")]
impl crate::UnityEngine::InputSystem::LowLevel::QueryUserIdCommand {
    pub const kMaxIdLength: i32 = 256i32;
    pub const kSize: i32 = 520i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+QueryUserIdCommand+_idBuffer_e__FixedBuffer"
    )]
    pub type _idBuffer_e__FixedBuffer = crate::UnityEngine::InputSystem::LowLevel::QueryUserIdCommand__idBuffer_e__FixedBuffer;
    pub fn Create() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::QueryUserIdCommand,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::QueryUserIdCommand = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ReadId", ())?;
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryUserIdCommand")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
for crate::UnityEngine::InputSystem::LowLevel::QueryUserIdCommand {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryUserIdCommand")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo>
for crate::UnityEngine::InputSystem::LowLevel::QueryUserIdCommand {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryUserIdCommand+_idBuffer_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct QueryUserIdCommand__idBuffer_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryUserIdCommand+_idBuffer_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::QueryUserIdCommand__idBuffer_e__FixedBuffer =>
    "UnityEngine.InputSystem.LowLevel"."QueryUserIdCommand/<idBuffer>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryUserIdCommand+_idBuffer_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::QueryUserIdCommand__idBuffer_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryUserIdCommand+_idBuffer_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::LowLevel::QueryUserIdCommand__idBuffer_e__FixedBuffer {}
