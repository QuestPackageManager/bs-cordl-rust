#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+_idBuffer_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QueryPairedUserAccountCommand__idBuffer_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+_idBuffer_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand__idBuffer_e__FixedBuffer
    => "UnityEngine.InputSystem.LowLevel"
    ."QueryPairedUserAccountCommand/<idBuffer>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+_idBuffer_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand__idBuffer_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+_idBuffer_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand__idBuffer_e__FixedBuffer {}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+_nameBuffer_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QueryPairedUserAccountCommand__nameBuffer_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+_nameBuffer_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand__nameBuffer_e__FixedBuffer
    => "UnityEngine.InputSystem.LowLevel"
    ."QueryPairedUserAccountCommand/<nameBuffer>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+_nameBuffer_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand__nameBuffer_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+_nameBuffer_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand__nameBuffer_e__FixedBuffer {}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct QueryPairedUserAccountCommand {
    padding: [u8; 1040usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand =>
    "UnityEngine.InputSystem.LowLevel"."QueryPairedUserAccountCommand"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand")]
impl crate::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand {
    pub const kMaxIdLength: i32 = 256i32;
    pub const kMaxNameLength: i32 = 256i32;
    pub const kSize: i32 = 1040i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+_nameBuffer_e__FixedBuffer"
    )]
    pub type _nameBuffer_e__FixedBuffer = crate::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand__nameBuffer_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+_idBuffer_e__FixedBuffer"
    )]
    pub type _idBuffer_e__FixedBuffer = crate::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand__idBuffer_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+Result"
    )]
    pub type Result = crate::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand_Result;
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_id",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_name",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_id(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_id",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_name(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_name",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+Result")]
#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryPairedUserAccountCommand_Result {
    DevicePairedToUserAccount = 2i64,
    UserAccountSelectionCanceled = 16i64,
    UserAccountSelectionComplete = 8i64,
    UserAccountSelectionInProgress = 4i64,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+QueryPairedUserAccountCommand+Result")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::QueryPairedUserAccountCommand_Result =>
    "UnityEngine.InputSystem.LowLevel"."QueryPairedUserAccountCommand/Result"
);