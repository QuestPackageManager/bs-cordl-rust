#[cfg(feature = "HMUI+ScrollView")]
#[repr(C)]
#[derive(Debug)]
pub struct ScrollView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _viewport: *mut crate::UnityEngine::RectTransform,
    pub _contentRectTransform: *mut crate::UnityEngine::RectTransform,
    pub _scrollViewDirection: crate::HMUI::ScrollView_ScrollViewDirection,
    pub _pageUpButton: *mut crate::UnityEngine::UI::Button,
    pub _pageDownButton: *mut crate::UnityEngine::UI::Button,
    pub _verticalScrollIndicator: *mut crate::HMUI::VerticalScrollIndicator,
    pub _smooth: f32,
    pub _joystickScrollSpeed: f32,
    pub _joystickQuickSnapMaxTime: f32,
    pub _scrollType: crate::HMUI::ScrollView_ScrollType,
    pub _fixedCellSize: f32,
    pub _scrollItemRelativeThresholdPosition: f32,
    pub _pageStepNormalizedSize: f32,
    pub _scrollingLastFrame: bool,
    pub _isHoveredByPointer: bool,
    pub _shouldAnimate: bool,
    pub _platformHelper: *mut crate::GlobalNamespace::IVRPlatformHelper,
    pub scrollPositionChangedEvent: *mut crate::System::Action_1<f32>,
    pub _buttonBinder: *mut crate::HMUI::ButtonBinder,
    pub _destinationPos: f32,
    pub _scrollFocusPositions: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _eventSystemListener: *mut crate::HMUI::EventSystemListener,
    pub _lastJoystickScrollDirection: crate::HMUI::ScrollView_ScrollDirection,
    pub _joystickScrollStartTime: f32,
}
#[cfg(feature = "HMUI+ScrollView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ScrollView => "HMUI"."ScrollView"
);
#[cfg(feature = "HMUI+ScrollView")]
impl std::ops::Deref for crate::HMUI::ScrollView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScrollView")]
impl std::ops::DerefMut for crate::HMUI::ScrollView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScrollView")]
impl crate::HMUI::ScrollView {
    pub const kThumbstickThreshold: f32 = 0.01f32;
    #[cfg(feature = "HMUI+ScrollView+ScrollDirection")]
    pub type ScrollDirection = crate::HMUI::ScrollView_ScrollDirection;
    #[cfg(feature = "HMUI+ScrollView+ScrollType")]
    pub type ScrollType = crate::HMUI::ScrollView_ScrollType;
    #[cfg(feature = "HMUI+ScrollView+ScrollViewDirection")]
    pub type ScrollViewDirection = crate::HMUI::ScrollView_ScrollViewDirection;
    #[cfg(feature = "HMUI+ScrollView+__c")]
    pub type __c = crate::HMUI::ScrollView___c;
    #[cfg(feature = "HMUI+ScrollView+__c__DisplayClass55_0")]
    pub type __c__DisplayClass55_0 = crate::HMUI::ScrollView___c__DisplayClass55_0;
    #[cfg(feature = "HMUI+ScrollView+__c__DisplayClass56_0")]
    pub type __c__DisplayClass56_0 = crate::HMUI::ScrollView___c__DisplayClass56_0;
    #[cfg(feature = "HMUI+ScrollView+__c__DisplayClass61_0")]
    pub type __c__DisplayClass61_0 = crate::HMUI::ScrollView___c__DisplayClass61_0;
    #[cfg(feature = "HMUI+ScrollView+__c__DisplayClass61_1")]
    pub type __c__DisplayClass61_1 = crate::HMUI::ScrollView___c__DisplayClass61_1;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckScrollInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckScrollInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleJoystickWasCenteredThisFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleJoystickWasCenteredThisFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleJoystickWasNotCenteredThisFrame(
        &mut self,
        deltaPos: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleJoystickWasNotCenteredThisFrame", (deltaPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePointerDidEnter(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePointerDidEnter", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePointerDidExit(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePointerDidExit", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PageDownButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PageDownButtonPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PageUpButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PageUpButtonPressed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshButtons(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshButtons", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveScrollDirection(
        &mut self,
        deltaPos: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::HMUI::ScrollView_ScrollDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::HMUI::ScrollView_ScrollDirection = __cordl_object
            .invoke("ResolveScrollDirection", (deltaPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollTo(
        &mut self,
        destinationPos: f32,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollTo", (destinationPos, animated))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollToEnd(
        &mut self,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollToEnd", (animated))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollToWorldPosition(
        &mut self,
        worldPosition: crate::UnityEngine::Vector3,
        pageRelativePosition: f32,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ScrollToWorldPosition",
                (worldPosition, pageRelativePosition, animated),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollToWorldPositionIfOutsideArea(
        &mut self,
        worldPosition: crate::UnityEngine::Vector3,
        pageRelativePosition: f32,
        relativeBoundaryStart: f32,
        relativeBoundaryEnd: f32,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ScrollToWorldPositionIfOutsideArea",
                (
                    worldPosition,
                    pageRelativePosition,
                    relativeBoundaryStart,
                    relativeBoundaryEnd,
                    animated,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetContentSize(
        &mut self,
        contentSize: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContentSize", (contentSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDestinationPos(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDestinationPos", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateContentSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateContentSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateVerticalScrollIndicator(
        &mut self,
        posY: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVerticalScrollIndicator", (posY))?;
        Ok(__cordl_ret.into())
    }
    pub fn WorldPositionToScrollViewPosition(
        &mut self,
        worldPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("WorldPositionToScrollViewPosition", (worldPosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_b__42_0(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::HMUI::ItemForFocussedScrolling>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("<Awake>b__42_0", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn _Awake_b__42_2(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::HMUI::ItemForFocussedScrolling>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("<Awake>b__42_2", (item))?;
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
    pub fn add_scrollPositionChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_scrollPositionChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contentSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_contentSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contentTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_contentTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_position(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_position", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollPageSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scrollPageSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollableSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scrollableSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_viewportTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_viewportTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_scrollPositionChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_scrollPositionChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+ScrollView")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ScrollView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+ScrollView+ScrollDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollView_ScrollDirection {
    Down = 2i32,
    Left = 3i32,
    None = 0i32,
    Right = 4i32,
    Up = 1i32,
}
#[cfg(feature = "HMUI+ScrollView+ScrollDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ScrollView_ScrollDirection => "HMUI"
    ."ScrollView/ScrollDirection"
);
#[cfg(feature = "HMUI+ScrollView+ScrollType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollView_ScrollType {
    FixedCellSize = 1i32,
    FocusItems = 2i32,
    PageSize = 0i32,
}
#[cfg(feature = "HMUI+ScrollView+ScrollType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ScrollView_ScrollType => "HMUI"
    ."ScrollView/ScrollType"
);
#[cfg(feature = "HMUI+ScrollView+ScrollViewDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollView_ScrollViewDirection {
    Horizontal = 1i32,
    Vertical = 0i32,
}
#[cfg(feature = "HMUI+ScrollView+ScrollViewDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ScrollView_ScrollViewDirection => "HMUI"
    ."ScrollView/ScrollViewDirection"
);
