#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
#[repr(C)]
#[derive(Debug)]
pub struct RandomizeAvatarColorMap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _totalIndices_k__BackingField: i32,
    pub _colorIndices_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarSDK::RandomizeAvatarColorMap => "BeatSaber.BeatAvatarSDK"
    ."RandomizeAvatarColorMap"
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::RandomizeAvatarColorMap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::RandomizeAvatarColorMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
impl crate::BeatSaber::BeatAvatarSDK::RandomizeAvatarColorMap {
    pub fn New(
        headTopPrimaryColorIndex: i32,
        headTopSecondaryColorIndex: i32,
        glassesColorIndex: i32,
        facialHairColorIndex: i32,
        handsColorIndex: i32,
        clothesPrimaryColorIndex: i32,
        clothesSecondaryColorIndex: i32,
        clothesDetailColorIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    headTopPrimaryColorIndex,
                    headTopSecondaryColorIndex,
                    glassesColorIndex,
                    facialHairColorIndex,
                    handsColorIndex,
                    clothesPrimaryColorIndex,
                    clothesSecondaryColorIndex,
                    clothesDetailColorIndex,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        headTopPrimaryColorIndex: i32,
        headTopSecondaryColorIndex: i32,
        glassesColorIndex: i32,
        facialHairColorIndex: i32,
        handsColorIndex: i32,
        clothesPrimaryColorIndex: i32,
        clothesSecondaryColorIndex: i32,
        clothesDetailColorIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    headTopPrimaryColorIndex,
                    headTopSecondaryColorIndex,
                    glassesColorIndex,
                    facialHairColorIndex,
                    handsColorIndex,
                    clothesPrimaryColorIndex,
                    clothesSecondaryColorIndex,
                    clothesDetailColorIndex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorIndices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object.invoke("get_colorIndices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalIndices(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_totalIndices", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+RandomizeAvatarColorMap")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::RandomizeAvatarColorMap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
