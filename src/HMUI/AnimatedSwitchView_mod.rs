#[cfg(feature = "HMUI+AnimatedSwitchView")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimatedSwitchView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _knobRectTransform: *mut crate::UnityEngine::RectTransform,
    pub _backgroundImage: *mut crate::HMUI::ImageView,
    pub _knobImage: *mut crate::HMUI::ImageView,
    pub _onText: *mut crate::TMPro::TextMeshProUGUI,
    pub _offText: *mut crate::TMPro::TextMeshProUGUI,
    pub _switchAnimationSmooth: f32,
    pub _disableAnimationDuration: f32,
    pub _highlightAnimationDuration: f32,
    pub _horizontalStretchAmount: f32,
    pub _verticalStretchAmount: f32,
    pub _onColors: *mut crate::HMUI::AnimatedSwitchView_ColorBlock,
    pub _offColors: *mut crate::HMUI::AnimatedSwitchView_ColorBlock,
    pub _onHighlightedColors: *mut crate::HMUI::AnimatedSwitchView_ColorBlock,
    pub _offHighlightedColors: *mut crate::HMUI::AnimatedSwitchView_ColorBlock,
    pub _disabledColors: *mut crate::HMUI::AnimatedSwitchView_ColorBlock,
    pub _animationState: crate::HMUI::AnimatedSwitchView_AnimationState,
    pub _switchAmount: f32,
    pub _highlightAmount: f32,
    pub _disabledAmount: f32,
    pub _originalKnobWidth: f32,
    pub _originalKnobHeight: f32,
    pub _toggle: *mut crate::HMUI::ToggleWithCallbacks,
}
#[cfg(feature = "HMUI+AnimatedSwitchView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::AnimatedSwitchView => "HMUI"
    ."AnimatedSwitchView"
);
#[cfg(feature = "HMUI+AnimatedSwitchView")]
impl std::ops::Deref for crate::HMUI::AnimatedSwitchView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView")]
impl std::ops::DerefMut for crate::HMUI::AnimatedSwitchView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView")]
impl crate::HMUI::AnimatedSwitchView {
    #[cfg(feature = "HMUI+AnimatedSwitchView+AnimationState")]
    pub type AnimationState = crate::HMUI::AnimatedSwitchView_AnimationState;
    #[cfg(feature = "HMUI+AnimatedSwitchView+ColorBlock")]
    pub type ColorBlock = crate::HMUI::AnimatedSwitchView_ColorBlock;
    #[cfg(feature = "HMUI+AnimatedSwitchView+GetColorDelegate")]
    pub type GetColorDelegate = crate::HMUI::AnimatedSwitchView_GetColorDelegate;
    #[cfg(feature = "HMUI+AnimatedSwitchView+__c")]
    pub type __c = crate::HMUI::AnimatedSwitchView___c;
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
    pub fn HandleOnValueChanged(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleOnValueChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleStateDidChange(
        &mut self,
        selectionState: crate::HMUI::ToggleWithCallbacks_SelectionState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleStateDidChange", (selectionState))?;
        Ok(__cordl_ret.into())
    }
    pub fn LerpColor(
        &mut self,
        switchAmount: f32,
        highlightAmount: f32,
        disabledAmount: f32,
        getColorDelegate: quest_hook::libil2cpp::Gc<
            crate::HMUI::AnimatedSwitchView_GetColorDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke(
                "LerpColor",
                (switchAmount, highlightAmount, disabledAmount, getColorDelegate),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LerpColors(
        &mut self,
        switchAmount: f32,
        highlightAmount: f32,
        disabledAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LerpColors", (switchAmount, highlightAmount, disabledAmount))?;
        Ok(__cordl_ret.into())
    }
    pub fn LerpPosition(
        &mut self,
        switchAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LerpPosition", (switchAmount))?;
        Ok(__cordl_ret.into())
    }
    pub fn LerpStretch(
        &mut self,
        switchAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LerpStretch", (switchAmount))?;
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
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "HMUI+AnimatedSwitchView")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::AnimatedSwitchView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView+AnimationState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimatedSwitchView_AnimationState {
    DisablingOff = 32i32,
    DisablingOn = 16i32,
    HighlightingOff = 8i32,
    HighlightingOn = 4i32,
    Idle = 0i32,
    SwitchingOff = 2i32,
    SwitchingOn = 1i32,
}
#[cfg(feature = "HMUI+AnimatedSwitchView+AnimationState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::AnimatedSwitchView_AnimationState =>
    "HMUI"."AnimatedSwitchView/AnimationState"
);
#[cfg(feature = "HMUI+AnimatedSwitchView+ColorBlock")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimatedSwitchView_ColorBlock {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub knobColor: crate::UnityEngine::Color,
    pub knobColor0: crate::UnityEngine::Color,
    pub knobColor1: crate::UnityEngine::Color,
    pub backgroundColor: crate::UnityEngine::Color,
    pub backgroundColor0: crate::UnityEngine::Color,
    pub backgroundColor1: crate::UnityEngine::Color,
}
#[cfg(feature = "HMUI+AnimatedSwitchView+ColorBlock")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::AnimatedSwitchView_ColorBlock => "HMUI"
    ."AnimatedSwitchView/ColorBlock"
);
#[cfg(feature = "HMUI+AnimatedSwitchView+ColorBlock")]
impl std::ops::Deref for crate::HMUI::AnimatedSwitchView_ColorBlock {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView+ColorBlock")]
impl std::ops::DerefMut for crate::HMUI::AnimatedSwitchView_ColorBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView+ColorBlock")]
impl crate::HMUI::AnimatedSwitchView_ColorBlock {
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
#[cfg(feature = "HMUI+AnimatedSwitchView+ColorBlock")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::AnimatedSwitchView_ColorBlock {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView+GetColorDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimatedSwitchView_GetColorDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HMUI+AnimatedSwitchView+GetColorDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::AnimatedSwitchView_GetColorDelegate =>
    "HMUI"."AnimatedSwitchView/GetColorDelegate"
);
#[cfg(feature = "HMUI+AnimatedSwitchView+GetColorDelegate")]
impl std::ops::Deref for crate::HMUI::AnimatedSwitchView_GetColorDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView+GetColorDelegate")]
impl std::ops::DerefMut for crate::HMUI::AnimatedSwitchView_GetColorDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView+GetColorDelegate")]
impl crate::HMUI::AnimatedSwitchView_GetColorDelegate {
    pub fn BeginInvoke(
        &mut self,
        colorBlock: quest_hook::libil2cpp::Gc<
            crate::HMUI::AnimatedSwitchView_ColorBlock,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (colorBlock, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        colorBlock: quest_hook::libil2cpp::Gc<crate::HMUI::AnimatedSwitchView_ColorBlock>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("Invoke", (colorBlock))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView+GetColorDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HMUI::AnimatedSwitchView_GetColorDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
