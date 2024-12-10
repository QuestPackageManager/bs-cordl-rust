#[cfg(feature = "UnityEngine+UIElements+IGroupManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IGroupManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IGroupManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IGroupManager =>
    "UnityEngine.UIElements"."IGroupManager"
);
#[cfg(feature = "UnityEngine+UIElements+IGroupManager")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IGroupManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IGroupManager")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IGroupManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IGroupManager")]
impl crate::UnityEngine::UIElements::IGroupManager {
    pub fn Init(
        &mut self,
        groupBox: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IGroupBox>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (groupBox))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnOptionSelectionChanged(
        &mut self,
        selectedOption: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGroupBoxOption,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnOptionSelectionChanged", (selectedOption))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterOption(
        &mut self,
        option: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGroupBoxOption,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterOption", (option))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterOption(
        &mut self,
        option: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IGroupBoxOption,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterOption", (option))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IGroupManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IGroupManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
