#[cfg(feature = "UnityEngine+UIElements+GroupBox")]
#[repr(C)]
#[derive(Debug)]
pub struct GroupBox {
    __cordl_parent: crate::UnityEngine::UIElements::BindableElement,
    pub m_TitleLabel: *mut crate::UnityEngine::UIElements::Label,
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::GroupBox =>
    "UnityEngine.UIElements"."GroupBox"
);
#[cfg(feature = "UnityEngine+UIElements+GroupBox")]
impl std::ops::Deref for crate::UnityEngine::UIElements::GroupBox {
    type Target = crate::UnityEngine::UIElements::BindableElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::GroupBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox")]
impl crate::UnityEngine::UIElements::GroupBox {
    #[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::GroupBox_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::GroupBox_UxmlTraits;
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString1(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (text))?;
        Ok(__cordl_object.into())
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
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_text(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_text", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::GroupBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox")]
impl AsRef<crate::UnityEngine::UIElements::IGroupBox>
for crate::UnityEngine::UIElements::GroupBox {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IGroupBox {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox")]
impl AsMut<crate::UnityEngine::UIElements::IGroupBox>
for crate::UnityEngine::UIElements::GroupBox {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IGroupBox {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct GroupBox_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::GroupBox,
        *mut crate::UnityEngine::UIElements::GroupBox_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::GroupBox_UxmlFactory =>
    "UnityEngine.UIElements"."GroupBox/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::GroupBox_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::GroupBox,
        *mut crate::UnityEngine::UIElements::GroupBox_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::GroupBox_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlFactory")]
impl crate::UnityEngine::UIElements::GroupBox_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::GroupBox_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct GroupBox_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BindableElement_UxmlTraits,
    pub m_Text: *mut crate::UnityEngine::UIElements::UxmlStringAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::GroupBox_UxmlTraits =>
    "UnityEngine.UIElements"."GroupBox/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::GroupBox_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BindableElement_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::GroupBox_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlTraits")]
impl crate::UnityEngine::UIElements::GroupBox_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+GroupBox+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::GroupBox_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
