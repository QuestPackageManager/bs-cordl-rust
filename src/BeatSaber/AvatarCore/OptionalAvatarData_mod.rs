#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OptionalAvatarData {
    pub dataType: u32,
    pub length: i32,
    pub data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::OptionalAvatarData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "OptionalAvatarData";
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
for crate::BeatSaber::AvatarCore::OptionalAvatarData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::AvatarCore::OptionalAvatarData {
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
for crate::BeatSaber::AvatarCore::OptionalAvatarData {
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
for crate::BeatSaber::AvatarCore::OptionalAvatarData {
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
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::AvatarCore::OptionalAvatarData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarData")]
impl crate::BeatSaber::AvatarCore::OptionalAvatarData {
    pub const kMaxDataSize: i32 = 2048i32;
    pub fn Equals(
        &mut self,
        other: crate::BeatSaber::AvatarCore::OptionalAvatarData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarData")]
impl AsRef<crate::System::IEquatable_1<crate::BeatSaber::AvatarCore::OptionalAvatarData>>
for crate::BeatSaber::AvatarCore::OptionalAvatarData {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::BeatSaber::AvatarCore::OptionalAvatarData> {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+OptionalAvatarData")]
impl AsMut<crate::System::IEquatable_1<crate::BeatSaber::AvatarCore::OptionalAvatarData>>
for crate::BeatSaber::AvatarCore::OptionalAvatarData {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::BeatSaber::AvatarCore::OptionalAvatarData,
    > {
        todo!()
    }
}
