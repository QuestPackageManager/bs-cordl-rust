#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IntegratedSubsystemDescriptor_1<TSubsystem: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::IntegratedSubsystemDescriptor,
    __cordl_phantom_TSubsystem: std::marker::PhantomData<TSubsystem>,
}
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::IntegratedSubsystemDescriptor_1 <
    TSubsystem > => "UnityEngine"."IntegratedSubsystemDescriptor`1" < TSubsystem >
);
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor_1")]
impl<TSubsystem: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::IntegratedSubsystemDescriptor_1<TSubsystem> {
    type Target = crate::UnityEngine::IntegratedSubsystemDescriptor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor_1")]
impl<TSubsystem: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::IntegratedSubsystemDescriptor_1<TSubsystem> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor_1")]
impl<
    TSubsystem: quest_hook::libil2cpp::Type,
> crate::UnityEngine::IntegratedSubsystemDescriptor_1<TSubsystem> {
    pub fn Create(&mut self) -> quest_hook::libil2cpp::Result<TSubsystem>
    where
        TSubsystem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TSubsystem = __cordl_object.invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystem>,
    >
    where
        TSubsystem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::ISubsystem> = __cordl_object
            .invoke("CreateImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSubsystem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        TSubsystem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+IntegratedSubsystemDescriptor_1")]
impl<TSubsystem: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::IntegratedSubsystemDescriptor_1<TSubsystem> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
