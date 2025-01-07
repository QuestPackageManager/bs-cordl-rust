#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModelSO")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarPartsModelSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _headTops: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO>,
        >,
    >,
    pub _eyes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
            >,
        >,
    >,
    pub _mouths: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
            >,
        >,
    >,
    pub _glasses: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO>,
        >,
    >,
    pub _facialHair: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO>,
        >,
    >,
    pub _hands: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO>,
        >,
    >,
    pub _clothes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO>,
        >,
    >,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModelSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarSDK";
    const CLASS_NAME: &'static str = "AvatarPartsModelSO";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPartsModelSO")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::AvatarPartsModelSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
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
    pub fn get_Clothes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        > = __cordl_object.invoke("get_Clothes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Eyes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
                >,
            >,
        > = __cordl_object.invoke("get_Eyes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FacialHair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        > = __cordl_object.invoke("get_FacialHair", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Glasses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        > = __cordl_object.invoke("get_Glasses", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Hands(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        > = __cordl_object.invoke("get_Hands", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Mouths(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarSpritePartSO,
                >,
            >,
        > = __cordl_object.invoke("get_Mouths", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headTops(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::BeatAvatarSDK::AvatarMeshPartSO,
                >,
            >,
        > = __cordl_object.invoke("get_headTops", ())?;
        Ok(__cordl_ret.into())
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
