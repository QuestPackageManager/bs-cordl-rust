#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemWithProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SubsystemWithProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _running_k__BackingField: bool,
    pub _providerBase_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::SubsystemsImplementation::SubsystemProvider,
    >,
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemWithProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SubsystemsImplementation::SubsystemWithProvider =>
    "UnityEngine.SubsystemsImplementation"."SubsystemWithProvider"
);
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemWithProvider")]
impl std::ops::Deref
for crate::UnityEngine::SubsystemsImplementation::SubsystemWithProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemWithProvider")]
impl std::ops::DerefMut
for crate::UnityEngine::SubsystemsImplementation::SubsystemWithProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemWithProvider")]
impl crate::UnityEngine::SubsystemsImplementation::SubsystemWithProvider {
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Stop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", ())?;
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
    pub fn get_providerBase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::SubsystemsImplementation::SubsystemProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::SubsystemsImplementation::SubsystemProvider,
        > = __cordl_object.invoke("get_providerBase", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_running(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_running", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_running(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_running", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemWithProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SubsystemsImplementation::SubsystemWithProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemWithProvider")]
impl AsRef<crate::UnityEngine::ISubsystem>
for crate::UnityEngine::SubsystemsImplementation::SubsystemWithProvider {
    fn as_ref(&self) -> &crate::UnityEngine::ISubsystem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemWithProvider")]
impl AsMut<crate::UnityEngine::ISubsystem>
for crate::UnityEngine::SubsystemsImplementation::SubsystemWithProvider {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::ISubsystem {
        unsafe { std::mem::transmute(self) }
    }
}
