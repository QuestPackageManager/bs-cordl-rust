#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonFileHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+JsonExtension+JsonFileHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::JsonExtension::JsonFileHandler =>
    "BGLib.JsonExtension"."JsonFileHandler"
);
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
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadFromFile", (filePath))?;
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
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadFromFile", (filePath, settings))?;
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
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadFromText", (textReader, settings))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteCompactWithoutDefault", (content, filePath))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteIndentedWithDefault", (content, filePath, indentation))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteToFile<T>(
        content: T,
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
        beforeSerialize: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::Newtonsoft::Json::JsonTextWriter>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteToFile", (content, filePath, settings, beforeSerialize))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteToText<T>(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        content: T,
        settings: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonSerializerSettings,
        >,
        beforeSerialize: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::Newtonsoft::Json::JsonTextWriter>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteToText", (writer, content, settings, beforeSerialize))?;
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
