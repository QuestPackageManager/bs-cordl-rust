#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Finger")]
#[repr(C)]
#[derive(Debug)]
pub struct Finger {
    __cordl_parent: crate::System::Object,
    pub _screen_k__BackingField: *mut crate::UnityEngine::InputSystem::Touchscreen,
    pub _index_k__BackingField: i32,
    pub m_StateHistory: *mut crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_1<
        crate::UnityEngine::InputSystem::LowLevel::TouchState,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Finger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::EnhancedTouch::Finger
    => "UnityEngine.InputSystem.EnhancedTouch"."Finger"
);
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Finger")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::EnhancedTouch::Finger {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Finger")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::EnhancedTouch::Finger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Finger")]
impl crate::UnityEngine::InputSystem::EnhancedTouch::Finger {
    pub fn FindTouch(
        &mut self,
        uniqueId: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::EnhancedTouch::Touch = __cordl_object
            .invoke("FindTouch", (uniqueId))?;
        Ok(__cordl_ret)
    }
    pub fn GetTouchHistory(
        &mut self,
        touch: crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory = __cordl_object
            .invoke("GetTouchHistory", (touch))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        screen: *mut crate::UnityEngine::InputSystem::Touchscreen,
        index: i32,
        updateMask: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (screen, index, updateMask))?;
        Ok(__cordl_object)
    }
    pub fn OnTouchRecorded(
        &mut self,
        record: crate::UnityEngine::InputSystem::LowLevel::InputStateHistory_Record,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTouchRecorded", (record))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        screen: *mut crate::UnityEngine::InputSystem::Touchscreen,
        index: i32,
        updateMask: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (screen, index, updateMask))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentTouch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::EnhancedTouch::Touch = __cordl_object
            .invoke("get_currentTouch", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_index(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_index", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActive", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lastTouch(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::EnhancedTouch::Touch,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::EnhancedTouch::Touch = __cordl_object
            .invoke("get_lastTouch", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_screen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::Touchscreen,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::Touchscreen = __cordl_object
            .invoke("get_screen", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_screenPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_screenPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_touchHistory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::EnhancedTouch::TouchHistory = __cordl_object
            .invoke("get_touchHistory", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+EnhancedTouch+Finger")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::EnhancedTouch::Finger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
