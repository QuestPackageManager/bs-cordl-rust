#[cfg(feature = "UnityEngine+UIElements+RectField")]
#[repr(C)]
#[derive(Debug)]
pub struct RectField {
    __cordl_parent: crate::UnityEngine::UIElements::BaseCompositeField_3<
        crate::UnityEngine::Rect,
        *mut crate::UnityEngine::UIElements::FloatField,
        f32,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RectField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RectField =>
    "UnityEngine.UIElements"."RectField"
);
#[cfg(feature = "UnityEngine+UIElements+RectField")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RectField {
    type Target = crate::UnityEngine::UIElements::BaseCompositeField_3<
        crate::UnityEngine::Rect,
        *mut crate::UnityEngine::UIElements::FloatField,
        f32,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectField")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RectField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectField")]
impl crate::UnityEngine::UIElements::RectField {
    #[cfg(feature = "UnityEngine+UIElements+RectField+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::RectField_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+RectField+__c")]
    pub type __c = crate::UnityEngine::UIElements::RectField___c;
    #[cfg(feature = "UnityEngine+UIElements+RectField+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::RectField_UxmlTraits;
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
    pub fn _ctor_String1(
        &mut self,
        label: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (label))?;
        Ok(__cordl_ret)
    }
    pub fn DescribeFields(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
                crate::UnityEngine::Rect,
                *mut crate::UnityEngine::UIElements::FloatField,
                f32,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
                crate::UnityEngine::Rect,
                *mut crate::UnityEngine::UIElements::FloatField,
                f32,
            >,
        > = __cordl_object.invoke("DescribeFields", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        label: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectField")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::RectField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct RectField_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::RectField,
        *mut crate::UnityEngine::UIElements::RectField_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RectField_UxmlFactory
    => "UnityEngine.UIElements"."RectField/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RectField_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::RectField,
        *mut crate::UnityEngine::UIElements::RectField_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RectField_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlFactory")]
impl crate::UnityEngine::UIElements::RectField_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RectField_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct RectField_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        crate::UnityEngine::Rect,
    >,
    pub m_XValue: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_YValue: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_WValue: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_HValue: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RectField_UxmlTraits =>
    "UnityEngine.UIElements"."RectField/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RectField_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        crate::UnityEngine::Rect,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RectField_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlTraits")]
impl crate::UnityEngine::UIElements::RectField_UxmlTraits {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectField+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RectField_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
