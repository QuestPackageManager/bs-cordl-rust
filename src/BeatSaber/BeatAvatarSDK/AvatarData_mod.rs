#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarData")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _headTopId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _glassesId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _facialHairId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _handsId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _clothesId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _eyesId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _mouthId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _headTopPrimaryColor_k__BackingField: crate::UnityEngine::Color,
    pub _headTopSecondaryColor_k__BackingField: crate::UnityEngine::Color,
    pub _glassesColor_k__BackingField: crate::UnityEngine::Color,
    pub _facialHairColor_k__BackingField: crate::UnityEngine::Color,
    pub _handsColor_k__BackingField: crate::UnityEngine::Color,
    pub _clothesPrimaryColor_k__BackingField: crate::UnityEngine::Color,
    pub _clothesSecondaryColor_k__BackingField: crate::UnityEngine::Color,
    pub _clothesDetailColor_k__BackingField: crate::UnityEngine::Color,
    pub _skinColorId_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::BeatAvatarSDK::AvatarData =>
    "BeatSaber.BeatAvatarSDK"."AvatarData"
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarData")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::AvatarData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarData")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::AvatarData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarData")]
impl crate::BeatSaber::BeatAvatarSDK::AvatarData {
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        > = __cordl_object.invoke("Clone", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_AvatarData0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Color_Color_Il2CppString_Color_Il2CppString_Color_Il2CppString_Color_Il2CppString_Color_Color_Color_Il2CppString_Il2CppString_Il2CppString1(
        headTopId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        headTopPrimaryColor: crate::UnityEngine::Color,
        headTopSecondaryColor: crate::UnityEngine::Color,
        glassesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        glassesColor: crate::UnityEngine::Color,
        facialHairId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        facialHairColor: crate::UnityEngine::Color,
        handsId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        handsColor: crate::UnityEngine::Color,
        clothesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        clothesPrimaryColor: crate::UnityEngine::Color,
        clothesSecondaryColor: crate::UnityEngine::Color,
        clothesDetailColor: crate::UnityEngine::Color,
        skinColorId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eyesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mouthId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    headTopId,
                    headTopPrimaryColor,
                    headTopSecondaryColor,
                    glassesId,
                    glassesColor,
                    facialHairId,
                    facialHairColor,
                    handsId,
                    handsColor,
                    clothesId,
                    clothesPrimaryColor,
                    clothesSecondaryColor,
                    clothesDetailColor,
                    skinColorId,
                    eyesId,
                    mouthId,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Color_Color_Il2CppString_Color_Il2CppString_Color_Il2CppString_Color_Il2CppString_Color_Color_Color_Il2CppString_Il2CppString_Il2CppString1(
        &mut self,
        headTopId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        headTopPrimaryColor: crate::UnityEngine::Color,
        headTopSecondaryColor: crate::UnityEngine::Color,
        glassesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        glassesColor: crate::UnityEngine::Color,
        facialHairId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        facialHairColor: crate::UnityEngine::Color,
        handsId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        handsColor: crate::UnityEngine::Color,
        clothesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        clothesPrimaryColor: crate::UnityEngine::Color,
        clothesSecondaryColor: crate::UnityEngine::Color,
        clothesDetailColor: crate::UnityEngine::Color,
        skinColorId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        eyesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mouthId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    headTopId,
                    headTopPrimaryColor,
                    headTopSecondaryColor,
                    glassesId,
                    glassesColor,
                    facialHairId,
                    facialHairColor,
                    handsId,
                    handsColor,
                    clothesId,
                    clothesPrimaryColor,
                    clothesSecondaryColor,
                    clothesDetailColor,
                    skinColorId,
                    eyesId,
                    mouthId,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clothesDetailColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_clothesDetailColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clothesId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_clothesId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clothesPrimaryColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_clothesPrimaryColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_clothesSecondaryColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_clothesSecondaryColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_eyesId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_eyesId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_facialHairColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_facialHairColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_facialHairId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_facialHairId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_glassesColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_glassesColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_glassesId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_glassesId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_handsColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_handsColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_handsId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_handsId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headTopId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_headTopId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headTopPrimaryColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_headTopPrimaryColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headTopSecondaryColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_headTopSecondaryColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mouthId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_mouthId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_skinColorId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_skinColorId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clothesDetailColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clothesDetailColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clothesId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clothesId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clothesPrimaryColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clothesPrimaryColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_clothesSecondaryColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clothesSecondaryColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_eyesId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_eyesId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_facialHairColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_facialHairColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_facialHairId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_facialHairId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_glassesColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_glassesColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_glassesId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_glassesId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_handsColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_handsColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_handsId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_handsId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_headTopId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headTopId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_headTopPrimaryColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headTopPrimaryColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_headTopSecondaryColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headTopSecondaryColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mouthId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mouthId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_skinColorId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_skinColorId", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarData")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::BeatAvatarSDK::AvatarData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
