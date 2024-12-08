#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchState")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TouchState {
    padding: [u8; 56usize],
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::TouchState
    => "UnityEngine.InputSystem.LowLevel"."TouchState"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::TouchState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+TouchState")]
impl crate::UnityEngine::InputSystem::LowLevel::TouchState {
    pub const kSizeInBytes: i32 = 56i32;
    pub fn set_isIndirectTouch(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isIndirectTouch",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isOrphanedPrimaryTouch(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isOrphanedPrimaryTouch",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isIndirectTouch(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isIndirectTouch",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isTapPress(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isTapPress",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isNoneEndedOrCanceled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isNoneEndedOrCanceled",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isPrimaryTouch(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isPrimaryTouch",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isTapRelease(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isTapRelease",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isTapRelease(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isTapRelease",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_phase(
        &mut self,
        value: crate::UnityEngine::InputSystem::TouchPhase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_phase",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isTap(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isTap",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isPrimaryTouch(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isPrimaryTouch",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isInProgress(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isInProgress",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_isOrphanedPrimaryTouch(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_isOrphanedPrimaryTouch",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isTap(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isTap",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_phase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputSystem::TouchPhase> {
        let __cordl_ret: crate::UnityEngine::InputSystem::TouchPhase = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_phase",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_isTapPress(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isTapPress",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_beganInSameFrame(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_beganInSameFrame",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_beganInSameFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_beganInSameFrame",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_format",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
