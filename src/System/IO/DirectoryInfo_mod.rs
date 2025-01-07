#[cfg(feature = "System+IO+DirectoryInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct DirectoryInfo {
    __cordl_parent: crate::System::IO::FileSystemInfo,
}
#[cfg(feature = "System+IO+DirectoryInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::IO::DirectoryInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.IO";
    const CLASS_NAME: &'static str = "DirectoryInfo";
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
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectories_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::IO::DirectoryInfo>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::IO::DirectoryInfo>,
            >,
        > = __cordl_object.invoke("GetDirectories", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDirectories_Il2CppString_EnumerationOptions1(
        &mut self,
        searchPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        enumerationOptions: quest_hook::libil2cpp::Gc<
            crate::System::IO::EnumerationOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::IO::DirectoryInfo>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::IO::DirectoryInfo>,
            >,
        > = __cordl_object
            .invoke("GetDirectories", (searchPattern, enumerationOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        originalPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isNormalized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (originalPath, fullPath, fileName, isNormalized))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalEnumerateInfos(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchTarget: crate::System::IO::SearchTarget,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::IO::FileSystemInfo>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::IO::FileSystemInfo>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InternalEnumerateInfos",
                (path, searchPattern, searchTarget, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_Il2CppString_Il2CppString__cordl_bool1(
        originalPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isNormalized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (originalPath, fullPath, fileName, isNormalized))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext2(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppString_Il2CppString__cordl_bool1(
        &mut self,
        originalPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fileName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isNormalized: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (originalPath, fullPath, fileName, isNormalized))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret.into())
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
