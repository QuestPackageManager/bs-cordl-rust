#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3IntField_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::Vector3IntField,
        *mut crate::UnityEngine::UIElements::Vector3IntField_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Vector3IntField_UxmlFactory => "UnityEngine.UIElements"
    ."Vector3IntField/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Vector3IntField_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::Vector3IntField,
        *mut crate::UnityEngine::UIElements::Vector3IntField_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Vector3IntField_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlFactory")]
impl crate::UnityEngine::UIElements::Vector3IntField_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Vector3IntField_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3IntField_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        crate::UnityEngine::Vector3Int,
    >,
    pub m_XValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_YValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_ZValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Vector3IntField_UxmlTraits => "UnityEngine.UIElements"
    ."Vector3IntField/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Vector3IntField_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        crate::UnityEngine::Vector3Int,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Vector3IntField_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlTraits")]
impl crate::UnityEngine::UIElements::Vector3IntField_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Vector3IntField_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector3IntField {
    __cordl_parent: crate::UnityEngine::UIElements::BaseCompositeField_3<
        crate::UnityEngine::Vector3Int,
        *mut crate::UnityEngine::UIElements::IntegerField,
        i32,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Vector3IntField =>
    "UnityEngine.UIElements"."Vector3IntField"
);
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Vector3IntField {
    type Target = crate::UnityEngine::UIElements::BaseCompositeField_3<
        crate::UnityEngine::Vector3Int,
        *mut crate::UnityEngine::UIElements::IntegerField,
        i32,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Vector3IntField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField")]
impl crate::UnityEngine::UIElements::Vector3IntField {
    #[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::Vector3IntField_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+Vector3IntField+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::Vector3IntField_UxmlTraits;
    #[cfg(feature = "UnityEngine+UIElements+Vector3IntField+__c")]
    pub type __c = crate::UnityEngine::UIElements::Vector3IntField___c;
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
                crate::UnityEngine::Vector3Int,
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
                crate::UnityEngine::Vector3Int,
                *mut crate::UnityEngine::UIElements::IntegerField,
                i32,
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
#[cfg(feature = "UnityEngine+UIElements+Vector3IntField")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Vector3IntField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
