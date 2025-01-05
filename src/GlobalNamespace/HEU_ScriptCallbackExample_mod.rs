#[cfg(feature = "HEU_ScriptCallbackExample")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ScriptCallbackExample {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "HEU_ScriptCallbackExample")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HEU_ScriptCallbackExample => ""
    ."HEU_ScriptCallbackExample"
);
#[cfg(feature = "HEU_ScriptCallbackExample")]
impl std::ops::Deref for crate::GlobalNamespace::HEU_ScriptCallbackExample {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ScriptCallbackExample")]
impl std::ops::DerefMut for crate::GlobalNamespace::HEU_ScriptCallbackExample {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ScriptCallbackExample")]
impl crate::GlobalNamespace::HEU_ScriptCallbackExample {
    pub fn AssetCallbackNoMsg(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssetCallbackNoMsg", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AssetCallbackWithMsg(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssetCallbackWithMsg", (msg))?;
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
#[cfg(feature = "HEU_ScriptCallbackExample")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HEU_ScriptCallbackExample {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
