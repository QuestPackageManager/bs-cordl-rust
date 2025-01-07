#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
#[repr(C)]
#[derive(Debug)]
pub struct ResourceManagerConfig {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Util::ResourceManagerConfig {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Util";
    const CLASS_NAME: &'static str = "ResourceManagerConfig";
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
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Util::ResourceManagerConfig {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Util::ResourceManagerConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
impl crate::UnityEngine::ResourceManagement::Util::ResourceManagerConfig {
    pub fn CreateArrayResult_Il2CppArray1<TObject>(
        allAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<TObject>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TObject = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArrayResult", (allAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArrayResult_Type_Il2CppArray0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        allAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArrayResult", (_cordl_type, allAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateListResult_Il2CppArray1<TObject>(
        allAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<TObject>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TObject = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateListResult", (allAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateListResult_Type_Il2CppArray0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        allAssets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IList>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IList> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateListResult", (_cordl_type, allAssets))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractKeyAndSubKey(
        keyObj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mainKey: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        subKey: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractKeyAndSubKey", (keyObj, mainKey, subKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInstance<T1, T2>() -> quest_hook::libil2cpp::Result<bool>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPathRemote(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPathRemote", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlatformCanLoadLocallyFromUrlPath() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PlatformCanLoadLocallyFromUrlPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldPathUseWebRequest(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldPathUseWebRequest", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn StripQueryParameters(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StripQueryParameters", (path))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+ResourceManagerConfig")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Util::ResourceManagerConfig {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
