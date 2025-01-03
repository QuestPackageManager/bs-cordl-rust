#[cfg(feature = "SonyLevelProductPackSourceSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductPackSourceSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _levelProductPackSource: *mut crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductPackSource,
    pub _levelProductRedirectionSources: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductRedirectionSource,
    >,
}
#[cfg(feature = "SonyLevelProductPackSourceSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SonyLevelProductPackSourceSO =>
    ""."SonyLevelProductPackSourceSO"
);
#[cfg(feature = "SonyLevelProductPackSourceSO")]
impl std::ops::Deref for crate::GlobalNamespace::SonyLevelProductPackSourceSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::SonyLevelProductPackSourceSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO")]
impl crate::GlobalNamespace::SonyLevelProductPackSourceSO {
    #[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductPackSource")]
    pub type LevelProductPackSource = crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductPackSource;
    #[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductRedirectionSource")]
    pub type LevelProductRedirectionSource = crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductRedirectionSource;
    #[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductSource")]
    pub type LevelProductSource = crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductSource;
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
    pub fn get_levelProductPackSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductPackSource,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductPackSource,
        > = __cordl_object.invoke("get_levelProductPackSource", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_redirectionSources(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductRedirectionSource,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductRedirectionSource,
            >,
        > = __cordl_object.invoke("get_redirectionSources", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_levelProductPackSource(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductPackSource,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_levelProductPackSource", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelProductPackSourceSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductPackSource")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductPackSourceSO_LevelProductPackSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _packIndex: i32,
    pub _levelPack: *mut crate::GlobalNamespace::BeatmapLevelPackSO,
    pub _levelProductsData: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductSource,
    >,
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductPackSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductPackSource => ""
    ."SonyLevelProductPackSourceSO/LevelProductPackSource"
);
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductPackSource")]
impl std::ops::Deref
for crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductPackSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductPackSource")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductPackSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductPackSource")]
impl crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductPackSource {
    pub fn New(
        levelPack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPackSO>,
        packIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelPack, packIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        levelPack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPackSO>,
        packIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelPack, packIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelPack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPackSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPackSO,
        > = __cordl_object.invoke("get_levelPack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelProductsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductSource,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductSource,
            >,
        > = __cordl_object.invoke("get_levelProductsData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_packIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_levelProductsData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductSource,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_levelProductsData", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductPackSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductPackSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductRedirectionSource")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductPackSourceSO_LevelProductRedirectionSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _sourceLevelPackId: *mut quest_hook::libil2cpp::Il2CppString,
    pub _redirectedProductLabel: *mut quest_hook::libil2cpp::Il2CppString,
    pub _validUntilDate: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductRedirectionSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductRedirectionSource => ""
    ."SonyLevelProductPackSourceSO/LevelProductRedirectionSource"
);
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductRedirectionSource")]
impl std::ops::Deref
for crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductRedirectionSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductRedirectionSource")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductRedirectionSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductRedirectionSource")]
impl crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductRedirectionSource {
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
    pub fn get_redirectedProductLabel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_redirectedProductLabel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sourceLevelPackId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_sourceLevelPackId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_validUntilDate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_validUntilDate", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductRedirectionSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductRedirectionSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductSource")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLevelProductPackSourceSO_LevelProductSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _levelIndex: i32,
    pub _ps5SieeDcCode: *mut quest_hook::libil2cpp::Il2CppString,
    pub _ps5SieaAcCode: *mut quest_hook::libil2cpp::Il2CppString,
    pub _ps4SieeDcCode: *mut quest_hook::libil2cpp::Il2CppString,
    pub _ps4SieaAcCode: *mut quest_hook::libil2cpp::Il2CppString,
    pub _level: *mut crate::GlobalNamespace::BeatmapLevelSO,
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductSource => ""
    ."SonyLevelProductPackSourceSO/LevelProductSource"
);
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductSource")]
impl std::ops::Deref
for crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductSource")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductSource")]
impl crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductSource {
    pub fn New(
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelSO>,
        levelIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (level, levelIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelSO>,
        levelIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (level, levelIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_level(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelSO,
        > = __cordl_object.invoke("get_level", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_levelIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ps4SieaAcCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ps4SieaAcCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ps4SieeDcCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ps4SieeDcCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ps5SieaAcCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ps5SieaAcCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ps5SieeDcCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ps5SieeDcCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ps4SieaAcCode(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps4SieaAcCode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ps4SieeDcCode(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps4SieeDcCode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ps5SieaAcCode(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps5SieaAcCode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ps5SieeDcCode(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ps5SieeDcCode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SonyLevelProductPackSourceSO+LevelProductSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLevelProductPackSourceSO_LevelProductSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
