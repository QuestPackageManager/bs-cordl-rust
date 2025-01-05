#[cfg(feature = "NamedColorListController")]
#[repr(C)]
#[derive(Debug)]
pub struct NamedColorListController {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ListColorController,
    >,
    pub _textValuePairs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::NamedColorListController_ColorValuePair,
            >,
        >,
    >,
    pub _value: i32,
    pub valueChangedEvent: quest_hook::libil2cpp::Gc<i32>,
}
#[cfg(feature = "NamedColorListController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NamedColorListController => ""
    ."NamedColorListController"
);
#[cfg(feature = "NamedColorListController")]
impl std::ops::Deref for crate::GlobalNamespace::NamedColorListController {
    type Target = quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ListColorController>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NamedColorListController")]
impl std::ops::DerefMut for crate::GlobalNamespace::NamedColorListController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NamedColorListController")]
impl crate::GlobalNamespace::NamedColorListController {
    #[cfg(feature = "NamedColorListController+ColorValuePair")]
    pub type ColorValuePair = crate::GlobalNamespace::NamedColorListController_ColorValuePair;
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
    pub fn ColorForValue(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("ColorForValue", (idx))?;
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
    pub fn Init(
        &mut self,
        values: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::NamedColorListController_ColorValuePair,
                >,
            >,
        >,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (values, value))?;
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
        value: quest_hook::libil2cpp::Gc<i32>,
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
        value: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_valueChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NamedColorListController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NamedColorListController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NamedColorListController")]
impl AsRef<quest_hook::libil2cpp::Gc<i32>>
for crate::GlobalNamespace::NamedColorListController {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<i32> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NamedColorListController")]
impl AsMut<quest_hook::libil2cpp::Gc<i32>>
for crate::GlobalNamespace::NamedColorListController {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<i32> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NamedColorListController+ColorValuePair")]
#[repr(C)]
#[derive(Debug)]
pub struct NamedColorListController_ColorValuePair {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub color: crate::UnityEngine::Color,
    pub value: i32,
}
#[cfg(feature = "NamedColorListController+ColorValuePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NamedColorListController_ColorValuePair => ""
    ."NamedColorListController/ColorValuePair"
);
#[cfg(feature = "NamedColorListController+ColorValuePair")]
impl std::ops::Deref
for crate::GlobalNamespace::NamedColorListController_ColorValuePair {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NamedColorListController+ColorValuePair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::NamedColorListController_ColorValuePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NamedColorListController+ColorValuePair")]
impl crate::GlobalNamespace::NamedColorListController_ColorValuePair {
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
}
#[cfg(feature = "NamedColorListController+ColorValuePair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NamedColorListController_ColorValuePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
