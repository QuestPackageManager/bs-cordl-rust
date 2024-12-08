#[cfg(feature = "UnityEngine+UIElements+RectIntField")]
#[repr(C)]
#[derive(Debug)]
pub struct RectIntField {
    __cordl_parent: crate::UnityEngine::UIElements::BaseCompositeField_3<
        crate::UnityEngine::RectInt,
        *mut crate::UnityEngine::UIElements::IntegerField,
        i32,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RectIntField =>
    "UnityEngine.UIElements"."RectIntField"
);
#[cfg(feature = "UnityEngine+UIElements+RectIntField")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RectIntField {
    type Target = crate::UnityEngine::UIElements::BaseCompositeField_3<
        crate::UnityEngine::RectInt,
        *mut crate::UnityEngine::UIElements::IntegerField,
        i32,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RectIntField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField")]
impl crate::UnityEngine::UIElements::RectIntField {
    #[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::RectIntField_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::RectIntField_UxmlTraits;
    #[cfg(feature = "UnityEngine+UIElements+RectIntField+__c")]
    pub type __c = crate::UnityEngine::UIElements::RectIntField___c;
    pub fn DescribeFields(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
                crate::UnityEngine::RectInt,
                *mut crate::UnityEngine::UIElements::IntegerField,
                i32,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::BaseCompositeField_3_FieldDescription<
                crate::UnityEngine::RectInt,
                *mut crate::UnityEngine::UIElements::IntegerField,
                i32,
            >,
        > = __cordl_object.invoke("DescribeFields", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        label: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label))?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::RectIntField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct RectIntField_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::RectIntField,
        *mut crate::UnityEngine::UIElements::RectIntField_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::RectIntField_UxmlFactory => "UnityEngine.UIElements"
    ."RectIntField/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RectIntField_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::RectIntField,
        *mut crate::UnityEngine::UIElements::RectIntField_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RectIntField_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlFactory")]
impl crate::UnityEngine::UIElements::RectIntField_UxmlFactory {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RectIntField_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct RectIntField_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        crate::UnityEngine::RectInt,
    >,
    pub m_XValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_YValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_WValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_HValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RectIntField_UxmlTraits
    => "UnityEngine.UIElements"."RectIntField/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RectIntField_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        crate::UnityEngine::RectInt,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RectIntField_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlTraits")]
impl crate::UnityEngine::UIElements::RectIntField_UxmlTraits {
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+UIElements+RectIntField+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RectIntField_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
