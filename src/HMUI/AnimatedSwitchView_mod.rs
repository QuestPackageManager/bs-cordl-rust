#[cfg(feature = "HMUI+AnimatedSwitchView")]
#[repr(C)]
#[derive(Debug)]
pub struct AnimatedSwitchView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _knobRectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub _backgroundImage: quest_hook::libil2cpp::Gc<crate::HMUI::ImageView>,
    pub _knobImage: quest_hook::libil2cpp::Gc<crate::HMUI::ImageView>,
    pub _onText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _offText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _switchAnimationSmooth: f32,
    pub _disableAnimationDuration: f32,
    pub _highlightAnimationDuration: f32,
    pub _horizontalStretchAmount: f32,
    pub _verticalStretchAmount: f32,
    pub _onColors: quest_hook::libil2cpp::Gc<crate::HMUI::AnimatedSwitchView_ColorBlock>,
    pub _offColors: quest_hook::libil2cpp::Gc<
        crate::HMUI::AnimatedSwitchView_ColorBlock,
    >,
    pub _onHighlightedColors: quest_hook::libil2cpp::Gc<
        crate::HMUI::AnimatedSwitchView_ColorBlock,
    >,
    pub _offHighlightedColors: quest_hook::libil2cpp::Gc<
        crate::HMUI::AnimatedSwitchView_ColorBlock,
    >,
    pub _disabledColors: quest_hook::libil2cpp::Gc<
        crate::HMUI::AnimatedSwitchView_ColorBlock,
    >,
    pub _animationState: crate::HMUI::AnimatedSwitchView_AnimationState,
    pub _switchAmount: f32,
    pub _highlightAmount: f32,
    pub _disabledAmount: f32,
    pub _originalKnobWidth: f32,
    pub _originalKnobHeight: f32,
    pub _toggle: quest_hook::libil2cpp::Gc<crate::HMUI::ToggleWithCallbacks>,
}
#[cfg(feature = "HMUI+AnimatedSwitchView")]
unsafe impl quest_hook::libil2cpp::Type for crate::HMUI::AnimatedSwitchView {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "AnimatedSwitchView";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimatedSwitchView_AnimationState {
    #[default]
    DisablingOff = 32i32,
    DisablingOn = 16i32,
    HighlightingOff = 8i32,
    HighlightingOn = 4i32,
    Idle = 0i32,
    SwitchingOff = 2i32,
    SwitchingOn = 1i32,
}
#[cfg(feature = "HMUI+AnimatedSwitchView+AnimationState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HMUI::AnimatedSwitchView_AnimationState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "AnimationState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView+AnimationState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HMUI::AnimatedSwitchView_AnimationState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView+AnimationState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HMUI::AnimatedSwitchView_AnimationState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView+AnimationState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HMUI::AnimatedSwitchView_AnimationState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "HMUI+AnimatedSwitchView+AnimationState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::HMUI::AnimatedSwitchView_AnimationState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
unsafe impl quest_hook::libil2cpp::Type for crate::HMUI::AnimatedSwitchView_ColorBlock {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "ColorBlock";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
unsafe impl quest_hook::libil2cpp::Type
for crate::HMUI::AnimatedSwitchView_GetColorDelegate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HMUI";
    const CLASS_NAME: &'static str = "GetColorDelegate";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
