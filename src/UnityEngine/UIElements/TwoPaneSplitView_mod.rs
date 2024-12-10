#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView")]
#[repr(C)]
#[derive(Debug)]
pub struct TwoPaneSplitView {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement,
    pub m_LeftPane: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_RightPane: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_FixedPane: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_FlexedPane: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_FixedPaneDimension: f32,
    pub m_DragLine: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_DragLineAnchor: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_CollapseMode: bool,
    pub m_Content: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_Orientation: crate::UnityEngine::UIElements::TwoPaneSplitViewOrientation,
    pub m_FixedPaneIndex: i32,
    pub m_FixedPaneInitialDimension: f32,
    pub m_Resizer: *mut crate::UnityEngine::UIElements::TwoPaneSplitViewResizer,
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TwoPaneSplitView =>
    "UnityEngine.UIElements"."TwoPaneSplitView"
);
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TwoPaneSplitView {
    type Target = crate::UnityEngine::UIElements::VisualElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TwoPaneSplitView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView")]
impl crate::UnityEngine::UIElements::TwoPaneSplitView {
    #[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlTraits;
    pub fn Init(
        &mut self,
        fixedPaneIndex: i32,
        fixedPaneInitialDimension: f32,
        orientation: crate::UnityEngine::UIElements::TwoPaneSplitViewOrientation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (fixedPaneIndex, fixedPaneInitialDimension, orientation))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnPostDisplaySetup(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPostDisplaySetup", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSizeChange(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::GeometryChangedEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSizeChange", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnViewDataReady(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnViewDataReady", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PostDisplaySetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostDisplaySetup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDragLineOffset(
        &mut self,
        offset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDragLineOffset", (offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFixedPaneDimension(
        &mut self,
        dimension: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFixedPaneDimension", (dimension))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLayout(
        &mut self,
        updateFixedPane: bool,
        updateDragLine: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLayout", (updateFixedPane, updateDragLine))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contentContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_contentContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedPane(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_fixedPane", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedPaneDimension(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fixedPaneDimension", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedPaneIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_fixedPaneIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexedPane(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_flexedPane", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_fixedPaneDimension(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fixedPaneDimension", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TwoPaneSplitView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct TwoPaneSplitView_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::TwoPaneSplitView,
        *mut crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::TwoPaneSplitView_UxmlFactory => "UnityEngine.UIElements"
    ."TwoPaneSplitView/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::TwoPaneSplitView,
        *mut crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlFactory")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlFactory")]
impl crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlFactory {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct TwoPaneSplitView_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement_UxmlTraits,
    pub m_FixedPaneIndex: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_FixedPaneInitialDimension: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_Orientation: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::TwoPaneSplitViewOrientation,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::TwoPaneSplitView_UxmlTraits => "UnityEngine.UIElements"
    ."TwoPaneSplitView/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::VisualElement_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlTraits")]
impl crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlTraits {
    pub fn Init(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TwoPaneSplitView+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TwoPaneSplitView_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
