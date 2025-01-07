#[cfg(feature = "UnityEngine+Playables+ScriptPlayableBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptPlayableBinding {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Playables+ScriptPlayableBinding")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Playables::ScriptPlayableBinding {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Playables";
    const CLASS_NAME: &'static str = "ScriptPlayableBinding";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Playables+ScriptPlayableBinding")]
impl std::ops::Deref for crate::UnityEngine::Playables::ScriptPlayableBinding {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+ScriptPlayableBinding")]
impl std::ops::DerefMut for crate::UnityEngine::Playables::ScriptPlayableBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+ScriptPlayableBinding")]
impl crate::UnityEngine::Playables::ScriptPlayableBinding {
    pub fn Create(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        key: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableBinding> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableBinding = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (name, key, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateScriptOutput(
        graph: crate::UnityEngine::Playables::PlayableGraph,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Playables::PlayableOutput> {
        let __cordl_ret: crate::UnityEngine::Playables::PlayableOutput = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateScriptOutput", (graph, name))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Playables+ScriptPlayableBinding")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Playables::ScriptPlayableBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
