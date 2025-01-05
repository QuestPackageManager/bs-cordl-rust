#[cfg(feature = "UnityEngine+UIElements+DefaultDispatchingStrategy")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultDispatchingStrategy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+DefaultDispatchingStrategy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DefaultDispatchingStrategy => "UnityEngine.UIElements"
    ."DefaultDispatchingStrategy"
);
#[cfg(feature = "UnityEngine+UIElements+DefaultDispatchingStrategy")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DefaultDispatchingStrategy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultDispatchingStrategy")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DefaultDispatchingStrategy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultDispatchingStrategy")]
impl crate::UnityEngine::UIElements::DefaultDispatchingStrategy {
    pub fn CanDispatchEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanDispatchEvent", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn DispatchEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DispatchEvent", (evt, panel))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultDispatchingStrategy")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DefaultDispatchingStrategy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultDispatchingStrategy")]
impl AsRef<crate::UnityEngine::UIElements::IEventDispatchingStrategy>
for crate::UnityEngine::UIElements::DefaultDispatchingStrategy {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IEventDispatchingStrategy {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultDispatchingStrategy")]
impl AsMut<crate::UnityEngine::UIElements::IEventDispatchingStrategy>
for crate::UnityEngine::UIElements::DefaultDispatchingStrategy {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IEventDispatchingStrategy {
        unsafe { std::mem::transmute(self) }
    }
}
