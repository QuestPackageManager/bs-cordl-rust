#[cfg(feature = "System+Runtime+CompilerServices+JitHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct JitHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+CompilerServices+JitHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::JitHelpers =>
    "System.Runtime.CompilerServices"."JitHelpers"
);
#[cfg(feature = "System+Runtime+CompilerServices+JitHelpers")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::JitHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+JitHelpers")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::JitHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+JitHelpers")]
impl crate::System::Runtime::CompilerServices::JitHelpers {
    pub fn UnsafeCast<T>(
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeCast", (o))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeEnumCast<T>(val: T) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeEnumCast", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsafeEnumCastLong<T>(val: T) -> quest_hook::libil2cpp::Result<i64>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnsafeEnumCastLong", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+JitHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::JitHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
