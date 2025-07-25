#[cfg(feature = "SteamLevelProductPacksSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SteamLevelProductPacksSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _levelPackProductData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
    >,
    pub _levelPackRedirectionData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
            >,
        >,
    >,
}
#[cfg(feature = "SteamLevelProductPacksSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SteamLevelProductPacksSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SteamLevelProductPacksSO";
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
#[cfg(feature = "SteamLevelProductPacksSO")]
impl std::ops::Deref for crate::GlobalNamespace::SteamLevelProductPacksSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::SteamLevelProductPacksSO {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl crate::GlobalNamespace::SteamLevelProductPacksSO {
    pub fn ILevelPackProductDataContainer_SteamLevelProductCollectionModel_LevelPackProductData_SteamLevelProductCollectionModel_LevelProductData__SetLevelPackProductData(
        &mut self,
        newLevelPackProductData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "ILevelPackProductDataContainer<SteamLevelProductCollectionModel.LevelPackProductData,SteamLevelProductCollectionModel.LevelProductData>.SetLevelPackProductData",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ILevelPackProductDataContainer<SteamLevelProductCollectionModel.LevelPackProductData,SteamLevelProductCollectionModel.LevelProductData>.SetLevelPackProductData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (newLevelPackProductData))?
        };
        Ok(__cordl_ret.into())
    }
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_levelPackProductData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
                        >,
                        0usize,
                    >("get_levelPackProductData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_levelPackProductData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_levelPackRedirectionData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_levelPackRedirectionData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_levelPackRedirectionData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackRedirectionData,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SteamLevelProductPacksSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl AsRef<
    crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    >,
> for crate::GlobalNamespace::SteamLevelProductPacksSO {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SteamLevelProductPacksSO")]
impl AsMut<
    crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    >,
> for crate::GlobalNamespace::SteamLevelProductPacksSO {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ILevelPackProductDataContainer_2<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelPackProductData,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SteamLevelProductCollectionModel_LevelProductData,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
