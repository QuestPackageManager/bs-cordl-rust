#[cfg(feature = "System+IO+DirectoryInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DirectoryInfo {
    __cordl_parent: crate::System::IO::FileSystemInfo,
}
#[cfg(feature = "System+IO+DirectoryInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::DirectoryInfo => "System.IO"
    ."DirectoryInfo"
);
#[cfg(feature = "System+IO+DirectoryInfo")]
impl std::ops::Deref for crate::System::IO::DirectoryInfo {
    type Target = crate::System::IO::FileSystemInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+DirectoryInfo")]
impl std::ops::DerefMut for crate::System::IO::DirectoryInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+DirectoryInfo")]
impl crate::System::IO::DirectoryInfo {
    pub fn Delete(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Delete", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDirectories_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::IO::DirectoryInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::IO::DirectoryInfo,
        > = __cordl_object.invoke("GetDirectories", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDirectories_Il2CppString_EnumerationOptions1(
        &mut self,
        searchPattern: *mut quest_hook::libil2cpp::Il2CppString,
        enumerationOptions: *mut crate::System::IO::EnumerationOptions,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::IO::DirectoryInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::IO::DirectoryInfo,
        > = __cordl_object
            .invoke("GetDirectories", (searchPattern, enumerationOptions))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        originalPath: *mut quest_hook::libil2cpp::Il2CppString,
        fullPath: *mut quest_hook::libil2cpp::Il2CppString,
        fileName: *mut quest_hook::libil2cpp::Il2CppString,
        isNormalized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (originalPath, fullPath, fileName, isNormalized))?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppString0(
        path: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_Il2CppString_Il2CppString__cordl_bool1(
        originalPath: *mut quest_hook::libil2cpp::Il2CppString,
        fullPath: *mut quest_hook::libil2cpp::Il2CppString,
        fileName: *mut quest_hook::libil2cpp::Il2CppString,
        isNormalized: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (originalPath, fullPath, fileName, isNormalized))?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext2(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        path: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_Il2CppString_Il2CppString__cordl_bool1(
        &mut self,
        originalPath: *mut quest_hook::libil2cpp::Il2CppString,
        fullPath: *mut quest_hook::libil2cpp::Il2CppString,
        fileName: *mut quest_hook::libil2cpp::Il2CppString,
        isNormalized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (originalPath, fullPath, fileName, isNormalized))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+IO+DirectoryInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::DirectoryInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
