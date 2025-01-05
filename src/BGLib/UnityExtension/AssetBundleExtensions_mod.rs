#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetBundleExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::UnityExtension::AssetBundleExtensions =>
    "BGLib.UnityExtension"."AssetBundleExtensions"
);
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl std::ops::Deref for crate::BGLib::UnityExtension::AssetBundleExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl std::ops::DerefMut for crate::BGLib::UnityExtension::AssetBundleExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl crate::BGLib::UnityExtension::AssetBundleExtensions {
    pub fn GetAwaiter_Gc0(
        assetBundleCreateRequest: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AssetBundleCreateRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
        >,
    > {
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAwaiter", (assetBundleCreateRequest))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAwaiter_Gc1(
        resourceRequest: quest_hook::libil2cpp::Gc<crate::UnityEngine::ResourceRequest>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        >,
    > {
        let __cordl_ret: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAwaiter", (resourceRequest))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+UnityExtension+AssetBundleExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::UnityExtension::AssetBundleExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
