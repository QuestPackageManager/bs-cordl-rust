#[cfg(feature = "UnityEngine+UIElements+RuntimeUIElementsBridge")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeUIElementsBridge {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIElementsBridge,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RuntimeUIElementsBridge")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RuntimeUIElementsBridge
    => "UnityEngine.UIElements"."RuntimeUIElementsBridge"
);
#[cfg(feature = "UnityEngine+UIElements+RuntimeUIElementsBridge")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RuntimeUIElementsBridge {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIElementsBridge,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RuntimeUIElementsBridge")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RuntimeUIElementsBridge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RuntimeUIElementsBridge")]
impl crate::UnityEngine::UIElements::RuntimeUIElementsBridge {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetWantsMouseJumping(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetWantsMouseJumping", (value))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+UIElements+RuntimeUIElementsBridge")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::RuntimeUIElementsBridge {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
