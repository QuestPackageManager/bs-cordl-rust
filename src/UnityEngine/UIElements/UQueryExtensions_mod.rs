#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct UQueryExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UQueryExtensions =>
    "UnityEngine.UIElements"."UQueryExtensions"
);
#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UQueryExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UQueryExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
impl crate::UnityEngine::UIElements::UQueryExtensions {
    pub fn Q_Gc_Gc_Gc0<T>(
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        className: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Q", (e, name, className))?;
        Ok(__cordl_ret.into())
    }
    pub fn Q_Gc_Gc_Gc1(
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        className: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Q", (e, name, className))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UQueryExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
