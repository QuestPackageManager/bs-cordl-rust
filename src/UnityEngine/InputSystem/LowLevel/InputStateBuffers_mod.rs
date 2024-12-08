#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBuffers+DoubleBuffers")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputStateBuffers_DoubleBuffers {
    pub deviceToBufferMapping: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBuffers+DoubleBuffers")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputStateBuffers_DoubleBuffers =>
    "UnityEngine.InputSystem.LowLevel"."InputStateBuffers/DoubleBuffers"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBuffers+DoubleBuffers")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputStateBuffers_DoubleBuffers {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBuffers+DoubleBuffers")]
impl crate::UnityEngine::InputSystem::LowLevel::InputStateBuffers_DoubleBuffers {
    pub fn SetBackBuffer(
        &mut self,
        deviceIndex: i32,
        ptr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetBackBuffer",
            (deviceIndex, ptr),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_valid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_valid",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetFrontBuffer(
        &mut self,
        deviceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetFrontBuffer",
            (deviceIndex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetBackBuffer(
        &mut self,
        deviceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetBackBuffer",
            (deviceIndex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SwapBuffers(
        &mut self,
        deviceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SwapBuffers",
            (deviceIndex),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetFrontBuffer(
        &mut self,
        deviceIndex: i32,
        ptr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFrontBuffer",
            (deviceIndex, ptr),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBuffers")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputStateBuffers {
    pub sizePerBuffer: u32,
    pub totalSize: u32,
    pub defaultStateBuffer: *mut quest_hook::libil2cpp::Il2CppObject,
    pub noiseMaskBuffer: *mut quest_hook::libil2cpp::Il2CppObject,
    pub resetMaskBuffer: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_AllBuffers: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_PlayerStateBuffers: crate::UnityEngine::InputSystem::LowLevel::InputStateBuffers_DoubleBuffers,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBuffers")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::InputStateBuffers =>
    "UnityEngine.InputSystem.LowLevel"."InputStateBuffers"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBuffers")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputStateBuffers {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBuffers")]
impl crate::UnityEngine::InputSystem::LowLevel::InputStateBuffers {
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputStateBuffers+DoubleBuffers")]
    pub type DoubleBuffers = crate::UnityEngine::InputSystem::LowLevel::InputStateBuffers_DoubleBuffers;
    pub fn MigrateAll(
        &mut self,
        devices: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
        deviceCount: i32,
        oldBuffers: crate::UnityEngine::InputSystem::LowLevel::InputStateBuffers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MigrateAll",
            (devices, deviceCount, oldBuffers),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AllocateAll(
        &mut self,
        devices: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::InputSystem::InputDevice,
        >,
        deviceCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AllocateAll",
            (devices, deviceCount),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FreeAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FreeAll",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetDoubleBuffersFor(
        &mut self,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputStateBuffers_DoubleBuffers,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputStateBuffers_DoubleBuffers = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDoubleBuffersFor",
            (updateType),
        )?;
        Ok(__cordl_ret)
    }
}
