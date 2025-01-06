#[cfg(feature = "PackPromoInfoSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PackPromoInfoSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _promoBannerInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PromoBannerInfoSO,
    >,
    pub _levelsPromoInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo,
            >,
        >,
    >,
}
#[cfg(feature = "PackPromoInfoSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PackPromoInfoSO => ""
    ."PackPromoInfoSO"
);
#[cfg(feature = "PackPromoInfoSO")]
impl std::ops::Deref for crate::GlobalNamespace::PackPromoInfoSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackPromoInfoSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PackPromoInfoSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackPromoInfoSO")]
impl crate::GlobalNamespace::PackPromoInfoSO {
    #[cfg(feature = "PackPromoInfoSO+LevelPromoInfo")]
    pub type LevelPromoInfo = crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo;
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
    pub fn get_levelPromoInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo,
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
                    crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo,
                >,
            >,
        > = __cordl_object.invoke("get_levelPromoInfos", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_promoBannerInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PromoBannerInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PromoBannerInfoSO,
        > = __cordl_object.invoke("get_promoBannerInfo", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PackPromoInfoSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PackPromoInfoSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PackPromoInfoSO+LevelPromoInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct PackPromoInfoSO_LevelPromoInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _promoBannerInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PromoBannerInfoSO,
    >,
}
#[cfg(feature = "PackPromoInfoSO+LevelPromoInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo
    => ""."PackPromoInfoSO/LevelPromoInfo"
);
#[cfg(feature = "PackPromoInfoSO+LevelPromoInfo")]
impl std::ops::Deref for crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackPromoInfoSO+LevelPromoInfo")]
impl std::ops::DerefMut for crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackPromoInfoSO+LevelPromoInfo")]
impl crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo {
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
    pub fn get_levelID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_levelID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_promoBannerInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PromoBannerInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PromoBannerInfoSO,
        > = __cordl_object.invoke("get_promoBannerInfo", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PackPromoInfoSO+LevelPromoInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
