#[cfg(feature = "SongPackMasksModel")]
#[repr(C)]
#[derive(Debug)]
pub struct SongPackMasksModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub content: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SongPackMasksModelSO>,
    pub allSongPackMask: crate::GlobalNamespace::SongPackMask,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub _songPackSerializedNameToMaskInfoDict: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            crate::GlobalNamespace::SongPackMasksModel_MaskInfo,
        >,
    >,
    pub _songPackMaskToSerializedNameDict: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::GlobalNamespace::SongPackMask,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
}
#[cfg(feature = "SongPackMasksModel")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::SongPackMasksModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SongPackMasksModel";
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
#[cfg(feature = "SongPackMasksModel")]
impl std::ops::Deref for crate::GlobalNamespace::SongPackMasksModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongPackMasksModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongPackMasksModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongPackMasksModel")]
impl crate::GlobalNamespace::SongPackMasksModel {
    #[cfg(feature = "SongPackMasksModel+LocalizedText")]
    pub type LocalizedText = crate::GlobalNamespace::SongPackMasksModel_LocalizedText;
    #[cfg(feature = "SongPackMasksModel+MaskInfo")]
    pub type MaskInfo = crate::GlobalNamespace::SongPackMasksModel_MaskInfo;
    pub fn GetSongPackMaskText(
        &mut self,
        songPackMask: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::SongPackMasksModel_LocalizedText,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SongPackMasksModel_LocalizedText = __cordl_object
            .invoke("GetSongPackMaskText", (songPackMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        songPackMasks: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SongPackMasksModelSO,
        >,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (songPackMasks, beatmapLevelsModel))?;
        Ok(__cordl_object.into())
    }
    pub fn ToLocalizedName(
        &mut self,
        serializedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::SongPackMasksModel_LocalizedText,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SongPackMasksModel_LocalizedText = __cordl_object
            .invoke("ToLocalizedName", (serializedName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSerializedName(
        &mut self,
        songPackMask: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToSerializedName", (songPackMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSongPackMask(
        &mut self,
        serializedName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SongPackMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SongPackMask = __cordl_object
            .invoke("ToSongPackMask", (serializedName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        songPackMasks: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::SongPackMasksModelSO,
        >,
        beatmapLevelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (songPackMasks, beatmapLevelsModel))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SongPackMasksModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SongPackMasksModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SongPackMasksModel+LocalizedText")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SongPackMasksModel_LocalizedText {
    pub textKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub isPlural: bool,
}
#[cfg(feature = "SongPackMasksModel+LocalizedText")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SongPackMasksModel_LocalizedText {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SongPackMasksModel/LocalizedText";
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
#[cfg(feature = "SongPackMasksModel+LocalizedText")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::SongPackMasksModel_LocalizedText {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "SongPackMasksModel+LocalizedText")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::SongPackMasksModel_LocalizedText {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "SongPackMasksModel+LocalizedText")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::SongPackMasksModel_LocalizedText {
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
#[cfg(feature = "SongPackMasksModel+LocalizedText")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::SongPackMasksModel_LocalizedText {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "SongPackMasksModel+LocalizedText")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::SongPackMasksModel_LocalizedText {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SongPackMasksModel+LocalizedText")]
impl crate::GlobalNamespace::SongPackMasksModel_LocalizedText {
    pub fn _ctor(
        &mut self,
        textKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isPlural: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (textKey, isPlural),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_text(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_text", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SongPackMasksModel+MaskInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SongPackMasksModel_MaskInfo {
    pub localizedText: crate::GlobalNamespace::SongPackMasksModel_LocalizedText,
    pub mask: crate::GlobalNamespace::SongPackMask,
}
#[cfg(feature = "SongPackMasksModel+MaskInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::SongPackMasksModel_MaskInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "SongPackMasksModel/MaskInfo";
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
#[cfg(feature = "SongPackMasksModel+MaskInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::SongPackMasksModel_MaskInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "SongPackMasksModel+MaskInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::SongPackMasksModel_MaskInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "SongPackMasksModel+MaskInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::SongPackMasksModel_MaskInfo {
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
#[cfg(feature = "SongPackMasksModel+MaskInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::SongPackMasksModel_MaskInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "SongPackMasksModel+MaskInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::SongPackMasksModel_MaskInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SongPackMasksModel+MaskInfo")]
impl crate::GlobalNamespace::SongPackMasksModel_MaskInfo {
    pub fn _ctor(
        &mut self,
        localizedText: crate::GlobalNamespace::SongPackMasksModel_LocalizedText,
        mask: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (localizedText, mask),
        )?;
        Ok(__cordl_ret.into())
    }
}
