#[cfg(feature = "UnityEngine+InputSystem+LowLevel+NativeInputRuntime")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeInputRuntime {
    __cordl_parent: crate::System::Object,
    pub m_RunInBackground: bool,
    pub m_ShutdownMethod: *mut crate::System::Action,
    pub m_OnUpdate: *mut crate::UnityEngine::InputSystem::LowLevel::InputUpdateDelegate,
    pub m_OnBeforeUpdate: *mut crate::System::Action_1<
        crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    >,
    pub m_OnShouldRunUpdate: *mut crate::System::Func_2<
        crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
        bool,
    >,
    pub m_PollingFrequency: f32,
    pub m_DidCallOnShutdown: bool,
    pub m_FocusChangedMethod: *mut crate::System::Action_1<bool>,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+NativeInputRuntime")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::LowLevel::NativeInputRuntime =>
    "UnityEngine.InputSystem.LowLevel"."NativeInputRuntime"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+NativeInputRuntime")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::LowLevel::NativeInputRuntime {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+NativeInputRuntime")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::LowLevel::NativeInputRuntime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+NativeInputRuntime")]
impl crate::UnityEngine::InputSystem::LowLevel::NativeInputRuntime {
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+NativeInputRuntime+__c__DisplayClass10_0"
    )]
    pub type __c__DisplayClass10_0 = crate::UnityEngine::InputSystem::LowLevel::NativeInputRuntime___c__DisplayClass10_0;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+NativeInputRuntime+__c__DisplayClass13_0"
    )]
    pub type __c__DisplayClass13_0 = crate::UnityEngine::InputSystem::LowLevel::NativeInputRuntime___c__DisplayClass13_0;
    #[cfg(
        feature = "UnityEngine+InputSystem+LowLevel+NativeInputRuntime+__c__DisplayClass7_0"
    )]
    pub type __c__DisplayClass7_0 = crate::UnityEngine::InputSystem::LowLevel::NativeInputRuntime___c__DisplayClass7_0;
    pub fn AllocateDeviceId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("AllocateDeviceId", ())?;
        Ok(__cordl_ret)
    }
    pub fn DeviceCommand(
        &mut self,
        deviceId: i32,
        commandPtr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("DeviceCommand", (deviceId, commandPtr))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnFocusChanged(
        &mut self,
        focus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusChanged", (focus))?;
        Ok(__cordl_ret)
    }
    pub fn OnShutdown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnShutdown", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnWantsToShutdown(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("OnWantsToShutdown", ())?;
        Ok(__cordl_ret)
    }
    pub fn QueueEvent(
        &mut self,
        ptr: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueEvent", (ptr))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterAnalyticsEvent(
        &mut self,
        name: *mut crate::System::String,
        maxPerHour: i32,
        maxPropertiesPerEvent: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RegisterAnalyticsEvent",
                (name, maxPerHour, maxPropertiesPerEvent),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SendAnalyticsEvent(
        &mut self,
        name: *mut crate::System::String,
        data: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendAnalyticsEvent", (name, data))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (updateType))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_currentTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentTimeForFixedUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object
            .invoke("get_currentTimeForFixedUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentTimeOffsetToRealtimeSinceStartup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object
            .invoke("get_currentTimeOffsetToRealtimeSinceStartup", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isInBatchMode(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInBatchMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isPlayerFocused(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isPlayerFocused", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onBeforeUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Action_1<
            crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Action_1<
            crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
        > = __cordl_object.invoke("get_onBeforeUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onDeviceDiscovered(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Action_2<i32, *mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Action_2<i32, *mut crate::System::String> = __cordl_object
            .invoke("get_onDeviceDiscovered", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onPlayerFocusChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Action_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Action_1<bool> = __cordl_object
            .invoke("get_onPlayerFocusChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onShouldRunUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
            bool,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
            bool,
        > = __cordl_object.invoke("get_onShouldRunUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onShutdown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Action> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Action = __cordl_object
            .invoke("get_onShutdown", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_onUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::LowLevel::InputUpdateDelegate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::LowLevel::InputUpdateDelegate = __cordl_object
            .invoke("get_onUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pollingFrequency(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pollingFrequency", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_runInBackground(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_runInBackground", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_screenOrientation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ScreenOrientation> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ScreenOrientation = __cordl_object
            .invoke("get_screenOrientation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_screenSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_screenSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_unscaledGameTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_unscaledGameTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_onBeforeUpdate(
        &mut self,
        value: *mut crate::System::Action_1<
            crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onBeforeUpdate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onDeviceDiscovered(
        &mut self,
        value: *mut crate::System::Action_2<i32, *mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onDeviceDiscovered", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onPlayerFocusChanged(
        &mut self,
        value: *mut crate::System::Action_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onPlayerFocusChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onShouldRunUpdate(
        &mut self,
        value: *mut crate::System::Func_2<
            crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
            bool,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onShouldRunUpdate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onShutdown(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onShutdown", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_onUpdate(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::LowLevel::InputUpdateDelegate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_onUpdate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_pollingFrequency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pollingFrequency", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_runInBackground(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_runInBackground", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+NativeInputRuntime")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::NativeInputRuntime {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
