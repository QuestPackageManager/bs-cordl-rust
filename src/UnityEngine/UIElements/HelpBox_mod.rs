#[cfg(feature = "UnityEngine+UIElements+HelpBox")]
#[repr(C)]
#[derive(Debug)]
pub struct HelpBox {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement,
    pub m_HelpBoxMessageType: crate::UnityEngine::UIElements::HelpBoxMessageType,
    pub m_Icon: *mut crate::UnityEngine::UIElements::VisualElement,
    pub m_IconClass: *mut crate::System::String,
    pub m_Label: *mut crate::UnityEngine::UIElements::Label,
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::HelpBox =>
    "UnityEngine.UIElements"."HelpBox"
);
#[cfg(feature = "UnityEngine+UIElements+HelpBox")]
impl std::ops::Deref for crate::UnityEngine::UIElements::HelpBox {
    type Target = crate::UnityEngine::UIElements::VisualElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::HelpBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox")]
impl crate::UnityEngine::UIElements::HelpBox {
    #[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::HelpBox_UxmlTraits;
    #[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::HelpBox_UxmlFactory;
    pub fn set_messageType(
        &mut self,
        value: crate::UnityEngine::UIElements::HelpBoxMessageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_messageType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateIcon(
        &mut self,
        messageType: crate::UnityEngine::UIElements::HelpBoxMessageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIcon", (messageType))?;
        Ok(__cordl_ret)
    }
    pub fn set_text(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_text", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_HelpBoxMessageType1(
        &mut self,
        text: *mut crate::System::String,
        messageType: crate::UnityEngine::UIElements::HelpBoxMessageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (text, messageType))?;
        Ok(__cordl_ret)
    }
    pub fn GetIconClass(
        &mut self,
        messageType: crate::UnityEngine::UIElements::HelpBoxMessageType,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetIconClass", (messageType))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String_HelpBoxMessageType1(
        text: *mut crate::System::String,
        messageType: crate::UnityEngine::UIElements::HelpBoxMessageType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (text, messageType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::HelpBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct HelpBox_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::HelpBox,
        *mut crate::UnityEngine::UIElements::HelpBox_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::HelpBox_UxmlFactory =>
    "UnityEngine.UIElements"."HelpBox/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::HelpBox_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::HelpBox,
        *mut crate::UnityEngine::UIElements::HelpBox_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::HelpBox_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlFactory")]
impl crate::UnityEngine::UIElements::HelpBox_UxmlFactory {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::HelpBox_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct HelpBox_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement_UxmlTraits,
    pub m_Text: *mut crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
    pub m_MessageType: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::HelpBoxMessageType,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::HelpBox_UxmlTraits =>
    "UnityEngine.UIElements"."HelpBox/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::HelpBox_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::VisualElement_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::HelpBox_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlTraits")]
impl crate::UnityEngine::UIElements::HelpBox_UxmlTraits {
    pub fn Init(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        bag: *mut crate::UnityEngine::UIElements::IUxmlAttributes,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+HelpBox+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::HelpBox_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
