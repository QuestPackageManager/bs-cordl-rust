#[cfg(feature = "UnityEngine+UIElements+RuntimeEventDispatcher")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeEventDispatcher {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+RuntimeEventDispatcher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RuntimeEventDispatcher
    => "UnityEngine.UIElements"."RuntimeEventDispatcher"
);
#[cfg(feature = "UnityEngine+UIElements+RuntimeEventDispatcher")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RuntimeEventDispatcher {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RuntimeEventDispatcher")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RuntimeEventDispatcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RuntimeEventDispatcher")]
impl crate::UnityEngine::UIElements::RuntimeEventDispatcher {
    pub fn Create() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventDispatcher>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventDispatcher,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+RuntimeEventDispatcher")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RuntimeEventDispatcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
