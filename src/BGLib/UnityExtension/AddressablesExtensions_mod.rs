#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::AddressablesExtensions =>
    "BGLib.UnityExtension"."AddressablesExtensions"
);
#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
impl std::ops::Deref for crate::BGLib::UnityExtension::AddressablesExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::AddressablesExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
impl crate::BGLib::UnityExtension::AddressablesExtensions {
    pub fn GetAwaiter<T>(
        asyncOperationHandle: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            T,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<T> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAwaiter", (asyncOperationHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadContent<T>(
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadContent", (label))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadContentAsync<T>(
        label: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::IKeyEvaluator,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<T>,
                >,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<T>,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadContentAsync", (label))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadContentOperation<T>(
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationHandle_1<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadContentOperation", (label))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+AddressablesExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::AddressablesExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
