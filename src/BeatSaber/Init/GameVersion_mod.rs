#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion")]
#[repr(C)]
#[derive(Debug)]
pub struct GameVersion {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub content: crate::BeatSaber::Init::GameVersion_Content,
    pub metadata: crate::System::Nullable_1<
        crate::BeatSaber::Init::GameVersion_Metadata,
    >,
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Init::GameVersion {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Init";
    const CLASS_NAME: &'static str = "GameVersion";
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
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl std::ops::Deref for crate::BeatSaber::Init::GameVersion {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl std::ops::DerefMut for crate::BeatSaber::Init::GameVersion {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>),
                        i32,
                        1usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
                        1usize,
                    >("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Parse",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Init::GameVersion,
        > = unsafe { cordl_method_info.invoke_unchecked((), (version))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePlatformId(
        &mut self,
        newPlatformId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
                        1usize,
                    >("UpdatePlatformId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdatePlatformId", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::Init::GameVersion,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (newPlatformId))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        content: crate::BeatSaber::Init::GameVersion_Content,
        metadata: crate::System::Nullable_1<crate::BeatSaber::Init::GameVersion_Metadata>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::BeatSaber::Init::GameVersion_Content,
                            crate::System::Nullable_1<
                                crate::BeatSaber::Init::GameVersion_Metadata,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (content, metadata))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReleasable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsReleasable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsReleasable", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion")]
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
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    >,
> for crate::BeatSaber::Init::GameVersion {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    >,
> for crate::BeatSaber::Init::GameVersion {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    >,
> for crate::BeatSaber::Init::GameVersion {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    >,
> for crate::BeatSaber::Init::GameVersion {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+BuildInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameVersion_BuildInfo {
    pub buildId: u64,
    pub platform: crate::BeatSaber::Init::RuntimePlatformType,
    pub platformId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub platformIdIntValue: i32,
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+BuildInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Init";
    const CLASS_NAME: &'static str = "GameVersion/BuildInfo";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+BuildInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+BuildInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+BuildInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+BuildInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+BuildInfo")]
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BeatSaber::Init::GameVersion_BuildInfo),
                        i32,
                        1usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_BuildInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BeatSaber::Init::GameVersion_BuildInfo),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractNumberFromPlatformId(
        platform: crate::BeatSaber::Init::RuntimePlatformType,
        inputPlatformId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::BeatSaber::Init::RuntimePlatformType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        2usize,
                    >("ExtractNumberFromPlatformId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractNumberFromPlatformId", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (platform, inputPlatformId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FormatPlatformId(
        platform: crate::BeatSaber::Init::RuntimePlatformType,
        inputPlatformId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::BeatSaber::Init::RuntimePlatformType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        2usize,
                    >("FormatPlatformId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FormatPlatformId", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (platform, inputPlatformId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        buildMetadataValue: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Init::GameVersion_BuildInfo> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::BeatSaber::Init::GameVersion_BuildInfo,
                        1usize,
                    >("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Parse",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_BuildInfo = unsafe {
            cordl_method_info.invoke_unchecked((), (buildMetadataValue))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        platform: crate::BeatSaber::Init::RuntimePlatformType,
        platformId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buildId: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::BeatSaber::Init::RuntimePlatformType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            u64,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (platform, platformId, buildId))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_BuildInfo>>
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_BuildInfo> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_BuildInfo>>
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_BuildInfo> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_BuildInfo>>
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_BuildInfo> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+BuildInfo")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_BuildInfo>>
for crate::BeatSaber::Init::GameVersion_BuildInfo {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_BuildInfo> {
        todo!()
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Content")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameVersion_Content {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Content")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Init::GameVersion_Content {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Init";
    const CLASS_NAME: &'static str = "GameVersion/Content";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Content")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::Init::GameVersion_Content {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Content")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::Init::GameVersion_Content {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Content")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::Init::GameVersion_Content {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Content")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::Init::GameVersion_Content {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Content")]
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BeatSaber::Init::GameVersion_Content),
                        i32,
                        1usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_Content,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BeatSaber::Init::GameVersion_Content),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ParseOnStartOf(
        versionCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Init::GameVersion_Content> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::BeatSaber::Init::GameVersion_Content,
                        1usize,
                    >("ParseOnStartOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ParseOnStartOf", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_Content = unsafe {
            cordl_method_info.invoke_unchecked((), (versionCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Il2CppString0(
        versionCode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Init::GameVersion_Content> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::BeatSaber::Init::GameVersion_Content,
                        1usize,
                    >("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Parse",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_Content = unsafe {
            cordl_method_info.invoke_unchecked((), (versionCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Il2CppString1(
        versionCore: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Init::GameVersion_Content> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::BeatSaber::Init::GameVersion_Content,
                        2usize,
                    >("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Parse",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_Content = unsafe {
            cordl_method_info.invoke_unchecked((), (versionCore, pattern))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFromApplicationVersion() -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Init::GameVersion_Content,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::BeatSaber::Init::GameVersion_Content,
                        0usize,
                    >("ReadFromApplicationVersion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadFromApplicationVersion", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_Content = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        major: u32,
        minor: u32,
        patch: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32, u32, u32),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (major, minor, patch))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Content>>
for crate::BeatSaber::Init::GameVersion_Content {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Content> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Content>>
for crate::BeatSaber::Init::GameVersion_Content {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Content> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Content>>
for crate::BeatSaber::Init::GameVersion_Content {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Content> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Content")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Content>>
for crate::BeatSaber::Init::GameVersion_Content {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Content> {
        todo!()
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Metadata")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameVersion_Metadata {
    pub preReleaseLabel: crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    pub buildInfo: crate::System::Nullable_1<
        crate::BeatSaber::Init::GameVersion_BuildInfo,
    >,
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Metadata")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Init::GameVersion_Metadata {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Init";
    const CLASS_NAME: &'static str = "GameVersion/Metadata";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Metadata")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::Init::GameVersion_Metadata {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Metadata")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::Init::GameVersion_Metadata {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Metadata")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::Init::GameVersion_Metadata {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Metadata")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::Init::GameVersion_Metadata {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+Metadata")]
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("BuildInfoText")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildInfoText", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_Metadata,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BeatSaber::Init::GameVersion_Metadata),
                        i32,
                        1usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_Metadata,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BeatSaber::Init::GameVersion_Metadata),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ExtractSubStrings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExtractSubStrings", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (versionMetadata, preReleaseLabelString, buildInfoString),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        versionMetadata: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::Init::GameVersion_Metadata> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::BeatSaber::Init::GameVersion_Metadata,
                        1usize,
                    >("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Parse",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_Metadata = unsafe {
            cordl_method_info.invoke_unchecked((), (versionMetadata))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_GameVersion_PreReleaseLabel_Nullable_1_1(
        &mut self,
        preReleaseLabel: crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
        buildInfo: crate::System::Nullable_1<
            crate::BeatSaber::Init::GameVersion_BuildInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
                            crate::System::Nullable_1<
                                crate::BeatSaber::Init::GameVersion_BuildInfo,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (preReleaseLabel, buildInfo))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Nullable_1_0(
        &mut self,
        buildInfo: crate::System::Nullable_1<
            crate::BeatSaber::Init::GameVersion_BuildInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Nullable_1<
                            crate::BeatSaber::Init::GameVersion_BuildInfo,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (buildInfo))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Metadata>>
for crate::BeatSaber::Init::GameVersion_Metadata {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Metadata> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Metadata>>
for crate::BeatSaber::Init::GameVersion_Metadata {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Metadata> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Metadata>>
for crate::BeatSaber::Init::GameVersion_Metadata {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Metadata> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+Metadata")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Metadata>>
for crate::BeatSaber::Init::GameVersion_Metadata {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_Metadata> {
        todo!()
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleaseLabel")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameVersion_PreReleaseLabel {
    pub prefix: crate::BeatSaber::Init::GameVersion_PreReleasePrefix,
    pub label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleaseLabel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Init";
    const CLASS_NAME: &'static str = "GameVersion/PreReleaseLabel";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleaseLabel")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleaseLabel")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleaseLabel")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleaseLabel")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleaseLabel")]
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BeatSaber::Init::GameVersion_PreReleaseLabel),
                        i32,
                        1usize,
                    >("CompareTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareTo", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::BeatSaber::Init::GameVersion_PreReleaseLabel),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FirstLetterToUpper(
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("FirstLetterToUpper")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FirstLetterToUpper", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked((), (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPrefixText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("GetPrefixText")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPrefixText", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Parse(
        releaseLabel: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
                        1usize,
                    >("Parse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Parse",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::BeatSaber::Init::GameVersion_PreReleaseLabel = unsafe {
            cordl_method_info.invoke_unchecked((), (releaseLabel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        prefix: crate::BeatSaber::Init::GameVersion_PreReleasePrefix,
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::BeatSaber::Init::GameVersion_PreReleasePrefix,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (prefix, label))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_PreReleaseLabel>,
> for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_PreReleaseLabel>,
> for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_PreReleaseLabel>,
> for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+Init+GameVersion+PreReleaseLabel")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::GameVersion_PreReleaseLabel>,
> for crate::BeatSaber::Init::GameVersion_PreReleaseLabel {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::BeatSaber::Init::GameVersion_PreReleaseLabel,
    > {
        todo!()
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleasePrefix")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameVersion_PreReleasePrefix {
    #[default]
    Alpha = 0i32,
    Beta = 10i32,
    Nightly = 20i32,
    ReleaseCandidate = 30i32,
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleasePrefix")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::Init::GameVersion_PreReleasePrefix {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.Init";
    const CLASS_NAME: &'static str = "GameVersion/PreReleasePrefix";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleasePrefix")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::Init::GameVersion_PreReleasePrefix {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleasePrefix")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::Init::GameVersion_PreReleasePrefix {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleasePrefix")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::Init::GameVersion_PreReleasePrefix {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_BeatSaber+Init+GameVersion+PreReleasePrefix")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::Init::GameVersion_PreReleasePrefix {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
