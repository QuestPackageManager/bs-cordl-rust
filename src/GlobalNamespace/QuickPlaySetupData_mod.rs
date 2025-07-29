#[cfg(feature = "cordl_class_QuickPlaySetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySetupData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub quickPlayAvailablePacksOverride: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride,
    >,
}
#[cfg(feature = "cordl_class_QuickPlaySetupData")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::QuickPlaySetupData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "QuickPlaySetupData";
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
#[cfg(feature = "cordl_class_QuickPlaySetupData")]
impl std::ops::Deref for crate::GlobalNamespace::QuickPlaySetupData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_QuickPlaySetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::QuickPlaySetupData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData")]
impl crate::GlobalNamespace::QuickPlaySetupData {
    #[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride")]
    pub type QuickPlaySongPacksOverride = crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride;
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
    pub fn get_hasOverride(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_hasOverride")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_hasOverride", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_QuickPlaySetupData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::QuickPlaySetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySetupData_QuickPlaySongPacksOverride {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub predefinedPackIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_PredefinedPack,
            >,
        >,
    >,
    pub localizedCustomPacks: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPack,
            >,
        >,
    >,
}
#[cfg(feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "QuickPlaySetupData/QuickPlaySongPacksOverride";
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
#[cfg(feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride")]
impl std::ops::Deref
for crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride")]
impl std::ops::DerefMut
for crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride")]
impl crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride {
    #[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack")]
    pub type LocalizedCustomPack = crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPack;
    #[cfg(
        feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName"
    )]
    pub type LocalizedCustomPackName = crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPackName;
    #[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack")]
    pub type PredefinedPack = crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_PredefinedPack;
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
}
#[cfg(feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySetupData_QuickPlaySongPacksOverride {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack"
)]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPack {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub serializedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub order: i32,
    pub localizedNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPackName,
            >,
        >,
    >,
    pub packIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPack {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "QuickPlaySetupData/QuickPlaySongPacksOverride/LocalizedCustomPack";
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
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack"
)]
impl std::ops::Deref
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPack {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPack {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack")]
impl crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPack {
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
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPack"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName"
)]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPackName {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub language: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub packName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPackName {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "QuickPlaySetupData/QuickPlaySongPacksOverride/LocalizedCustomPackName";
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
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName"
)]
impl std::ops::Deref
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPackName {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPackName {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName")]
impl crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPackName {
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
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+LocalizedCustomPackName"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_LocalizedCustomPackName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack"
)]
#[repr(C)]
#[derive(Debug)]
pub struct QuickPlaySongPacksOverride_QuickPlaySetupData_PredefinedPack {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub order: i32,
    pub packId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_PredefinedPack {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "QuickPlaySetupData/QuickPlaySongPacksOverride/PredefinedPack";
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
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack"
)]
impl std::ops::Deref
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_PredefinedPack {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_PredefinedPack {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack")]
impl crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_PredefinedPack {
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
}
#[cfg(
    feature = "cordl_class_QuickPlaySetupData+QuickPlaySongPacksOverride+PredefinedPack"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::QuickPlaySongPacksOverride_QuickPlaySetupData_PredefinedPack {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
