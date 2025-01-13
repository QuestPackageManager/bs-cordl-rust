#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
#[repr(C)]
#[derive(Debug)]
pub struct ContentSizeFitter {
    __cordl_parent: crate::UnityEngine::EventSystems::UIBehaviour,
    pub m_HorizontalFit: crate::UnityEngine::UI::ContentSizeFitter_FitMode,
    pub m_VerticalFit: crate::UnityEngine::UI::ContentSizeFitter_FitMode,
    pub m_Rect: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub m_Tracker: crate::UnityEngine::DrivenRectTransformTracker,
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UI::ContentSizeFitter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "ContentSizeFitter";
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
    pub fn HandleSelfFittingAlongAxis(
        &mut self,
        axis: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSelfFittingAlongAxis", (axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRectTransformDimensionsChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRectTransformDimensionsChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLayoutHorizontal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayoutHorizontal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLayoutVertical(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLayoutVertical", ())?;
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
        Ok(__cordl_ret.into())
    }
    pub fn get_rectTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_rectTransform", ())?;
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
impl AsRef<crate::UnityEngine::UI::ILayoutController>
for crate::UnityEngine::UI::ContentSizeFitter {
    fn as_ref(&self) -> &crate::UnityEngine::UI::ILayoutController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
impl AsMut<crate::UnityEngine::UI::ILayoutController>
for crate::UnityEngine::UI::ContentSizeFitter {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::ILayoutController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
impl AsRef<crate::UnityEngine::UI::ILayoutSelfController>
for crate::UnityEngine::UI::ContentSizeFitter {
    fn as_ref(&self) -> &crate::UnityEngine::UI::ILayoutSelfController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter")]
impl AsMut<crate::UnityEngine::UI::ILayoutSelfController>
for crate::UnityEngine::UI::ContentSizeFitter {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UI::ILayoutSelfController {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter+FitMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContentSizeFitter_FitMode {
    #[default]
    MinSize = 1i32,
    PreferredSize = 2i32,
    Unconstrained = 0i32,
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter+FitMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::ContentSizeFitter_FitMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "ContentSizeFitter/FitMode";
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
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter+FitMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UI::ContentSizeFitter_FitMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter+FitMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UI::ContentSizeFitter_FitMode {
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
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter+FitMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UI::ContentSizeFitter_FitMode {
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
#[cfg(feature = "UnityEngine+UI+ContentSizeFitter+FitMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UI::ContentSizeFitter_FitMode {
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
