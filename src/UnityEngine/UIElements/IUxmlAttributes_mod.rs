#[cfg(feature = "UnityEngine+UIElements+IUxmlAttributes")]
#[repr(C)]
#[derive(Debug)]
pub struct IUxmlAttributes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlAttributes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IUxmlAttributes =>
    "UnityEngine.UIElements"."IUxmlAttributes"
);
#[cfg(feature = "UnityEngine+UIElements+IUxmlAttributes")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IUxmlAttributes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlAttributes")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IUxmlAttributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlAttributes")]
impl crate::UnityEngine::UIElements::IUxmlAttributes {
    pub fn TryGetAttributeValue(
        &mut self,
        attributeName: *mut crate::System::String,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetAttributeValue", (attributeName, value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IUxmlAttributes")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IUxmlAttributes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}