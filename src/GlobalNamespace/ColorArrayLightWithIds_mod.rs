#[cfg(feature = "ColorArrayLightWithIds")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorArrayLightWithIds {
    __cordl_parent: crate::GlobalNamespace::LightWithIds,
    pub _colorArrayLightWithIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::ColorArrayLightWithIds_ColorArrayLightWithId,
        >,
    >,
    pub _materialController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MaterialController,
    >,
    pub _materialPropertyBlockControllers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::MaterialPropertyBlockController,
        >,
    >,
    pub _colorsArrayPropertyName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _colorsArrayOffsetPropertyName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _colorsArrayPropertyId: i32,
    pub _colorsArrayOffsetPropertyId: i32,
    pub _colorsArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
}
#[cfg(feature = "ColorArrayLightWithIds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ColorArrayLightWithIds => ""
    ."ColorArrayLightWithIds"
);
#[cfg(feature = "ColorArrayLightWithIds")]
impl std::ops::Deref for crate::GlobalNamespace::ColorArrayLightWithIds {
    type Target = crate::GlobalNamespace::LightWithIds;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorArrayLightWithIds")]
impl std::ops::DerefMut for crate::GlobalNamespace::ColorArrayLightWithIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorArrayLightWithIds")]
impl crate::GlobalNamespace::ColorArrayLightWithIds {
    #[cfg(feature = "ColorArrayLightWithIds+ColorArrayLightWithId")]
    pub type ColorArrayLightWithId = crate::GlobalNamespace::ColorArrayLightWithIds_ColorArrayLightWithId;
    pub fn GetLightWithIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LightWithIds_LightWithId,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::LightWithIds_LightWithId,
            >,
        > = __cordl_object.invoke("GetLightWithIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleColorLightWithIdDidSetColor(
        &mut self,
        index: i32,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleColorLightWithIdDidSetColor", (index, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessNewColorData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNewColorData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterArrayForColorChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterArrayForColorChanges", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorArrayOffsetToMaterialPropertyBlocks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorArrayOffsetToMaterialPropertyBlocks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorDataToMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorDataToMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterArrayFromColorChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterArrayFromColorChanges", ())?;
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
}
#[cfg(feature = "ColorArrayLightWithIds")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ColorArrayLightWithIds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ColorArrayLightWithIds+ColorArrayLightWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorArrayLightWithIds_ColorArrayLightWithId {
    __cordl_parent: crate::GlobalNamespace::LightWithIds_LightWithId,
    pub _index: i32,
    pub didSetColorEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<i32, crate::UnityEngine::Color>,
    >,
}
#[cfg(feature = "ColorArrayLightWithIds+ColorArrayLightWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ColorArrayLightWithIds_ColorArrayLightWithId => ""
    ."ColorArrayLightWithIds/ColorArrayLightWithId"
);
#[cfg(feature = "ColorArrayLightWithIds+ColorArrayLightWithId")]
impl std::ops::Deref
for crate::GlobalNamespace::ColorArrayLightWithIds_ColorArrayLightWithId {
    type Target = crate::GlobalNamespace::LightWithIds_LightWithId;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ColorArrayLightWithIds+ColorArrayLightWithId")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ColorArrayLightWithIds_ColorArrayLightWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ColorArrayLightWithIds+ColorArrayLightWithId")]
impl crate::GlobalNamespace::ColorArrayLightWithIds_ColorArrayLightWithId {
    pub fn ColorWasSet(
        &mut self,
        newColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColorWasSet", (newColor))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        index: i32,
        lightId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (index, lightId))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        index: i32,
        lightId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (index, lightId))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didSetColorEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<i32, crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSetColorEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSetColorEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<i32, crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSetColorEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ColorArrayLightWithIds+ColorArrayLightWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ColorArrayLightWithIds_ColorArrayLightWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
