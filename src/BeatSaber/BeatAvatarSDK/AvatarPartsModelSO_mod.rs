#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModelSO")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarPartsModelSO {
    __cordl_parent: PersistentScriptableObject,
    pub _headTops: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
    >,
    pub _eyes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
    >,
    pub _mouths: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
    >,
    pub _glasses: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
    >,
    pub _facialHair: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
    >,
    pub _hands: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
    >,
    pub _clothes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
    >,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModelSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO =>
    "BeatSaber.BeatAvatarSDK"."AvatarPartsModelSO"
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModelSO")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModelSO")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModelSO")]
impl crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_Clothes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        > = __cordl_object.invoke("get_Clothes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Eyes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
        > = __cordl_object.invoke("get_Eyes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FacialHair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        > = __cordl_object.invoke("get_FacialHair", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Glasses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        > = __cordl_object.invoke("get_Glasses", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Hands(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        > = __cordl_object.invoke("get_Hands", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Mouths(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
        > = __cordl_object.invoke("get_Mouths", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_headTops(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
        > = __cordl_object.invoke("get_headTops", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModelSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
