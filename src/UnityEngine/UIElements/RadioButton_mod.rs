#[cfg(feature = "UnityEngine+UIElements+RadioButton")]
#[repr(C)]
#[derive(Debug)]
pub struct RadioButton {
    __cordl_parent: crate::UnityEngine::UIElements::BaseBoolField,
    pub m_CheckmarkBackground: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RadioButton =>
    "UnityEngine.UIElements"."RadioButton"
);
#[cfg(feature = "UnityEngine+UIElements+RadioButton")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RadioButton {
    type Target = crate::UnityEngine::UIElements::BaseBoolField;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RadioButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton")]
impl crate::UnityEngine::UIElements::RadioButton {
    #[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::RadioButton_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::RadioButton_UxmlTraits;
    pub fn InitLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitLabel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString1(
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label))?;
        Ok(__cordl_object.into())
    }
    pub fn OnOptionAttachToPanel(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::AttachToPanelEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnOptionAttachToPanel", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnOptionDetachFromPanel(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DetachFromPanelEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnOptionDetachFromPanel", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValueWithoutNotify(
        &mut self,
        newValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValueWithoutNotify", (newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToggleValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ToggleValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IGroupBoxOption_SetSelected(
        &mut self,
        selected: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IGroupBoxOption.SetSelected", (selected))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCheckmark(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCheckmark", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMixedValueContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMixedValueContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString1(
        &mut self,
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (label))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_value(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_value", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::RadioButton {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton")]
impl AsRef<crate::UnityEngine::UIElements::IGroupBoxOption>
for crate::UnityEngine::UIElements::RadioButton {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IGroupBoxOption {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton")]
impl AsMut<crate::UnityEngine::UIElements::IGroupBoxOption>
for crate::UnityEngine::UIElements::RadioButton {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IGroupBoxOption {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct RadioButton_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::RadioButton,
        *mut crate::UnityEngine::UIElements::RadioButton_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RadioButton_UxmlFactory
    => "UnityEngine.UIElements"."RadioButton/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RadioButton_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::RadioButton,
        *mut crate::UnityEngine::UIElements::RadioButton_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RadioButton_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlFactory")]
impl crate::UnityEngine::UIElements::RadioButton_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RadioButton_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct RadioButton_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseFieldTraits_2<
        bool,
        *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    >,
    pub m_Text: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RadioButton_UxmlTraits
    => "UnityEngine.UIElements"."RadioButton/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RadioButton_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseFieldTraits_2<
        bool,
        *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RadioButton_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlTraits")]
impl crate::UnityEngine::UIElements::RadioButton_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+RadioButton+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RadioButton_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
