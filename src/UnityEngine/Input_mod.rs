#[cfg(feature = "UnityEngine+Input")]
#[repr(C)]
#[derive(Debug)]
pub struct Input {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Input")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Input => "UnityEngine"."Input"
);
#[cfg(feature = "UnityEngine+Input")]
impl std::ops::Deref for crate::UnityEngine::Input {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Input")]
impl std::ops::DerefMut for crate::UnityEngine::Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Input")]
impl crate::UnityEngine::Input {
    pub fn CheckDisabled() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckDisabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearLastPenContactEvent() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearLastPenContactEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAxis(
        axisName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAxis", (axisName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAxisRaw(
        axisName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAxisRaw", (axisName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButton(
        buttonName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetButton", (buttonName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButtonDown(
        buttonName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetButtonDown", (buttonName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKey(
        key: crate::UnityEngine::KeyCode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyDown(
        key: crate::UnityEngine::KeyCode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyDown", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyDownInt(
        key: crate::UnityEngine::KeyCode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyDownInt", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyInt(
        key: crate::UnityEngine::KeyCode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyInt", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyUp(
        key: crate::UnityEngine::KeyCode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyUp", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeyUpInt(
        key: crate::UnityEngine::KeyCode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeyUpInt", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastPenContactEvent() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::PenData,
    > {
        let __cordl_ret: crate::UnityEngine::PenData = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLastPenContactEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastPenContactEvent_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::PenData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLastPenContactEvent_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMouseButton(button: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMouseButton", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMouseButtonDown(button: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMouseButtonDown", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMouseButtonUp(button: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMouseButtonUp", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTouch(
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Touch> {
        let __cordl_ret: crate::UnityEngine::Touch = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTouch", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTouch_Injected(
        index: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Touch>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTouch_Injected", (index, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_anyKey() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_anyKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_compositionCursorPos() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Vector2,
    > {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_compositionCursorPos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_compositionCursorPos_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_compositionCursorPos_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_compositionString() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_compositionString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_imeCompositionMode() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::IMECompositionMode,
    > {
        let __cordl_ret: crate::UnityEngine::IMECompositionMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_imeCompositionMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mousePosition() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Vector3,
    > {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_mousePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mousePosition_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_mousePosition_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mousePresent() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_mousePresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mouseScrollDelta() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Vector2,
    > {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_mouseScrollDelta", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mouseScrollDelta_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_mouseScrollDelta_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_touchCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_touchCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_touchSupported() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_touchSupported", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_compositionCursorPos(
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_compositionCursorPos", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_compositionCursorPos_Injected(
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_compositionCursorPos_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_imeCompositionMode(
        value: crate::UnityEngine::IMECompositionMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_imeCompositionMode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Input")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Input {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
