#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingStrings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::Settings::SettingStrings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.Settings";
    const CLASS_NAME: &'static str = "SettingStrings";
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
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
impl std::ops::Deref for crate::BeatSaber::Settings::SettingStrings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    T,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("AppendProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AppendProperty", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (sb, name, value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Decode(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        log: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::BeatSaber::Settings::Settings,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                bool,
                3usize,
            >("Decode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Decode", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (settings, text, log))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        stream: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::BeatSaber::Settings::Settings,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Encode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Encode", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (settings, stream))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsPureWhiteSpace(
        property: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::ReadOnlySpan_1<char>),
                bool,
                1usize,
            >("IsPureWhiteSpace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsPureWhiteSpace", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (property)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                bool,
                4usize,
            >("ReadBool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadBool", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (name, text, value, err))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::ByRefMut<T>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                bool,
                4usize,
            >("ReadEnum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadEnum", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (name, text, value, err))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                bool,
                4usize,
            >("ReadFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadFloat", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (name, text, value, err))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                bool,
                4usize,
            >("ReadInt")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadInt", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (name, text, value, err))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadProperties(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        log: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::BeatSaber::Settings::Settings,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                bool,
                3usize,
            >("ReadProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadProperties", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (settings, text, log))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::BeatSaber::Settings::Settings,
                    >,
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                bool,
                4usize,
            >("ReadProperty")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadProperty", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (settings, name, value, log))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadPropertyLine(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        property: crate::System::ReadOnlySpan_1<char>,
        log: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::BeatSaber::Settings::Settings,
                    >,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                bool,
                3usize,
            >("ReadPropertyLine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadPropertyLine", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (settings, property, log))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                ),
                bool,
                4usize,
            >("ReadString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadString", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (_cordl__, text, value, err))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteProperties(
        settings: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::Settings::Settings>,
        text: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::BeatSaber::Settings::Settings,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteProperties")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteProperties", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (settings, text))
        };
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
