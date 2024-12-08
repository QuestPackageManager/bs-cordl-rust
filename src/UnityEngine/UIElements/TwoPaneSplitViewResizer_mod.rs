#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitViewResizer")]
#[repr(C)]
#[derive(Debug)]
pub struct TwoPaneSplitViewResizer {
    __cordl_parent: crate::UnityEngine::UIElements::PointerManipulator,
    pub m_Start: crate::UnityEngine::Vector3,
    pub m_Active: bool,
    pub m_SplitView: *mut crate::UnityEngine::UIElements::TwoPaneSplitView,
    pub m_Direction: i32,
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitViewResizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TwoPaneSplitViewResizer
    => "UnityEngine.UIElements"."TwoPaneSplitViewResizer"
);
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitViewResizer")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TwoPaneSplitViewResizer {
    type Target = crate::UnityEngine::UIElements::PointerManipulator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitViewResizer")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TwoPaneSplitViewResizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitViewResizer")]
impl crate::UnityEngine::UIElements::TwoPaneSplitViewResizer {
    pub fn get_flexedPaneMinDimension(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexedPaneMinDimension", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        splitView: *mut crate::UnityEngine::UIElements::TwoPaneSplitView,
        dir: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (splitView, dir))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallbacksOnTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallbacksOnTarget", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerMove(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::PointerMoveEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerMove", (e))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerUp(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::PointerUpEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (e))?;
        Ok(__cordl_ret)
    }
    pub fn get_fixedPane(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_fixedPane", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fixedPaneMargins(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fixedPaneMargins", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flexedPane(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_flexedPane", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_flexedPaneMargin(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_flexedPaneMargin", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterCallbacksFromTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCallbacksFromTarget", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerDown(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::PointerDownEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (e))?;
        Ok(__cordl_ret)
    }
    pub fn get_fixedPaneMinDimension(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fixedPaneMinDimension", ())?;
        Ok(__cordl_ret)
    }
    pub fn ApplyDelta(
        &mut self,
        delta: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyDelta", (delta))?;
        Ok(__cordl_ret)
    }
    pub fn get_orientation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TwoPaneSplitViewOrientation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::TwoPaneSplitViewOrientation = __cordl_object
            .invoke("get_orientation", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        splitView: *mut crate::UnityEngine::UIElements::TwoPaneSplitView,
        dir: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (splitView, dir))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitViewResizer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TwoPaneSplitViewResizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
