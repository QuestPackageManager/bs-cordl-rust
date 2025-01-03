#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidDeviceCapabilities")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AndroidDeviceCapabilities {
    pub deviceDescriptor: *mut quest_hook::libil2cpp::Il2CppString,
    pub productId: i32,
    pub vendorId: i32,
    pub isVirtual: bool,
    pub motionAxes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::Android::LowLevel::AndroidAxis,
    >,
    pub inputSources: crate::UnityEngine::InputSystem::Android::LowLevel::AndroidInputSource,
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidDeviceCapabilities")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Android::LowLevel::AndroidDeviceCapabilities =>
    "UnityEngine.InputSystem.Android.LowLevel"."AndroidDeviceCapabilities"
);
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidDeviceCapabilities")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Android::LowLevel::AndroidDeviceCapabilities {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidDeviceCapabilities")]
impl crate::UnityEngine::InputSystem::Android::LowLevel::AndroidDeviceCapabilities {
    #[cfg(
        feature = "UnityEngine+InputSystem+Android+LowLevel+AndroidDeviceCapabilities+__c"
    )]
    pub type __c = crate::UnityEngine::InputSystem::Android::LowLevel::AndroidDeviceCapabilities___c;
    pub fn FromJson(
        json: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Android::LowLevel::AndroidDeviceCapabilities,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Android::LowLevel::AndroidDeviceCapabilities = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromJson", (json))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToJson(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToJson", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
}
