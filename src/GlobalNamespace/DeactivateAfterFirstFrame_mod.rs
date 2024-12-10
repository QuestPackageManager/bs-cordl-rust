#[cfg(feature = "DeactivateAfterFirstFrame")]
#[repr(C)]
#[derive(Debug)]
pub struct DeactivateAfterFirstFrame {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "DeactivateAfterFirstFrame")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DeactivateAfterFirstFrame => ""
    ."DeactivateAfterFirstFrame"
);
#[cfg(feature = "DeactivateAfterFirstFrame")]
impl std::ops::Deref for crate::GlobalNamespace::DeactivateAfterFirstFrame {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DeactivateAfterFirstFrame")]
impl std::ops::DerefMut for crate::GlobalNamespace::DeactivateAfterFirstFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DeactivateAfterFirstFrame")]
impl crate::GlobalNamespace::DeactivateAfterFirstFrame {
    #[cfg(feature = "DeactivateAfterFirstFrame+_Start_d__0")]
    pub type _Start_d__0 = crate::GlobalNamespace::DeactivateAfterFirstFrame__Start_d__0;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("Start", ())?;
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
#[cfg(feature = "DeactivateAfterFirstFrame")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DeactivateAfterFirstFrame {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
