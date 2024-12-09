#[cfg(feature = "BeatmapLevelsPromoDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsPromoDataSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _promo: *mut crate::GlobalNamespace::BeatmapLevelsPromoDataSO_BeatmapLevelsPromo,
}
#[cfg(feature = "BeatmapLevelsPromoDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelsPromoDataSO => ""
    ."BeatmapLevelsPromoDataSO"
);
#[cfg(feature = "BeatmapLevelsPromoDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelsPromoDataSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsPromoDataSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelsPromoDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsPromoDataSO")]
impl crate::GlobalNamespace::BeatmapLevelsPromoDataSO {
    #[cfg(feature = "BeatmapLevelsPromoDataSO+BeatmapLevelsPromo")]
    pub type BeatmapLevelsPromo = crate::GlobalNamespace::BeatmapLevelsPromoDataSO_BeatmapLevelsPromo;
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
    pub fn get_promo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapLevelsPromoDataSO_BeatmapLevelsPromo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapLevelsPromoDataSO_BeatmapLevelsPromo = __cordl_object
            .invoke("get_promo", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_promo(
        &mut self,
        value: *mut crate::GlobalNamespace::BeatmapLevelsPromoDataSO_BeatmapLevelsPromo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_promo", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelsPromoDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelsPromoDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelsPromoDataSO+BeatmapLevelsPromo")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsPromoDataSO_BeatmapLevelsPromo {
    __cordl_parent: crate::System::Object,
    pub _promotedBeatmapLevelPacks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub _updatedBeatmapLevelPacks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub _promotedBeatmapLevels: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub _updatedBeatmapLevels: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "BeatmapLevelsPromoDataSO+BeatmapLevelsPromo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapLevelsPromoDataSO_BeatmapLevelsPromo => ""
    ."BeatmapLevelsPromoDataSO/BeatmapLevelsPromo"
);
#[cfg(feature = "BeatmapLevelsPromoDataSO+BeatmapLevelsPromo")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapLevelsPromoDataSO_BeatmapLevelsPromo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsPromoDataSO+BeatmapLevelsPromo")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapLevelsPromoDataSO_BeatmapLevelsPromo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsPromoDataSO+BeatmapLevelsPromo")]
impl crate::GlobalNamespace::BeatmapLevelsPromoDataSO_BeatmapLevelsPromo {
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
    pub fn get_promotedBeatmapLevelPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_promotedBeatmapLevelPacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_promotedBeatmapLevels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_promotedBeatmapLevels", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_updatedBeatmapLevelPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_updatedBeatmapLevelPacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_updatedBeatmapLevels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_updatedBeatmapLevels", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelsPromoDataSO+BeatmapLevelsPromo")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelsPromoDataSO_BeatmapLevelsPromo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
