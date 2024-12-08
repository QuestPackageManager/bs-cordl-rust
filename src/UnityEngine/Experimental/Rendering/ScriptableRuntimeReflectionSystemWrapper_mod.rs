#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableRuntimeReflectionSystemWrapper {
    __cordl_parent: crate::System::Object,
    pub _implementation_k__BackingField: *mut crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper =>
    "UnityEngine.Experimental.Rendering"."ScriptableRuntimeReflectionSystemWrapper"
);
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
impl std::ops::Deref
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
impl crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper {
    pub fn get_implementation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem = __cordl_object
            .invoke("get_implementation", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Internal_ScriptableRuntimeReflectionSystemWrapper_TickRealtimeProbes(
        &mut self,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Internal_ScriptableRuntimeReflectionSystemWrapper_TickRealtimeProbes",
                (result),
            )?;
        Ok(__cordl_ret)
    }
    pub fn set_implementation(
        &mut self,
        value: *mut crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_implementation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "UnityEngine+Experimental+Rendering+ScriptableRuntimeReflectionSystemWrapper"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::Rendering::ScriptableRuntimeReflectionSystemWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
