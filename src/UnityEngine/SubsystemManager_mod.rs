#[cfg(feature = "UnityEngine+SubsystemManager")]
#[repr(C)]
#[derive(Debug)]
pub struct SubsystemManager {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+SubsystemManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SubsystemManager => "UnityEngine"
    ."SubsystemManager"
);
#[cfg(feature = "UnityEngine+SubsystemManager")]
impl std::ops::Deref for crate::UnityEngine::SubsystemManager {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemManager")]
impl std::ops::DerefMut for crate::UnityEngine::SubsystemManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemManager")]
impl crate::UnityEngine::SubsystemManager {
    pub fn AddSubsystemSubset<TBaseTypeInList, TQueryType>(
        copyFrom: quest_hook::libil2cpp::Gc<TBaseTypeInList>,
        copyTo: quest_hook::libil2cpp::Gc<TQueryType>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TBaseTypeInList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TQueryType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddSubsystemSubset", (copyFrom, copyTo))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearSubsystems() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearSubsystems", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstances<T>(
        subsystems: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInstances", (subsystems))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIntegratedSubsystemByPtr(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::IntegratedSubsystem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::IntegratedSubsystem,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIntegratedSubsystemByPtr", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSubsystemDescriptors<T>(
        descriptors: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSubsystemDescriptors", (descriptors))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSubsystems<T>(
        subsystems: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSubsystems", (subsystems))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeIntegratedSubsystem(
        ptr: crate::System::IntPtr,
        subsystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::IntegratedSubsystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeIntegratedSubsystem", (ptr, subsystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReloadSubsystemsCompleted() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReloadSubsystemsCompleted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReloadSubsystemsStarted() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReloadSubsystemsStarted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDeprecatedSubsystem(
        subsystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::Subsystem>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveDeprecatedSubsystem", (subsystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveIntegratedSubsystemByPtr(
        ptr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveIntegratedSubsystemByPtr", (ptr))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveStandaloneSubsystem(
        subsystem: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::SubsystemsImplementation::SubsystemWithProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveStandaloneSubsystem", (subsystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn StaticConstructScriptingClassMap() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StaticConstructScriptingClassMap", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SubsystemManager")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SubsystemManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
