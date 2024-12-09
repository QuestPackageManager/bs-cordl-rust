#[cfg(feature = "UnityEngine+InputSystem+Users+InputUserAccountHandle")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputUserAccountHandle {
    pub m_ApiName: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Handle: u64,
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUserAccountHandle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Users::InputUserAccountHandle =>
    "UnityEngine.InputSystem.Users"."InputUserAccountHandle"
);
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUserAccountHandle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Users::InputUserAccountHandle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Users+InputUserAccountHandle")]
impl crate::UnityEngine::InputSystem::Users::InputUserAccountHandle {
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_InputUserAccountHandle0(
        &mut self,
        other: crate::UnityEngine::InputSystem::Users::InputUserAccountHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        apiName: *mut quest_hook::libil2cpp::Il2CppString,
        handle: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (apiName, handle),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_apiName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_apiName",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_handle(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_handle",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
