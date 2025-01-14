#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
#[repr(C)]
#[derive(Debug)]
pub struct IAssetBundleResource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.ResourceProviders";
    const CLASS_NAME: &'static str = "IAssetBundleResource";
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
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource {
    pub fn GetAssetBundle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle>,
                0usize,
            >("GetAssetBundle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetAssetBundle", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AssetBundle> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+IAssetBundleResource")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::ResourceProviders::IAssetBundleResource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
