#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtilityNative")]
#[repr(C)]
#[derive(Debug)]
pub struct UIElementsRuntimeUtilityNative {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtilityNative")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIElementsRuntimeUtilityNative => "UnityEngine.UIElements"
    ."UIElementsRuntimeUtilityNative"
);
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtilityNative")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIElementsRuntimeUtilityNative {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtilityNative")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIElementsRuntimeUtilityNative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtilityNative")]
impl crate::UnityEngine::UIElements::UIElementsRuntimeUtilityNative {
    pub fn RegisterPlayerloopCallback() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterPlayerloopCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RepaintOffscreenPanels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RepaintOffscreenPanels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RepaintOverlayPanels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RepaintOverlayPanels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterPlayerloopCallback() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnregisterPlayerloopCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRuntimePanels() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UpdateRuntimePanels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VisualElementCreation() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VisualElementCreation", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIElementsRuntimeUtilityNative")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIElementsRuntimeUtilityNative {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
