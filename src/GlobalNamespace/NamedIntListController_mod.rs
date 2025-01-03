#[cfg(feature = "NamedIntListController")]
#[repr(C)]
#[derive(Debug)]
pub struct NamedIntListController {
    __cordl_parent: crate::GlobalNamespace::ListSettingsController,
    pub _textValuePairs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::NamedIntListController_TextValuePair,
        >,
    >,
    pub _value: i32,
    pub valueChangedEvent: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
}
#[cfg(feature = "NamedIntListController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NamedIntListController => ""
    ."NamedIntListController"
);
#[cfg(feature = "NamedIntListController")]
impl std::ops::Deref for crate::GlobalNamespace::NamedIntListController {
    type Target = crate::GlobalNamespace::ListSettingsController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NamedIntListController")]
impl std::ops::DerefMut for crate::GlobalNamespace::NamedIntListController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NamedIntListController")]
impl crate::GlobalNamespace::NamedIntListController {
    #[cfg(feature = "NamedIntListController+TextValuePair")]
    pub type TextValuePair = crate::GlobalNamespace::NamedIntListController_TextValuePair;
    pub fn ApplyValue(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyValue", (idx))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn InitValues(
        &mut self,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::NamedIntListController_TextValuePair,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitValues", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetValue(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TextForValue(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("TextForValue", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_valueChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_valueChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_valueChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_valueChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NamedIntListController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NamedIntListController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NamedIntListController")]
impl AsRef<crate::HMUI::IValueChanger_1<i32>>
for crate::GlobalNamespace::NamedIntListController {
    fn as_ref(&self) -> &crate::HMUI::IValueChanger_1<i32> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NamedIntListController")]
impl AsMut<crate::HMUI::IValueChanger_1<i32>>
for crate::GlobalNamespace::NamedIntListController {
    fn as_mut(&mut self) -> &mut crate::HMUI::IValueChanger_1<i32> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NamedIntListController+TextValuePair")]
#[repr(C)]
#[derive(Debug)]
pub struct NamedIntListController_TextValuePair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub localizationKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub value: i32,
}
#[cfg(feature = "NamedIntListController+TextValuePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NamedIntListController_TextValuePair => ""
    ."NamedIntListController/TextValuePair"
);
#[cfg(feature = "NamedIntListController+TextValuePair")]
impl std::ops::Deref for crate::GlobalNamespace::NamedIntListController_TextValuePair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NamedIntListController+TextValuePair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::NamedIntListController_TextValuePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NamedIntListController+TextValuePair")]
impl crate::GlobalNamespace::NamedIntListController_TextValuePair {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localizedText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_localizedText", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NamedIntListController+TextValuePair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NamedIntListController_TextValuePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
