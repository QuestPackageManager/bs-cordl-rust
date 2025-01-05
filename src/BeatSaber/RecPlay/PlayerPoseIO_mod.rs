#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerPoseIO {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::PlayerPoseIO =>
    "BeatSaber.RecPlay"."PlayerPoseIO"
);
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
impl std::ops::Deref for crate::BeatSaber::RecPlay::PlayerPoseIO {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
impl std::ops::DerefMut for crate::BeatSaber::RecPlay::PlayerPoseIO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
impl crate::BeatSaber::RecPlay::PlayerPoseIO {
    pub fn Clamp(
        s: quest_hook::libil2cpp::ByRefMut<crate::System::ReadOnlySpan_1<char>>,
        start: char,
        end: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clamp", (s, start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeserializeFromText(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerPoseFrames: quest_hook::libil2cpp::ByRefMut<
            crate::BeatSaber::RecPlay::PlayerPoseFrames,
        >,
        log: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeFromText", (text, playerPoseFrames, log))?;
        Ok(__cordl_ret.into())
    }
    pub fn NextToken(
        s: quest_hook::libil2cpp::ByRefMut<crate::System::ReadOnlySpan_1<char>>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NextToken", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeAsText(
        frames: crate::BeatSaber::RecPlay::PlayerPoseFrames,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializeAsText", (frames))?;
        Ok(__cordl_ret.into())
    }
    pub fn _SerializeAsText_g__WriteFrames_0_0(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        frames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::BeatSaber::RecPlay::PoseFrame>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("<SerializeAsText>g__WriteFrames|0_0", (sb, name, frames))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::RecPlay::PlayerPoseIO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
