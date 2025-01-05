#[cfg(feature = "UnityEngine+UIElements+IEnumerableExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct IEnumerableExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+IEnumerableExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IEnumerableExtensions
    => "UnityEngine.UIElements"."IEnumerableExtensions"
);
#[cfg(feature = "UnityEngine+UIElements+IEnumerableExtensions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IEnumerableExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IEnumerableExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IEnumerableExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IEnumerableExtensions")]
impl crate::UnityEngine::UIElements::IEnumerableExtensions {
    pub fn HasValues(
        collection: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasValues", (collection))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+IEnumerableExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IEnumerableExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
