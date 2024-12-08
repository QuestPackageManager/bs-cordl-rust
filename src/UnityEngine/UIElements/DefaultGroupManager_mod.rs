#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultGroupManager {
    __cordl_parent: crate::System::Object,
    pub m_GroupOptions: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::IGroupBoxOption,
    >,
    pub m_SelectedOption: *mut crate::UnityEngine::UIElements::IGroupBoxOption,
    pub m_GroupBox: *mut crate::UnityEngine::UIElements::IGroupBox,
}
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DefaultGroupManager =>
    "UnityEngine.UIElements"."DefaultGroupManager"
);
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DefaultGroupManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DefaultGroupManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
impl crate::UnityEngine::UIElements::DefaultGroupManager {
    pub fn UnregisterOption(
        &mut self,
        option: *mut crate::UnityEngine::UIElements::IGroupBoxOption,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterOption", (option))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        groupBox: *mut crate::UnityEngine::UIElements::IGroupBox,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (groupBox))?;
        Ok(__cordl_ret)
    }
    pub fn OnOptionSelectionChanged(
        &mut self,
        selectedOption: *mut crate::UnityEngine::UIElements::IGroupBoxOption,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnOptionSelectionChanged", (selectedOption))?;
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
    pub fn RegisterOption(
        &mut self,
        option: *mut crate::UnityEngine::UIElements::IGroupBoxOption,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterOption", (option))?;
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
#[cfg(feature = "UnityEngine+UIElements+DefaultGroupManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DefaultGroupManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
