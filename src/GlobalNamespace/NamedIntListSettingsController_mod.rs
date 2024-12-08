#[cfg(feature = "NamedIntListSettingsController")]
#[repr(C)]
#[derive(Debug)]
pub struct NamedIntListSettingsController {
    __cordl_parent: ListSettingsController,
    pub _textValuePairs: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::NamedIntListSettingsController_TextValuePair,
    >,
    pub valueDidChangeEvent: *mut crate::System::Action_1<i32>,
    pub _selectedIndex: i32,
}
#[cfg(feature = "NamedIntListSettingsController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NamedIntListSettingsController => ""
    ."NamedIntListSettingsController"
);
#[cfg(feature = "NamedIntListSettingsController")]
impl std::ops::Deref for NamedIntListSettingsController {
    type Target = ListSettingsController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NamedIntListSettingsController")]
impl std::ops::DerefMut for NamedIntListSettingsController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NamedIntListSettingsController")]
impl NamedIntListSettingsController {
    #[cfg(feature = "NamedIntListSettingsController+TextValuePair")]
    pub type TextValuePair = crate::GlobalNamespace::NamedIntListSettingsController_TextValuePair;
    #[cfg(feature = "NamedIntListSettingsController+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::GlobalNamespace::NamedIntListSettingsController___c__DisplayClass6_0;
    pub fn SetValue(
        &mut self,
        value: i32,
        applyValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value, applyValue))?;
        Ok(__cordl_ret)
    }
    pub fn GetInitValues(
        &mut self,
        idx: quest_hook::libil2cpp::ByRefMut<i32>,
        numberOfElements: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetInitValues", (idx, numberOfElements))?;
        Ok(__cordl_ret)
    }
    pub fn TextForValue(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("TextForValue", (idx))?;
        Ok(__cordl_ret)
    }
    pub fn remove_valueDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_valueDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_valueDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_valueDidChangeEvent", (value))?;
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
    pub fn ApplyValue(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyValue", (idx))?;
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
#[cfg(feature = "NamedIntListSettingsController")]
impl quest_hook::libil2cpp::ObjectType for NamedIntListSettingsController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NamedIntListSettingsController+TextValuePair")]
#[repr(C)]
#[derive(Debug)]
pub struct NamedIntListSettingsController_TextValuePair {
    __cordl_parent: crate::System::Object,
    pub text: *mut crate::System::String,
    pub value: i32,
}
#[cfg(feature = "NamedIntListSettingsController+TextValuePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NamedIntListSettingsController_TextValuePair => ""
    ."NamedIntListSettingsController/TextValuePair"
);
#[cfg(feature = "NamedIntListSettingsController+TextValuePair")]
impl std::ops::Deref
for crate::GlobalNamespace::NamedIntListSettingsController_TextValuePair {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NamedIntListSettingsController+TextValuePair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::NamedIntListSettingsController_TextValuePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NamedIntListSettingsController+TextValuePair")]
impl crate::GlobalNamespace::NamedIntListSettingsController_TextValuePair {
    pub fn get_localizedText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_localizedText", ())?;
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
#[cfg(feature = "NamedIntListSettingsController+TextValuePair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NamedIntListSettingsController_TextValuePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
