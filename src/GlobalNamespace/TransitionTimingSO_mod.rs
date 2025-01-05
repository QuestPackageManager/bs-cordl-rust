#[cfg(feature = "TransitionTimingSO")]
#[repr(C)]
#[derive(Debug)]
pub struct TransitionTimingSO {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >,
    pub _easeType: crate::GlobalNamespace::EaseType,
    pub _easeDuration: f32,
}
#[cfg(feature = "TransitionTimingSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TransitionTimingSO => ""
    ."TransitionTimingSO"
);
#[cfg(feature = "TransitionTimingSO")]
impl std::ops::Deref for crate::GlobalNamespace::TransitionTimingSO {
    type Target = quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PersistentScriptableObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TransitionTimingSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::TransitionTimingSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TransitionTimingSO")]
impl crate::GlobalNamespace::TransitionTimingSO {
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
    pub fn get_easeDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_easeDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_easeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EaseType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EaseType = __cordl_object
            .invoke("get_easeType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TransitionTimingSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TransitionTimingSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
