#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
#[repr(C)]
#[derive(Debug)]
pub struct SubsystemDescriptorStore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::SubsystemsImplementation::SubsystemDescriptorStore {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.SubsystemsImplementation";
    const CLASS_NAME: &'static str = "SubsystemDescriptorStore";
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
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
impl std::ops::Deref
for crate::UnityEngine::SubsystemsImplementation::SubsystemDescriptorStore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
impl std::ops::DerefMut
for crate::UnityEngine::SubsystemsImplementation::SubsystemDescriptorStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
impl crate::UnityEngine::SubsystemsImplementation::SubsystemDescriptorStore {
    pub fn AddDescriptorSubset<TBaseTypeInList, TQueryType>(
        copyFrom: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TBaseTypeInList>,
        >,
        copyTo: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TQueryType>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TBaseTypeInList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TQueryType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddDescriptorSubset", (copyFrom, copyTo))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearManagedDescriptors() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearManagedDescriptors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSubsystemDescriptors<T>(
        descriptors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSubsystemDescriptors", (descriptors))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeManagedDescriptor(
        ptr: crate::System::IntPtr,
        desc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::IntegratedSubsystemDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeManagedDescriptor", (ptr, desc))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDeprecatedDescriptor(
        descriptor: quest_hook::libil2cpp::Gc<crate::UnityEngine::SubsystemDescriptor>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterDeprecatedDescriptor", (descriptor))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDescriptor<TDescriptor, TBaseTypeInList>(
        descriptor: TDescriptor,
        storeInList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<TBaseTypeInList>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDescriptor: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TBaseTypeInList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterDescriptor", (descriptor, storeInList))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReportSingleSubsystemAnalytics(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReportSingleSubsystemAnalytics", (id))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SubsystemsImplementation+SubsystemDescriptorStore")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SubsystemsImplementation::SubsystemDescriptorStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
