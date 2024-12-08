#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
#[repr(C)]
#[derive(Debug)]
pub struct ContentSizeFitter {
    __cordl_parent: crate::UnityEngine::EventSystems::UIBehaviour,
    pub m_HorizontalFit: crate::UnityEngine::UI::ContentSizeFitter_FitMode,
    pub m_VerticalFit: crate::UnityEngine::UI::ContentSizeFitter_FitMode,
    pub m_Rect: *mut crate::UnityEngine::RectTransform,
    pub m_Tracker: crate::UnityEngine::DrivenRectTransformTracker,
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::ContentSizeFitter =>
    "UnityEngine.UI"."ContentSizeFitter"
);
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
impl std::ops::Deref for crate::UnityEngine::UI::ContentSizeFitter {
    type Target = crate::UnityEngine::EventSystems::UIBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
impl std::ops::DerefMut for crate::UnityEngine::UI::ContentSizeFitter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
impl crate::UnityEngine::UI::ContentSizeFitter {
    #[cfg(feature = "UnityEngine+UI+ContentSizeFitter+FitMode")]
    pub type FitMode = crate::UnityEngine::UI::ContentSizeFitter_FitMode;
    pub fn get_verticalFit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UI::ContentSizeFitter_FitMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::ContentSizeFitter_FitMode = __cordl_object
            .invoke("get_verticalFit", ())?;
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
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLayoutHorizontal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayoutHorizontal", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetLayoutVertical(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayoutVertical", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_verticalFit(
        &mut self,
        value: crate::UnityEngine::UI::ContentSizeFitter_FitMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalFit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnRectTransformDimensionsChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRectTransformDimensionsChange", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_horizontalFit(
        &mut self,
        value: crate::UnityEngine::UI::ContentSizeFitter_FitMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalFit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleSelfFittingAlongAxis(
        &mut self,
        axis: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSelfFittingAlongAxis", (axis))?;
        Ok(__cordl_ret)
    }
    pub fn get_rectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RectTransform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RectTransform = __cordl_object
            .invoke("get_rectTransform", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_horizontalFit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UI::ContentSizeFitter_FitMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UI::ContentSizeFitter_FitMode = __cordl_object
            .invoke("get_horizontalFit", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::ContentSizeFitter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter+FitMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentSizeFitter_FitMode {
    MinSize = 1i32,
    PreferredSize = 2i32,
    Unconstrained = 0i32,
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter+FitMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::ContentSizeFitter_FitMode =>
    "UnityEngine.UI"."ContentSizeFitter/FitMode"
);
