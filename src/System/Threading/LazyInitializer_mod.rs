#[cfg(feature = "System+Threading+LazyInitializer")]
#[repr(C)]
#[derive(Debug)]
pub struct LazyInitializer {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Threading+LazyInitializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::LazyInitializer =>
    "System.Threading"."LazyInitializer"
);
#[cfg(feature = "System+Threading+LazyInitializer")]
impl std::ops::Deref for crate::System::Threading::LazyInitializer {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+LazyInitializer")]
impl std::ops::DerefMut for crate::System::Threading::LazyInitializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+LazyInitializer")]
impl crate::System::Threading::LazyInitializer {
    pub fn EnsureInitializedCore_ByRefMut0<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureInitializedCore", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitializedCore_ByRefMut_ByRefMut_Gc2<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        initialized: quest_hook::libil2cpp::ByRefMut<bool>,
        syncLock: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        valueFactory: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EnsureInitializedCore",
                (target, initialized, syncLock, valueFactory),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitializedCore_ByRefMut_Gc3<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        syncLock: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        valueFactory: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureInitializedCore", (target, syncLock, valueFactory))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitializedCore_Gc1<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        valueFactory: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureInitializedCore", (target, valueFactory))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitialized_ByRefMut0<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureInitialized", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitialized_ByRefMut_ByRefMut_Gc2<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        initialized: quest_hook::libil2cpp::ByRefMut<bool>,
        syncLock: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        valueFactory: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureInitialized", (target, initialized, syncLock, valueFactory))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitialized_ByRefMut_Gc3<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        syncLock: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        valueFactory: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureInitialized", (target, syncLock, valueFactory))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitialized_Gc1<T>(
        target: quest_hook::libil2cpp::ByRefMut<T>,
        valueFactory: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureInitialized", (target, valueFactory))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureLockInitialized(
        syncLock: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureLockInitialized", (syncLock))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+LazyInitializer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Threading::LazyInitializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
