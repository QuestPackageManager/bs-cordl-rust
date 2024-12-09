#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer+_data_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer+_data_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer =>
    "UnityEngine.InputSystem"."InputSystem/DeltaStateEventBuffer/<data>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer+_data_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer+_data_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer {}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct InputSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputSystem =>
    "UnityEngine.InputSystem"."InputSystem"
);
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
impl crate::UnityEngine::InputSystem::InputSystem {
    pub const kAssemblyVersion: &'static str = "1.7.0";
    pub const kDocUrl: &'static str = "https://docs.unity3d.com/Packages/com.unity.inputsystem@1.7";
    #[cfg(feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer")]
    pub type DeltaStateEventBuffer = crate::UnityEngine::InputSystem::InputSystem_DeltaStateEventBuffer;
    #[cfg(feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer")]
    pub type StateEventBuffer = crate::UnityEngine::InputSystem::InputSystem_StateEventBuffer;
    #[cfg(feature = "UnityEngine+InputSystem+InputSystem+__c")]
    pub type __c = crate::UnityEngine::InputSystem::InputSystem___c;
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::InputSystem::InputSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputSystem_DeltaStateEventBuffer {
    pub stateEvent: crate::UnityEngine::InputSystem::LowLevel::DeltaStateEvent,
    pub data: crate::UnityEngine::InputSystem::DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer,
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputSystem_DeltaStateEventBuffer =>
    "UnityEngine.InputSystem"."InputSystem/DeltaStateEventBuffer"
);
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputSystem_DeltaStateEventBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer")]
impl crate::UnityEngine::InputSystem::InputSystem_DeltaStateEventBuffer {
    pub const kMaxSize: i32 = 512i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputSystem+DeltaStateEventBuffer+_data_e__FixedBuffer"
    )]
    pub type _data_e__FixedBuffer = crate::UnityEngine::InputSystem::DeltaStateEventBuffer_InputSystem__data_e__FixedBuffer;
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputSystem_StateEventBuffer {
    pub stateEvent: crate::UnityEngine::InputSystem::LowLevel::StateEvent,
    pub data: crate::UnityEngine::InputSystem::StateEventBuffer_InputSystem__data_e__FixedBuffer,
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputSystem_StateEventBuffer => "UnityEngine.InputSystem"
    ."InputSystem/StateEventBuffer"
);
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputSystem_StateEventBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer")]
impl crate::UnityEngine::InputSystem::InputSystem_StateEventBuffer {
    pub const kMaxSize: i32 = 512i32;
    #[cfg(
        feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer+_data_e__FixedBuffer"
    )]
    pub type _data_e__FixedBuffer = crate::UnityEngine::InputSystem::StateEventBuffer_InputSystem__data_e__FixedBuffer;
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer+_data_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StateEventBuffer_InputSystem__data_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer+_data_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::StateEventBuffer_InputSystem__data_e__FixedBuffer =>
    "UnityEngine.InputSystem"."InputSystem/StateEventBuffer/<data>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer+_data_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::StateEventBuffer_InputSystem__data_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+InputSystem+StateEventBuffer+_data_e__FixedBuffer"
)]
impl crate::UnityEngine::InputSystem::StateEventBuffer_InputSystem__data_e__FixedBuffer {}
