#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySongPacksOverride_LocalizedCustomPack {
    __cordl_parent: crate::System::Object,
    pub serializedName: *mut crate::System::String,
    pub order: i32,
    pub localizedNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPackName,
    >,
    pub packIds: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPack => ""
    ."QuickPlaySetupData/QuickPlaySongPacksOverride/LocalizedCustomPack"
);
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack")]
impl std::ops::Deref
for crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPack {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack")]
impl std::ops::DerefMut
for crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack")]
impl crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPack {
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
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySongPacksOverride_LocalizedCustomPackName {
    __cordl_parent: crate::System::Object,
    pub language: *mut crate::System::String,
    pub packName: *mut crate::System::String,
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPackName => ""
    ."QuickPlaySetupData/QuickPlaySongPacksOverride/LocalizedCustomPackName"
);
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName")]
impl std::ops::Deref
for crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPackName {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName")]
impl std::ops::DerefMut
for crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPackName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName")]
impl crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPackName {
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
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPackName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySongPacksOverride_PredefinedPack {
    __cordl_parent: crate::System::Object,
    pub order: i32,
    pub packId: *mut crate::System::String,
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::QuickPlaySongPacksOverride_PredefinedPack => ""
    ."QuickPlaySetupData/QuickPlaySongPacksOverride/PredefinedPack"
);
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack")]
impl std::ops::Deref
for crate::GlobalNamespace::QuickPlaySongPacksOverride_PredefinedPack {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack")]
impl std::ops::DerefMut
for crate::GlobalNamespace::QuickPlaySongPacksOverride_PredefinedPack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack")]
impl crate::GlobalNamespace::QuickPlaySongPacksOverride_PredefinedPack {
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
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySongPacksOverride_PredefinedPack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "QuickPlaySetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySetupData {
    __cordl_parent: crate::System::Object,
    pub quickPlayAvailablePacksOverride: *mut crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride,
}
#[cfg(feature = "QuickPlaySetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::QuickPlaySetupData => ""
    ."QuickPlaySetupData"
);
#[cfg(feature = "QuickPlaySetupData")]
impl std::ops::Deref for crate::GlobalNamespace::QuickPlaySetupData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::QuickPlaySetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData")]
impl crate::GlobalNamespace::QuickPlaySetupData {
    #[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride")]
    pub type QuickPlaySongPacksOverride = crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride;
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
    pub fn get_hasOverride(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasOverride", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "QuickPlaySetupData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::QuickPlaySetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySetupData_QuickPlaySongPacksOverride {
    __cordl_parent: crate::System::Object,
    pub predefinedPackIds: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::QuickPlaySongPacksOverride_PredefinedPack,
    >,
    pub localizedCustomPacks: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPack,
    >,
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride => ""
    ."QuickPlaySetupData/QuickPlaySongPacksOverride"
);
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride")]
impl std::ops::Deref
for crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride")]
impl std::ops::DerefMut
for crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride")]
impl crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride {
    #[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack")]
    pub type LocalizedCustomPack = crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPack;
    #[cfg(
        feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName"
    )]
    pub type LocalizedCustomPackName = crate::GlobalNamespace::QuickPlaySongPacksOverride_LocalizedCustomPackName;
    #[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack")]
    pub type PredefinedPack = crate::GlobalNamespace::QuickPlaySongPacksOverride_PredefinedPack;
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
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
