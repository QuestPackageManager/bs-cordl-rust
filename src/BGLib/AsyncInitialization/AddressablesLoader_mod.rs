#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesLoader {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BGLib::AsyncInitialization::AddressablesLoader
    => "BGLib.AsyncInitialization"."AddressablesLoader"
);
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl std::ops::Deref for crate::BGLib::AsyncInitialization::AddressablesLoader {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl std::ops::DerefMut for crate::BGLib::AsyncInitialization::AddressablesLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl crate::BGLib::AsyncInitialization::AddressablesLoader {
    pub fn InstantiateFromAddressableToContainer<TInstantiate, TReturn>(
        prefab: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AddressableAssets::AssetReferenceGameObject,
        >,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TReturn>>
    where
        TInstantiate: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TReturn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<TReturn> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstantiateFromAddressableToContainer", (prefab, container))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BGLib+AsyncInitialization+AddressablesLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::AsyncInitialization::AddressablesLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
