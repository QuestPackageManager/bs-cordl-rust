#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonFileHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::JsonExtension::JsonFileHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.JsonExtension";
    const CLASS_NAME: &'static str = "JsonFileHandler";
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
#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
impl std::ops::Deref for crate::BGLib::JsonExtension::JsonFileHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
impl std::ops::DerefMut for crate::BGLib::JsonExtension::JsonFileHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
impl crate::BGLib::JsonExtension::JsonFileHandler {
    pub fn ReadFromFile_Il2CppString0<T>(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                T,
                1usize,
            >("ReadFromFile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadFromFile", 1usize
                )
            });
        let __cordl_ret: T = unsafe { method.invoke_unchecked((), (filePath)) };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFromFile_JsonSerializerSettings1<T>(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                ),
                T,
                2usize,
            >("ReadFromFile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadFromFile", 2usize
                )
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (filePath, settings))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadFromText<T>(
        textReader: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                ),
                T,
                2usize,
            >("ReadFromText")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ReadFromText", 2usize
                )
            });
        let __cordl_ret: T = unsafe {
            method.invoke_unchecked((), (textReader, settings))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteCompactWithoutDefault<T>(
        content: T,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (T, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("WriteCompactWithoutDefault")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteCompactWithoutDefault", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (content, filePath))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteIndentedWithDefault<T>(
        content: T,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        indentation: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (T, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>, i32),
                quest_hook::libil2cpp::Void,
                3usize,
            >("WriteIndentedWithDefault")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteIndentedWithDefault", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (content, filePath, indentation))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteToFile<T>(
        content: T,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
        beforeSerialize: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonTextWriter>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    T,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonTextWriter,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("WriteToFile")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteToFile", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (content, filePath, settings, beforeSerialize))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteToText<T>(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        content: T,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
        beforeSerialize: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonTextWriter>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
                    T,
                    quest_hook::libil2cpp::Gc<
                        crate::Newtonsoft::Json::JsonSerializerSettings,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<
                                crate::Newtonsoft::Json::JsonTextWriter,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("WriteToText")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WriteToText", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (writer, content, settings, beforeSerialize))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
impl quest_hook::libil2cpp::ObjectType for crate::BGLib::JsonExtension::JsonFileHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
