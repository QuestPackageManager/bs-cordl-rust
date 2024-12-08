#[cfg(feature = "UnityEngine+UIElements+BoundsIntField")]
#[repr(C)]
#[derive(Debug)]
pub struct BoundsIntField {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1<
        crate::UnityEngine::BoundsInt,
    >,
    pub m_PositionField: *mut crate::UnityEngine::UIElements::Vector3IntField,
    pub m_SizeField: *mut crate::UnityEngine::UIElements::Vector3IntField,
}
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::BoundsIntField =>
    "UnityEngine.UIElements"."BoundsIntField"
);
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BoundsIntField {
    type Target = crate::UnityEngine::UIElements::BaseField_1<
        crate::UnityEngine::BoundsInt,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BoundsIntField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField")]
impl crate::UnityEngine::UIElements::BoundsIntField {
    #[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::BoundsIntField_UxmlTraits;
    #[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::BoundsIntField_UxmlFactory;
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
    pub fn SetValueWithoutNotify(
        &mut self,
        newValue: crate::UnityEngine::BoundsInt,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValueWithoutNotify", (newValue))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMixedValueContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMixedValueContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn __ctor_b__10_0(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::ChangeEvent_1<
            crate::UnityEngine::Vector3Int,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__10_0", (e))?;
        Ok(__cordl_ret)
    }
    pub fn __ctor_b__10_1(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::ChangeEvent_1<
            crate::UnityEngine::Vector3Int,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__10_1", (e))?;
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
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BoundsIntField {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct BoundsIntField_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::BoundsIntField,
        *mut crate::UnityEngine::UIElements::BoundsIntField_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::BoundsIntField_UxmlFactory => "UnityEngine.UIElements"
    ."BoundsIntField/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BoundsIntField_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::BoundsIntField,
        *mut crate::UnityEngine::UIElements::BoundsIntField_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BoundsIntField_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlFactory")]
impl crate::UnityEngine::UIElements::BoundsIntField_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BoundsIntField_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct BoundsIntField_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        crate::UnityEngine::BoundsInt,
    >,
    pub m_PositionXValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_PositionYValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_PositionZValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_SizeXValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_SizeYValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_SizeZValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::BoundsIntField_UxmlTraits => "UnityEngine.UIElements"
    ."BoundsIntField/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::BoundsIntField_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        crate::UnityEngine::BoundsInt,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::BoundsIntField_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlTraits")]
impl crate::UnityEngine::UIElements::BoundsIntField_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+BoundsIntField+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::BoundsIntField_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
