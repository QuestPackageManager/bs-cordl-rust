#[cfg(feature = "UnityEngine+UIElements+UxmlAsset")]
#[repr(C)]
#[derive(Debug)]
pub struct UxmlAsset {
    __cordl_parent: crate::System::Object,
    pub m_FullTypeName: *mut crate::System::String,
    pub m_Id: i32,
    pub m_OrderInDocument: i32,
    pub m_ParentId: i32,
    pub m_Properties: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UxmlAsset")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UxmlAsset =>
    "UnityEngine.UIElements"."UxmlAsset"
);
#[cfg(feature = "UnityEngine+UIElements+UxmlAsset")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UxmlAsset {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlAsset")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UxmlAsset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlAsset")]
impl crate::UnityEngine::UIElements::UxmlAsset {
    pub fn SetAttribute(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAttribute", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn SetOrAddProperty(
        &mut self,
        propertyName: *mut crate::System::String,
        propertyValue: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOrAddProperty", (propertyName, propertyValue))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetAttributeValue(
        &mut self,
        propertyName: *mut crate::System::String,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetAttributeValue", (propertyName, value))?;
        Ok(__cordl_ret)
    }
    pub fn get_fullTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_fullTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_id(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_orderInDocument(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_orderInDocument", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_parentId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_parentId", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UxmlAsset")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::UxmlAsset {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
