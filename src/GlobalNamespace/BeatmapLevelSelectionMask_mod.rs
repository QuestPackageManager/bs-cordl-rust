#[cfg(feature = "BeatmapLevelSelectionMask")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeatmapLevelSelectionMask {
    pub difficulties: crate::GlobalNamespace::BeatmapDifficultyMask,
    pub modifiers: crate::GlobalNamespace::GameplayModifierMask,
    pub songPacks: crate::GlobalNamespace::SongPackMask,
}
#[cfg(feature = "BeatmapLevelSelectionMask")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapLevelSelectionMask {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapLevelSelectionMask";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::BeatmapLevelSelectionMask {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::BeatmapLevelSelectionMask {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::BeatmapLevelSelectionMask {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::BeatmapLevelSelectionMask {
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
#[cfg(feature = "BeatmapLevelSelectionMask")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapLevelSelectionMask {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelSelectionMask")]
impl crate::GlobalNamespace::BeatmapLevelSelectionMask {
    pub fn Deserialize(
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        version: u32,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapLevelSelectionMask,
    > {
        let __cordl_ret: crate::GlobalNamespace::BeatmapLevelSelectionMask = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Deserialize", (reader, version))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_BeatmapLevelSelectionMask1(
        &mut self,
        other: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        version: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Serialize",
            (writer, version),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        difficulties: crate::GlobalNamespace::BeatmapDifficultyMask,
        modifiers: crate::GlobalNamespace::GameplayModifierMask,
        songPacks: crate::GlobalNamespace::SongPackMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (difficulties, modifiers, songPacks),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        l: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        r: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (l, r))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        l: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        r: crate::GlobalNamespace::BeatmapLevelSelectionMask,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (l, r))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelSelectionMask")]
impl AsRef<
    crate::System::IEquatable_1<crate::GlobalNamespace::BeatmapLevelSelectionMask>,
> for crate::GlobalNamespace::BeatmapLevelSelectionMask {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::GlobalNamespace::BeatmapLevelSelectionMask,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatmapLevelSelectionMask")]
impl AsMut<
    crate::System::IEquatable_1<crate::GlobalNamespace::BeatmapLevelSelectionMask>,
> for crate::GlobalNamespace::BeatmapLevelSelectionMask {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::GlobalNamespace::BeatmapLevelSelectionMask,
    > {
        todo!()
    }
}
