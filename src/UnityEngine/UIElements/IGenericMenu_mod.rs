#[cfg(feature = "UnityEngine+UIElements+IGenericMenu")]
#[repr(C)]
#[derive(Debug)]
pub struct IGenericMenu {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IGenericMenu")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IGenericMenu =>
    "UnityEngine.UIElements"."IGenericMenu"
);
#[cfg(feature = "UnityEngine+UIElements+IGenericMenu")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IGenericMenu {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IGenericMenu")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IGenericMenu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IGenericMenu")]
impl crate::UnityEngine::UIElements::IGenericMenu {
    pub fn AddItem_Action0(
        &mut self,
        itemName: *mut crate::System::String,
        isChecked: bool,
        action: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddItem", (itemName, isChecked, action))?;
        Ok(__cordl_ret)
    }
    pub fn AddItem_Action_1_Object1(
        &mut self,
        itemName: *mut crate::System::String,
        isChecked: bool,
        action: *mut crate::System::Action_1<*mut crate::System::Object>,
        data: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddItem", (itemName, isChecked, action, data))?;
        Ok(__cordl_ret)
    }
    pub fn DropDown(
        &mut self,
        position: crate::UnityEngine::Rect,
        targetElement: *mut crate::UnityEngine::UIElements::VisualElement,
        anchored: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DropDown", (position, targetElement, anchored))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IGenericMenu")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::IGenericMenu {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
