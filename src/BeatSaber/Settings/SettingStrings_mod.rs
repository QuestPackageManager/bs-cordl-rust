#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingStrings {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::SettingStrings =>
    "BeatSaber.Settings"."SettingStrings"
);
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
impl std::ops::Deref for crate::BeatSaber::Settings::SettingStrings {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::SettingStrings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
impl crate::BeatSaber::Settings::SettingStrings {
    pub fn AppendProperty<T>(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendProperty", (sb, name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decode(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        log: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decode", (settings, text, log))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        stream: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Encode", (settings, stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPureWhiteSpace(
        property: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPureWhiteSpace", (property))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadBool(
        name: crate::System::ReadOnlySpan_1<char>,
        text: crate::System::ReadOnlySpan_1<char>,
        value: quest_hook::libil2cpp::ByRefMut<bool>,
        err: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadBool", (name, text, value, err))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadEnum<T>(
        name: crate::System::ReadOnlySpan_1<char>,
        text: crate::System::ReadOnlySpan_1<char>,
        value: quest_hook::libil2cpp::ByRefMut<T>,
        err: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadEnum", (name, text, value, err))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadFloat(
        name: crate::System::ReadOnlySpan_1<char>,
        text: crate::System::ReadOnlySpan_1<char>,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
        err: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadFloat", (name, text, value, err))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadInt(
        name: crate::System::ReadOnlySpan_1<char>,
        text: crate::System::ReadOnlySpan_1<char>,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
        err: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadInt", (name, text, value, err))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadProperties(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        log: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadProperties", (settings, text, log))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadProperty(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        name: crate::System::ReadOnlySpan_1<char>,
        value: crate::System::ReadOnlySpan_1<char>,
        log: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadProperty", (settings, name, value, log))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadPropertyLine(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        property: crate::System::ReadOnlySpan_1<char>,
        log: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadPropertyLine", (settings, property, log))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadString(
        _cordl__: crate::System::ReadOnlySpan_1<char>,
        text: crate::System::ReadOnlySpan_1<char>,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        err: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadString", (_cordl__, text, value, err))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteProperties(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        text: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteProperties", (settings, text))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Settings::SettingStrings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
