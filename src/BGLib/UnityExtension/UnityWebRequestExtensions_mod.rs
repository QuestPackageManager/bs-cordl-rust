#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityWebRequestExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::UnityWebRequestExtensions
    => "BGLib.UnityExtension"."UnityWebRequestExtensions"
);
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
impl std::ops::Deref for crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
impl crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    pub fn GetAwaiter(
        webRequestOperation: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequestAsyncOperation,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            crate::UnityEngine::Networking::UnityWebRequest_Result,
        >,
    > {
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            crate::UnityEngine::Networking::UnityWebRequest_Result,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAwaiter", (webRequestOperation))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendWebRequestAsync(
        request: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::UnityWebRequest,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::UnityEngine::Networking::UnityWebRequest_Result,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::UnityEngine::Networking::UnityWebRequest_Result,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendWebRequestAsync", (request, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+UnityWebRequestExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::UnityWebRequestExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
