#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemIdentifier")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AvatarSystemIdentifier {
    pub value: *mut quest_hook::libil2cpp::Il2CppString,
    pub hash: u32,
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemIdentifier")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::AvatarSystemIdentifier =>
    "BeatSaber.AvatarCore"."AvatarSystemIdentifier"
);
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemIdentifier")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::AvatarCore::AvatarSystemIdentifier {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemIdentifier")]
impl crate::BeatSaber::AvatarCore::AvatarSystemIdentifier {
    pub fn Equals_AvatarSystemIdentifier0(
        &mut self,
        other: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
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
    pub fn HashAvatarSystemTypeMultiplier(
        avatarSystemTypeIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HashAvatarSystemTypeMultiplier", (avatarSystemTypeIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        avatarSystemTypeIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (avatarSystemTypeIdentifier),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        obj1: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
        obj2: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (obj1, obj2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        obj1: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
        obj2: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (obj1, obj2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemIdentifier")]
impl AsRef<
    crate::System::IEquatable_1<crate::BeatSaber::AvatarCore::AvatarSystemIdentifier>,
> for crate::BeatSaber::AvatarCore::AvatarSystemIdentifier {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
    > {
        todo!()
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystemIdentifier")]
impl AsMut<
    crate::System::IEquatable_1<crate::BeatSaber::AvatarCore::AvatarSystemIdentifier>,
> for crate::BeatSaber::AvatarCore::AvatarSystemIdentifier {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
    > {
        todo!()
    }
}
