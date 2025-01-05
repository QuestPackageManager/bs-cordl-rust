#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::RuntimeHelpers =>
    "System.Runtime.CompilerServices"."RuntimeHelpers"
);
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::RuntimeHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::RuntimeHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
impl crate::System::Runtime::CompilerServices::RuntimeHelpers {
    pub fn GetHashCode(
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetHashCode", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectValue(
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectValue", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeArray_IntPtr0(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        fldHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeArray", (array, fldHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeArray_RuntimeFieldHandle1(
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        fldHandle: crate::System::RuntimeFieldHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeArray", (array, fldHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsReferenceOrContainsReferences<T>() -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsReferenceOrContainsReferences", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PrepareConstrainedRegions() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrepareConstrainedRegions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RunClassConstructor_IntPtr0(
        _cordl_type: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RunClassConstructor", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn RunClassConstructor_RuntimeTypeHandle1(
        _cordl_type: crate::System::RuntimeTypeHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RunClassConstructor", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn SufficientExecutionStack() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SufficientExecutionStack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnsureSufficientExecutionStack() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryEnsureSufficientExecutionStack", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OffsetToStringData() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_OffsetToStringData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::RuntimeHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
