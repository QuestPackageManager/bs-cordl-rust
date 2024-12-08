#[cfg(feature = "PackPromoInfoSO+LevelPromoInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct PackPromoInfoSO_LevelPromoInfo {
    __cordl_parent: crate::System::Object,
    pub _levelID: *mut crate::System::String,
    pub _promoBannerInfo: *mut PromoBannerInfoSO,
}
#[cfg(feature = "PackPromoInfoSO+LevelPromoInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo
    => ""."PackPromoInfoSO/LevelPromoInfo"
);
#[cfg(feature = "PackPromoInfoSO+LevelPromoInfo")]
impl std::ops::Deref for crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo {
    type Target = crate::System::Object;
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
    pub fn get_levelID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_levelID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_promoBannerInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PromoBannerInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PromoBannerInfoSO = __cordl_object
            .invoke("get_promoBannerInfo", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "PackPromoInfoSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PackPromoInfoSO {
    __cordl_parent: PersistentScriptableObject,
    pub _promoBannerInfo: *mut PromoBannerInfoSO,
    pub _levelsPromoInfo: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo,
    >,
}
#[cfg(feature = "PackPromoInfoSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PackPromoInfoSO => ""."PackPromoInfoSO"
);
#[cfg(feature = "PackPromoInfoSO")]
impl std::ops::Deref for PackPromoInfoSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PackPromoInfoSO")]
impl std::ops::DerefMut for PackPromoInfoSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PackPromoInfoSO")]
impl PackPromoInfoSO {
    #[cfg(feature = "PackPromoInfoSO+LevelPromoInfo")]
    pub type LevelPromoInfo = crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo;
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
    pub fn get_levelPromoInfos(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PackPromoInfoSO_LevelPromoInfo,
        > = __cordl_object.invoke("get_levelPromoInfos", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_promoBannerInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PromoBannerInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PromoBannerInfoSO = __cordl_object
            .invoke("get_promoBannerInfo", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PackPromoInfoSO")]
impl quest_hook::libil2cpp::ObjectType for PackPromoInfoSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
