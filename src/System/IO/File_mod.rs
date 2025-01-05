#[cfg(feature = "System+IO+File")]
#[repr(C)]
#[derive(Debug)]
pub struct File {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+IO+File")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::File => "System.IO"."File"
);
#[cfg(feature = "System+IO+File")]
impl std::ops::Deref for crate::System::IO::File {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+File")]
impl std::ops::DerefMut for crate::System::IO::File {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+File")]
impl crate::System::IO::File {
    pub fn AppendText(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::StreamWriter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::StreamWriter> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendText", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateText(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::StreamWriter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::StreamWriter> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateText", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_Gc0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::FileStream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::FileStream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_i32_1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::FileStream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::FileStream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (path, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn Delete(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Delete", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn Exists(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exists", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAccessControl_AccessControlSections1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::AccessControl::FileSecurity>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::FileSecurity,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAccessControl", (path, includeSections))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAccessControl_Gc0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Security::AccessControl::FileSecurity>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::FileSecurity,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAccessControl", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IO::FileAttributes> {
        let __cordl_ret: crate::System::IO::FileAttributes = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttributes", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalReadAllLines(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalReadAllLines", (path, encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalReadAllText(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalReadAllText", (path, encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteAllBytes(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalWriteAllBytes", (path, bytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalWriteAllLines(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        contents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalWriteAllLines", (writer, contents))?;
        Ok(__cordl_ret.into())
    }
    pub fn Move(
        sourceFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Move", (sourceFileName, destFileName))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenRead(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::FileStream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::FileStream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenRead", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenText(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::StreamReader>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::StreamReader> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenText", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenWrite(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::FileStream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::FileStream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenWrite", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn Open_FileAccess_FileShare1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::FileStream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::FileStream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Open", (path, mode, access, share))?;
        Ok(__cordl_ret.into())
    }
    pub fn Open_Gc_FileMode0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::FileStream>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::FileStream> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Open", (path, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAllBytes(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadAllBytes", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAllBytesUnknownLength(
        fs: quest_hook::libil2cpp::Gc<crate::System::IO::FileStream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadAllBytesUnknownLength", (fs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAllLines(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadAllLines", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAllText(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadAllText", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn Replace_Gc_Gc_Gc0(
        sourceFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destinationFileName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        destinationBackupFileName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Replace",
                (sourceFileName, destinationFileName, destinationBackupFileName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Replace__cordl_bool1(
        sourceFileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destinationFileName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        destinationBackupFileName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        ignoreMetadataErrors: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Replace",
                (
                    sourceFileName,
                    destinationFileName,
                    destinationBackupFileName,
                    ignoreMetadataErrors,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAllBytes(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteAllBytes", (path, bytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAllLines_Gc_Gc0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        contents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteAllLines", (path, contents))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAllLines_Gc_Gc1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        contents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteAllLines", (path, contents))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAllText_Gc1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        contents: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteAllText", (path, contents, encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAllText_Gc_Gc0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        contents: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteAllText", (path, contents))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+File")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::File {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
