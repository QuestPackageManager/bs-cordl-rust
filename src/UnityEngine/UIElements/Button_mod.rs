#[cfg(feature = "UnityEngine+UIElements+Button")]
#[repr(C)]
#[derive(Debug)]
pub struct Button {
    __cordl_parent: crate::UnityEngine::UIElements::TextElement,
    pub m_Clickable: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Clickable,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Button")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Button =>
    "UnityEngine.UIElements"."Button"
);
#[cfg(feature = "UnityEngine+UIElements+Button")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Button {
    type Target = crate::UnityEngine::UIElements::TextElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Button")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Button")]
impl crate::UnityEngine::UIElements::Button {
    #[cfg(feature = "UnityEngine+UIElements+Button+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::Button_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+Button+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::Button_UxmlTraits;
    pub fn DoMeasure(
        &mut self,
        desiredWidth: f32,
        widthMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
        desiredHeight: f32,
        heightMode: crate::UnityEngine::UIElements::VisualElement_MeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("DoMeasure", (desiredWidth, widthMode, desiredHeight, heightMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Action1(
        clickEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (clickEvent))?;
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
    pub fn _ctor_Action1(
        &mut self,
        clickEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (clickEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clickable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Clickable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Clickable,
        > = __cordl_object.invoke("get_clickable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clickable(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Clickable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clickable", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Button")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::Button {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct Button_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Button>,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Button_UxmlTraits>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Button_UxmlFactory =>
    "UnityEngine.UIElements"."Button/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Button_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Button>,
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Button_UxmlTraits>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Button_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlFactory")]
impl crate::UnityEngine::UIElements::Button_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Button_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct Button_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::TextElement_UxmlTraits,
}
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Button_UxmlTraits =>
    "UnityEngine.UIElements"."Button/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Button_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::TextElement_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Button_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlTraits")]
impl crate::UnityEngine::UIElements::Button_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+Button+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Button_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
