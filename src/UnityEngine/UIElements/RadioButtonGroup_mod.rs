#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct RadioButtonGroup {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1<i32>,
    pub m_RadioButtons: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::RadioButton,
        >,
    >,
    pub m_RadioButtonValueChangedCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::EventCallback_1<
            *mut crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
        >,
    >,
    pub m_RadioButtonContainer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RadioButtonGroup =>
    "UnityEngine.UIElements"."RadioButtonGroup"
);
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RadioButtonGroup {
    type Target = crate::UnityEngine::UIElements::BaseField_1<i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RadioButtonGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup")]
impl crate::UnityEngine::UIElements::RadioButtonGroup {
    #[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::RadioButtonGroup_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::RadioButtonGroup_UxmlTraits;
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_List_1_1(
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        radioButtonChoices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label, radioButtonChoices))?;
        Ok(__cordl_object.into())
    }
    pub fn RadioButtonValueChangedCallback(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ChangeEvent_1<bool>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RadioButtonValueChangedCallback", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValueWithoutNotify(
        &mut self,
        newValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValueWithoutNotify", (newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IGroupBox_OnOptionAdded(
        &mut self,
        option: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGroupBoxOption,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IGroupBox.OnOptionAdded", (option))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IGroupBox_OnOptionRemoved(
        &mut self,
        option: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGroupBoxOption,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IGroupBox.OnOptionRemoved", (option))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateRadioButtons(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateRadioButtons", ())?;
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
    pub fn _ctor_Il2CppString_List_1_1(
        &mut self,
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        radioButtonChoices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (label, radioButtonChoices))?;
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
    pub fn set_choices(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_choices", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RadioButtonGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup")]
impl AsRef<crate::UnityEngine::UIElements::IGroupBox>
for crate::UnityEngine::UIElements::RadioButtonGroup {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IGroupBox {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup")]
impl AsMut<crate::UnityEngine::UIElements::IGroupBox>
for crate::UnityEngine::UIElements::RadioButtonGroup {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IGroupBox {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct RadioButtonGroup_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::RadioButtonGroup,
        *mut crate::UnityEngine::UIElements::RadioButtonGroup_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::RadioButtonGroup_UxmlFactory => "UnityEngine.UIElements"
    ."RadioButtonGroup/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RadioButtonGroup_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::RadioButtonGroup,
        *mut crate::UnityEngine::UIElements::RadioButtonGroup_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlFactory")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::RadioButtonGroup_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlFactory")]
impl crate::UnityEngine::UIElements::RadioButtonGroup_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RadioButtonGroup_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct RadioButtonGroup_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseFieldTraits_2<
        i32,
        *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    >,
    pub m_Choices: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::RadioButtonGroup_UxmlTraits => "UnityEngine.UIElements"
    ."RadioButtonGroup/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RadioButtonGroup_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseFieldTraits_2<
        i32,
        *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RadioButtonGroup_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlTraits")]
impl crate::UnityEngine::UIElements::RadioButtonGroup_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+RadioButtonGroup+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RadioButtonGroup_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
