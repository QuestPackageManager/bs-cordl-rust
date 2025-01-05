#[cfg(feature = "UnityEngine+Cursor")]
#[repr(C)]
#[derive(Debug)]
pub struct Cursor {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Cursor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Cursor => "UnityEngine"."Cursor"
);
#[cfg(feature = "UnityEngine+Cursor")]
impl std::ops::Deref for crate::UnityEngine::Cursor {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Cursor")]
impl std::ops::DerefMut for crate::UnityEngine::Cursor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Cursor")]
impl crate::UnityEngine::Cursor {
    pub fn SetCursor(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        hotspot: crate::UnityEngine::Vector2,
        cursorMode: crate::UnityEngine::CursorMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetCursor", (texture, hotspot, cursorMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCursor_Injected(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        hotspot: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        cursorMode: crate::UnityEngine::CursorMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetCursor_Injected", (texture, hotspot, cursorMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lockState() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::CursorLockMode,
    > {
        let __cordl_ret: crate::UnityEngine::CursorLockMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_lockState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lockState(
        value: crate::UnityEngine::CursorLockMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_lockState", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_visible(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_visible", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Cursor")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Cursor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
