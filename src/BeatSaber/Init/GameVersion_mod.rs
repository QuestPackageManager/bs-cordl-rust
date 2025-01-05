#[cfg(feature = "BeatSaber+Init+GameVersion")]
#[repr(C)]
#[derive(Debug)]
pub struct GameVersion {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub content: crate::BeatSaber::Init::GameVersion_Content,
    pub metadata: crate::System::Nullable_1<
        crate::BeatSaber::Init::GameVersion_Metadata,
    >,
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::GameVersion => "BeatSaber.Init"
    ."GameVersion"
);
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl std::ops::Deref for crate::BeatSaber::Init::GameVersion {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl std::ops::DerefMut for crate::BeatSaber::Init::GameVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl crate::BeatSaber::Init::GameVersion {
    pub const kNumericPattern: &'static str = "0|[1-9]\\d*";
    pub const kPascalCasePattern: &'static str = "(?:[A-Z][a-zA-Z]*)?";
    #[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
    pub type BuildInfo = crate::BeatSaber::Init::GameVersion_BuildInfo;
    #[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
    pub type Content = crate::BeatSaber::Init::GameVersion_Content;
    #[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
    pub type Metadata = crate::BeatSaber::Init::GameVersion_Metadata;
    #[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
    pub type PreReleaseLabel = crate::BeatSaber::Init::GameVersion_PreReleaseLabel;
    #[cfg(feature = "BeatSaber+Init+GameVersion+PreReleasePrefix")]
    pub type PreReleasePrefix = crate::BeatSaber::Init::GameVersion_PreReleasePrefix;
    pub fn CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        content: crate::BeatSaber::Init::GameVersion_Content,
        metadata: crate::System::Nullable_1<crate::BeatSaber::Init::GameVersion_Metadata>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (content, metadata))?;
        Ok(__cordl_object.into())
    }
    pub fn Parse(
        version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Init::GameVersion,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Parse", (version))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePlatformId(
        &mut self,
        newPlatformId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Init::GameVersion,
        > = __cordl_object.invoke("UpdatePlatformId", (newPlatformId))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        content: crate::BeatSaber::Init::GameVersion_Content,
        metadata: crate::System::Nullable_1<crate::BeatSaber::Init::GameVersion_Metadata>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (content, metadata))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReleasable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReleasable", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Init::GameVersion {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl AsRef<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    >,
> for crate::BeatSaber::Init::GameVersion {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl AsMut<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    >,
> for crate::BeatSaber::Init::GameVersion {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl AsRef<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    >,
> for crate::BeatSaber::Init::GameVersion {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl AsMut<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    >,
> for crate::BeatSaber::Init::GameVersion {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GameVersion_BuildInfo {
    pub buildId: u64,
    pub platform: crate::BeatSaber::Init::RuntimePlatformType,
    pub platformId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub platformIdIntValue: i32,
}
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::GameVersion_BuildInfo =>
    "BeatSaber.Init"."GameVersion/BuildInfo"
);
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
impl crate::BeatSaber::Init::GameVersion_BuildInfo {
    pub const kBuildIdGroupName: &'static str = "buildid";
    pub const kBuildMetadataGroupName: &'static str = "buildmetadata";
    pub const kPlatformGroupName: &'static str = "platform";
    pub const kPlatformIdGroupName: &'static str = "platformid";
    pub fn CompareTo(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_BuildInfo,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_BuildInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractNumberFromPlatformId(
        platform: crate::BeatSaber::Init::RuntimePlatformType,
        inputPlatformId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractNumberFromPlatformId", (platform, inputPlatformId))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatPlatformId(
        platform: crate::BeatSaber::Init::RuntimePlatformType,
        inputPlatformId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FormatPlatformId", (platform, inputPlatformId))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        buildMetadataValue: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Init::GameVersion_BuildInfo> {
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_BuildInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (buildMetadataValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        platform: crate::BeatSaber::Init::RuntimePlatformType,
        platformId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buildId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (platform, platformId, buildId),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
impl AsRef<crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_BuildInfo>>
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_BuildInfo> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
impl AsMut<crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_BuildInfo>>
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        crate::BeatSaber::Init::GameVersion_BuildInfo,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
impl AsRef<crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_BuildInfo>>
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_BuildInfo> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
impl AsMut<crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_BuildInfo>>
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::BeatSaber::Init::GameVersion_BuildInfo,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GameVersion_Content {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::GameVersion_Content =>
    "BeatSaber.Init"."GameVersion/Content"
);
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Init::GameVersion_Content {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
impl crate::BeatSaber::Init::GameVersion_Content {
    pub const kMajorGroupName: &'static str = "major";
    pub const kMinorGroupName: &'static str = "minor";
    pub const kPatchGroupName: &'static str = "patch";
    pub const kVersionCorePattern: &'static str = "^(?<major>0|[1-9]\\d*)\\.(?<minor>0|[1-9]\\d*)\\.(?<patch>0|[1-9]\\d*)$";
    pub const kVersionCorePatternOnStartOfString: &'static str = "^(?<major>0|[1-9]\\d*)\\.(?<minor>0|[1-9]\\d*)\\.(?<patch>0|[1-9]\\d*)";
    pub fn CompareTo(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_Content,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_Content,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseOnStartOf(
        versionCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Init::GameVersion_Content> {
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_Content = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseOnStartOf", (versionCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Il2CppString0(
        versionCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Init::GameVersion_Content> {
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_Content = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (versionCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Il2CppString1(
        versionCore: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Init::GameVersion_Content> {
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_Content = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (versionCore, pattern))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFromApplicationVersion() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Init::GameVersion_Content,
    > {
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_Content = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadFromApplicationVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        major: u32,
        minor: u32,
        patch: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (major, minor, patch),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
impl AsRef<crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_Content>>
for crate::BeatSaber::Init::GameVersion_Content {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_Content> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
impl AsMut<crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_Content>>
for crate::BeatSaber::Init::GameVersion_Content {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_Content> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
impl AsRef<crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_Content>>
for crate::BeatSaber::Init::GameVersion_Content {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_Content> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
impl AsMut<crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_Content>>
for crate::BeatSaber::Init::GameVersion_Content {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_Content> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GameVersion_Metadata {
    pub preReleaseLabel: crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    pub buildInfo: crate::System::Nullable_1<
        crate::BeatSaber::Init::GameVersion_BuildInfo,
    >,
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::GameVersion_Metadata =>
    "BeatSaber.Init"."GameVersion/Metadata"
);
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Init::GameVersion_Metadata {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
impl crate::BeatSaber::Init::GameVersion_Metadata {
    pub fn BuildInfoText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "BuildInfoText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_Metadata,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_Metadata,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractSubStrings(
        versionMetadata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        preReleaseLabelString: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        buildInfoString: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExtractSubStrings",
                (versionMetadata, preReleaseLabelString, buildInfoString),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        versionMetadata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Init::GameVersion_Metadata> {
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_Metadata = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (versionMetadata))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GameVersion_PreReleaseLabel_Nullable_1_1(
        &mut self,
        preReleaseLabel: crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
        buildInfo: crate::System::Nullable_1<
            crate::BeatSaber::Init::GameVersion_BuildInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (preReleaseLabel, buildInfo),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Nullable_1_0(
        &mut self,
        buildInfo: crate::System::Nullable_1<
            crate::BeatSaber::Init::GameVersion_BuildInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (buildInfo),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
impl AsRef<crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_Metadata>>
for crate::BeatSaber::Init::GameVersion_Metadata {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_Metadata> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
impl AsMut<crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_Metadata>>
for crate::BeatSaber::Init::GameVersion_Metadata {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        crate::BeatSaber::Init::GameVersion_Metadata,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
impl AsRef<crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_Metadata>>
for crate::BeatSaber::Init::GameVersion_Metadata {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_Metadata> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
impl AsMut<crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_Metadata>>
for crate::BeatSaber::Init::GameVersion_Metadata {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_Metadata> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GameVersion_PreReleaseLabel {
    pub prefix: crate::BeatSaber::Init::GameVersion_PreReleasePrefix,
    pub label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::GameVersion_PreReleaseLabel =>
    "BeatSaber.Init"."GameVersion/PreReleaseLabel"
);
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
impl crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    pub const kPrereleaseLabelGroupName: &'static str = "prereleaselabel";
    pub const kPrereleasePrefixGroupName: &'static str = "prereleaseprefix";
    pub fn CompareTo(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn FirstLetterToUpper(
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FirstLetterToUpper", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrefixText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetPrefixText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        releaseLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    > {
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_PreReleaseLabel = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (releaseLabel))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        prefix: crate::BeatSaber::Init::GameVersion_PreReleasePrefix,
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (prefix, label),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
impl AsRef<
    crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_PreReleaseLabel>,
> for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
impl AsMut<
    crate::System::IComparable_1<crate::BeatSaber::Init::GameVersion_PreReleaseLabel>,
> for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
impl AsRef<
    crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_PreReleaseLabel>,
> for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
impl AsMut<
    crate::System::IEquatable_1<crate::BeatSaber::Init::GameVersion_PreReleaseLabel>,
> for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleasePrefix")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameVersion_PreReleasePrefix {
    #[default]
    Alpha = 0i32,
    Beta = 10i32,
    Nightly = 20i32,
    ReleaseCandidate = 30i32,
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleasePrefix")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::GameVersion_PreReleasePrefix =>
    "BeatSaber.Init"."GameVersion/PreReleasePrefix"
);
