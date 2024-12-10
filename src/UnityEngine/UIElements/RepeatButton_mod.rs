#[cfg(feature = "UnityEngine+UIElements+RepeatButton")]
#[repr(C)]
#[derive(Debug)]
pub struct RepeatButton {
    __cordl_parent: crate::UnityEngine::UIElements::TextElement,
    pub m_Clickable: *mut crate::UnityEngine::UIElements::Clickable,
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RepeatButton =>
    "UnityEngine.UIElements"."RepeatButton"
);
#[cfg(feature = "UnityEngine+UIElements+RepeatButton")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RepeatButton {
    type Target = crate::UnityEngine::UIElements::TextElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RepeatButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton")]
impl crate::UnityEngine::UIElements::RepeatButton {
    #[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::RepeatButton_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::RepeatButton_UxmlTraits;
    pub fn AddAction(
        &mut self,
        clickEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAction", (clickEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Action_i64_i64_1(
        clickEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
        delay: i64,
        interval: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (clickEvent, delay, interval))?;
        Ok(__cordl_object.into())
    }
    pub fn SetAction(
        &mut self,
        clickEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
        delay: i64,
        interval: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAction", (clickEvent, delay, interval))?;
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
    pub fn _ctor_Action_i64_i64_1(
        &mut self,
        clickEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
        delay: i64,
        interval: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (clickEvent, delay, interval))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::RepeatButton {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct RepeatButton_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::RepeatButton,
        *mut crate::UnityEngine::UIElements::RepeatButton_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::RepeatButton_UxmlFactory => "UnityEngine.UIElements"
    ."RepeatButton/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RepeatButton_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::RepeatButton,
        *mut crate::UnityEngine::UIElements::RepeatButton_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RepeatButton_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlFactory")]
impl crate::UnityEngine::UIElements::RepeatButton_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RepeatButton_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct RepeatButton_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::TextElement_UxmlTraits,
    pub m_Delay: *mut crate::UnityEngine::UIElements::UxmlLongAttributeDescription,
    pub m_Interval: *mut crate::UnityEngine::UIElements::UxmlLongAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RepeatButton_UxmlTraits
    => "UnityEngine.UIElements"."RepeatButton/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RepeatButton_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::TextElement_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RepeatButton_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlTraits")]
impl crate::UnityEngine::UIElements::RepeatButton_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+RepeatButton+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RepeatButton_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
