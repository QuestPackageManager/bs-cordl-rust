#[cfg(feature = "UnityEngine+UIElements+EnumField")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumField {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1<
        *mut crate::System::Enum,
    >,
    pub m_EnumType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub m_IncludeObsoleteValues: bool,
    pub m_TextElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::TextElement,
    >,
    pub m_ArrowElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_EnumData: crate::UnityEngine::EnumData,
    pub createMenuCallback: quest_hook::libil2cpp::Gc<
        crate::System::Func_1<*mut crate::UnityEngine::UIElements::IGenericMenu>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+EnumField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EnumField =>
    "UnityEngine.UIElements"."EnumField"
);
#[cfg(feature = "UnityEngine+UIElements+EnumField")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EnumField {
    type Target = crate::UnityEngine::UIElements::BaseField_1<*mut crate::System::Enum>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumField")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EnumField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumField")]
impl crate::UnityEngine::UIElements::EnumField {
    #[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::EnumField_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::EnumField_UxmlTraits;
    pub fn ChangeValueFromMenu(
        &mut self,
        menuItem: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeValueFromMenu", (menuItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsPointer(
        &mut self,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ContainsPointer", (pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_Enum0(
        &mut self,
        defaultValue: quest_hook::libil2cpp::Gc<crate::System::Enum>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init__cordl_bool1(
        &mut self,
        defaultValue: quest_hook::libil2cpp::Gc<crate::System::Enum>,
        includeObsoleteValues: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (defaultValue, includeObsoleteValues))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        defaultValue: quest_hook::libil2cpp::Gc<crate::System::Enum>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Enum1(
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultValue: quest_hook::libil2cpp::Gc<crate::System::Enum>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label, defaultValue))?;
        Ok(__cordl_object.into())
    }
    pub fn OnNavigationSubmit(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::NavigationSubmitEvent,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNavigationSubmit", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerDownEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerDownEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDownEvent", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerMoveEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::PointerMoveEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerMoveEvent", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateDataFromType(
        &mut self,
        enumType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateDataFromType", (enumType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessPointerDown<T>(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::PointerEventBase_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPointerDown", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetValueWithoutNotify(
        &mut self,
        newValue: quest_hook::libil2cpp::Gc<crate::System::Enum>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValueWithoutNotify", (newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowMenu(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowMenu", ())?;
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
    pub fn UpdateValueLabel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Enum>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValueLabel", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ShowMenu_b__33_0(
        &mut self,
        contentView: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<ShowMenu>b__33_0", (contentView))?;
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
    pub fn _ctor_Il2CppString_Enum1(
        &mut self,
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        defaultValue: quest_hook::libil2cpp::Gc<crate::System::Enum>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (label, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_includeObsoleteValues(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_includeObsoleteValues", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumField")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::EnumField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumField_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::EnumField,
        *mut crate::UnityEngine::UIElements::EnumField_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EnumField_UxmlFactory
    => "UnityEngine.UIElements"."EnumField/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EnumField_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::EnumField,
        *mut crate::UnityEngine::UIElements::EnumField_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EnumField_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlFactory")]
impl crate::UnityEngine::UIElements::EnumField_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EnumField_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct EnumField_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        *mut crate::System::Enum,
    >,
    pub m_Type: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlTypeAttributeDescription_1<
            *mut crate::System::Enum,
        >,
    >,
    pub m_Value: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    >,
    pub m_IncludeObsoleteValues: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EnumField_UxmlTraits =>
    "UnityEngine.UIElements"."EnumField/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EnumField_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        *mut crate::System::Enum,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EnumField_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlTraits")]
impl crate::UnityEngine::UIElements::EnumField_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+EnumField+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EnumField_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
