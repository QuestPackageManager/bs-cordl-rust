#[cfg(feature = "UnityEngine+UIElements+FocusController")]
#[repr(C)]
#[derive(Debug)]
pub struct FocusController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _focusRing_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::IFocusRing,
    >,
    pub m_SelectedTextElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::TextElement,
    >,
    pub m_FocusedElements: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::FocusController_FocusedElement,
        >,
    >,
    pub m_LastFocusedElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Focusable,
    >,
    pub m_LastPendingFocusedElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Focusable,
    >,
    pub m_PendingFocusCount: i32,
    pub _imguiKeyboardControl_k__BackingField: i32,
}
#[cfg(feature = "UnityEngine+UIElements+FocusController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::FocusController =>
    "UnityEngine.UIElements"."FocusController"
);
#[cfg(feature = "UnityEngine+UIElements+FocusController")]
impl std::ops::Deref for crate::UnityEngine::UIElements::FocusController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FocusController")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::FocusController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FocusController")]
impl crate::UnityEngine::UIElements::FocusController {
    #[cfg(feature = "UnityEngine+UIElements+FocusController+FocusedElement")]
    pub type FocusedElement = crate::UnityEngine::UIElements::FocusController_FocusedElement;
    pub fn AboutToGrabFocus(
        &mut self,
        focusable: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
        willTakeFocusFrom: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        >,
        direction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        >,
        dispatchMode: crate::UnityEngine::UIElements::DispatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AboutToGrabFocus",
                (focusable, willTakeFocusFrom, direction, dispatchMode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AboutToReleaseFocus(
        &mut self,
        focusable: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
        willGiveFocusTo: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        >,
        direction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        >,
        dispatchMode: crate::UnityEngine::UIElements::DispatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AboutToReleaseFocus",
                (focusable, willGiveFocusTo, direction, dispatchMode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Blur(
        &mut self,
        focusable: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
        bIsFocusDelegated: bool,
        dispatchMode: crate::UnityEngine::UIElements::DispatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Blur", (focusable, bIsFocusDelegated, dispatchMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn BlurLastFocusedElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BlurLastFocusedElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoFocusChange(
        &mut self,
        f: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoFocusChange", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn FocusNextInDirection(
        &mut self,
        direction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        > = __cordl_object.invoke("FocusNextInDirection", (direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFocusableParentForPointerEvent(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
        effectiveTarget: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetFocusableParentForPointerEvent", (target, effectiveTarget))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLeafFocusedElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        > = __cordl_object.invoke("GetLeafFocusedElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRetargetedFocusedElement(
        &mut self,
        retargetAgainst: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        > = __cordl_object.invoke("GetRetargetedFocusedElement", (retargetAgainst))?;
        Ok(__cordl_ret.into())
    }
    pub fn GrabFocus(
        &mut self,
        focusable: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
        willTakeFocusFrom: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        >,
        direction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        >,
        bIsFocusDelegated: bool,
        dispatchMode: crate::UnityEngine::UIElements::DispatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GrabFocus",
                (
                    focusable,
                    willTakeFocusFrom,
                    direction,
                    bIsFocusDelegated,
                    dispatchMode,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFocused(
        &mut self,
        f: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsFocused", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLocalElement(
        &mut self,
        f: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLocalElement", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPendingFocus(
        &mut self,
        f: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPendingFocus", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        focusRing: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IFocusRing>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (focusRing))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessPendingFocusChange(
        &mut self,
        f: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPendingFocusChange", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReevaluateFocus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReevaluateFocus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseFocus(
        &mut self,
        focusable: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
        willGiveFocusTo: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        >,
        direction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        >,
        dispatchMode: crate::UnityEngine::UIElements::DispatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReleaseFocus",
                (focusable, willGiveFocusTo, direction, dispatchMode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFocusToLastFocusedElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFocusToLastFocusedElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchFocusOnEvent(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwitchFocusOnEvent", (e))?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchFocus_FocusChangeDirection__cordl_bool_DispatchMode1(
        &mut self,
        newFocusedElement: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        >,
        direction: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::FocusChangeDirection,
        >,
        bIsFocusDelegated: bool,
        dispatchMode: crate::UnityEngine::UIElements::DispatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SwitchFocus",
                (newFocusedElement, direction, bIsFocusDelegated, dispatchMode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SwitchFocus__cordl_bool_DispatchMode0(
        &mut self,
        newFocusedElement: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        >,
        bIsFocusDelegated: bool,
        dispatchMode: crate::UnityEngine::UIElements::DispatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SwitchFocus",
                (newFocusedElement, bIsFocusDelegated, dispatchMode),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncIMGUIFocus(
        &mut self,
        imguiKeyboardControlID: i32,
        imguiContainerHavingKeyboardControl: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        >,
        forceSwitch: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SyncIMGUIFocus",
                (
                    imguiKeyboardControlID,
                    imguiContainerHavingKeyboardControl,
                    forceSwitch,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        focusRing: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IFocusRing>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (focusRing))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_focusRing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IFocusRing>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IFocusRing,
        > = __cordl_object.invoke("get_focusRing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_focusedElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Focusable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Focusable,
        > = __cordl_object.invoke("get_focusedElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_imguiKeyboardControl(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_imguiKeyboardControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_imguiKeyboardControl(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_imguiKeyboardControl", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_selectedTextElement(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_selectedTextElement", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+FocusController")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::FocusController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+FocusController+FocusedElement")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FocusController_FocusedElement {
    pub m_SubTreeRoot: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_FocusedElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Focusable,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+FocusController+FocusedElement")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::FocusController_FocusedElement => "UnityEngine.UIElements"
    ."FocusController/FocusedElement"
);
#[cfg(feature = "UnityEngine+UIElements+FocusController+FocusedElement")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::FocusController_FocusedElement {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+FocusController+FocusedElement")]
impl crate::UnityEngine::UIElements::FocusController_FocusedElement {}
