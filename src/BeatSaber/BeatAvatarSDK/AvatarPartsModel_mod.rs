#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarPartsModel {
    __cordl_parent: crate::System::Object,
    pub _headTopCollection_k__BackingField: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
    >,
    pub _eyesCollection_k__BackingField: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
    >,
    pub _mouthCollection_k__BackingField: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
    >,
    pub _glassesCollection_k__BackingField: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
    >,
    pub _facialHairCollection_k__BackingField: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
    >,
    pub _handsCollection_k__BackingField: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
    >,
    pub _clothesCollection_k__BackingField: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
    >,
    pub _skinColors_k__BackingField: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::BeatAvatarSDK::SkinColorSO,
    >,
    pub _indexById: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        i32,
    >,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::BeatAvatarSDK::AvatarPartsModel =>
    "BeatSaber.BeatAvatarSDK"."AvatarPartsModel"
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
impl crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel {
    pub fn GetColorIndexById(
        &mut self,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetColorIndexById", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetRandomColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::BeatAvatarSDK::SkinColorSO,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::BeatAvatarSDK::SkinColorSO = __cordl_object
            .invoke("GetRandomColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSkinColorById(
        &mut self,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::BeatAvatarSDK::SkinColorSO,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::BeatAvatarSDK::SkinColorSO = __cordl_object
            .invoke("GetSkinColorById", (id))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        avatarPartData: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO,
        skinColorSet: *mut crate::BeatSaber::BeatAvatarSDK::SkinColorSetSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (avatarPartData, skinColorSet))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        avatarPartData: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO,
        skinColorSet: *mut crate::BeatSaber::BeatAvatarSDK::SkinColorSetSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (avatarPartData, skinColorSet))?;
        Ok(__cordl_ret)
    }
    pub fn get_clothesCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        > = __cordl_object.invoke("get_clothesCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_eyesCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
        > = __cordl_object.invoke("get_eyesCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_facialHairCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        > = __cordl_object.invoke("get_facialHairCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_glassesCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        > = __cordl_object.invoke("get_glassesCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_handsCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        > = __cordl_object.invoke("get_handsCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_headTopCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        > = __cordl_object.invoke("get_headTopCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_mouthCollection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartCollection_1<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
        > = __cordl_object.invoke("get_mouthCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_skinColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::SkinColorSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::SkinColorSO,
        > = __cordl_object.invoke("get_skinColors", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
