#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IntegratedSubsystem_1<TSubsystemDescriptor: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::IntegratedSubsystem>,
    __cordl_phantom_TSubsystemDescriptor: std::marker::PhantomData<TSubsystemDescriptor>,
}
#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::IntegratedSubsystem_1 <
    TSubsystemDescriptor > => "UnityEngine"."IntegratedSubsystem`1" <
    TSubsystemDescriptor >
);
#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
impl<TSubsystemDescriptor: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::IntegratedSubsystem_1<TSubsystemDescriptor> {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::IntegratedSubsystem>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
impl<TSubsystemDescriptor: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::IntegratedSubsystem_1<TSubsystemDescriptor> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
impl<
    TSubsystemDescriptor: quest_hook::libil2cpp::Type,
> crate::UnityEngine::IntegratedSubsystem_1<TSubsystemDescriptor> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSubsystemDescriptor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSubsystemDescriptor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SubsystemDescriptor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<TSubsystemDescriptor>
    where
        TSubsystemDescriptor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TSubsystemDescriptor = __cordl_object
            .invoke("get_SubsystemDescriptor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subsystemDescriptor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<TSubsystemDescriptor>
    where
        TSubsystemDescriptor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TSubsystemDescriptor = __cordl_object
            .invoke("get_subsystemDescriptor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystem_1")]
impl<TSubsystemDescriptor: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::IntegratedSubsystem_1<TSubsystemDescriptor> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
