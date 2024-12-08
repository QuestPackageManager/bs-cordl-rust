#[cfg(feature = "MonoBehaviourCallbackHooks")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoBehaviourCallbackHooks {
    __cordl_parent: crate::UnityEngine::ResourceManagement::Util::ComponentSingleton_1<
        *mut crate::GlobalNamespace::MonoBehaviourCallbackHooks,
    >,
    pub m_OnUpdateDelegate: *mut crate::System::Action_1<f32>,
    pub m_OnLateUpdateDelegate: *mut crate::System::Action_1<f32>,
}
#[cfg(feature = "MonoBehaviourCallbackHooks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MonoBehaviourCallbackHooks =>
    ""."MonoBehaviourCallbackHooks"
);
#[cfg(feature = "MonoBehaviourCallbackHooks")]
impl std::ops::Deref for crate::GlobalNamespace::MonoBehaviourCallbackHooks {
    type Target = crate::UnityEngine::ResourceManagement::Util::ComponentSingleton_1<
        *mut crate::GlobalNamespace::MonoBehaviourCallbackHooks,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MonoBehaviourCallbackHooks")]
impl std::ops::DerefMut for crate::GlobalNamespace::MonoBehaviourCallbackHooks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MonoBehaviourCallbackHooks")]
impl crate::GlobalNamespace::MonoBehaviourCallbackHooks {
    pub fn GetGameObjectName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetGameObjectName", ())?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn add_OnLateUpdateDelegate(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_OnLateUpdateDelegate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_OnUpdateDelegate(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_OnUpdateDelegate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_OnLateUpdateDelegate(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_OnLateUpdateDelegate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_OnUpdateDelegate(
        &mut self,
        value: *mut crate::System::Action_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_OnUpdateDelegate", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MonoBehaviourCallbackHooks")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MonoBehaviourCallbackHooks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
