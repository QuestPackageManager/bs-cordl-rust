#[cfg(feature = "UnityEngine+UIElements+IBaseUxmlObjectFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct IBaseUxmlObjectFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IBaseUxmlObjectFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IBaseUxmlObjectFactory
    => "UnityEngine.UIElements"."IBaseUxmlObjectFactory"
);
#[cfg(feature = "UnityEngine+UIElements+IBaseUxmlObjectFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IBaseUxmlObjectFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IBaseUxmlObjectFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IBaseUxmlObjectFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IBaseUxmlObjectFactory")]
impl crate::UnityEngine::UIElements::IBaseUxmlObjectFactory {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IBaseUxmlObjectFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IBaseUxmlObjectFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+IBaseUxmlObjectFactory")]
impl AsRef<crate::UnityEngine::UIElements::IBaseUxmlFactory>
for crate::UnityEngine::UIElements::IBaseUxmlObjectFactory {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IBaseUxmlFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IBaseUxmlObjectFactory")]
impl AsMut<crate::UnityEngine::UIElements::IBaseUxmlFactory>
for crate::UnityEngine::UIElements::IBaseUxmlObjectFactory {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IBaseUxmlFactory {
        unsafe { std::mem::transmute(self) }
    }
}
