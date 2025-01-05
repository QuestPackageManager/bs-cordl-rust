#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryKeyNameCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct QueryKeyNameCommand {
    padding: [u8; 268usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryKeyNameCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::QueryKeyNameCommand =>
    "UnityEngine.InputSystem.LowLevel"."QueryKeyNameCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryKeyNameCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::QueryKeyNameCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryKeyNameCommand")]
impl crate::UnityEngine::InputSystem::LowLevel::QueryKeyNameCommand {
    pub const kMaxNameLength: i32 = 256i32;
    pub const kSize: i32 = 268i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+QueryKeyNameCommand+_nameBuffer_e__FixedBuffer"
    )]
    pub type _nameBuffer_e__FixedBuffer = crate::UnityEngine::InputSystem::LowLevel::QueryKeyNameCommand__nameBuffer_e__FixedBuffer;
    pub fn Create(
        key: crate::UnityEngine::InputSystem::Key,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::QueryKeyNameCommand,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::QueryKeyNameCommand = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadKeyName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ReadKeyName", ())?;
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryKeyNameCommand")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::QueryKeyNameCommand {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryKeyNameCommand")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo,
    >,
> for crate::UnityEngine::InputSystem::LowLevel::QueryKeyNameCommand {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::LowLevel::IInputDeviceCommandInfo,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryKeyNameCommand+_nameBuffer_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct QueryKeyNameCommand__nameBuffer_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryKeyNameCommand+_nameBuffer_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::QueryKeyNameCommand__nameBuffer_e__FixedBuffer
    => "UnityEngine.InputSystem.LowLevel"
    ."QueryKeyNameCommand/<nameBuffer>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryKeyNameCommand+_nameBuffer_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::QueryKeyNameCommand__nameBuffer_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryKeyNameCommand+_nameBuffer_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::LowLevel::QueryKeyNameCommand__nameBuffer_e__FixedBuffer {}
