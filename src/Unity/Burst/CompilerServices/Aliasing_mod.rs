#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
#[repr(C)]
#[derive(Debug)]
pub struct Aliasing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Burst::CompilerServices::Aliasing =>
    "Unity.Burst.CompilerServices"."Aliasing"
);
#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
impl std::ops::Deref for crate::Unity::Burst::CompilerServices::Aliasing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
impl std::ops::DerefMut for crate::Unity::Burst::CompilerServices::Aliasing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
impl crate::Unity::Burst::CompilerServices::Aliasing {
    pub fn ExpectAliased_ByRefMut_ByRefMut1<A, B>(
        a: quest_hook::libil2cpp::ByRefMut<A>,
        b: quest_hook::libil2cpp::ByRefMut<B>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpectAliased", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectAliased_ByRefMut_Il2CppObject3<A>(
        a: quest_hook::libil2cpp::ByRefMut<A>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpectAliased", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectAliased_Il2CppObject_ByRefMut2<B>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::ByRefMut<B>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpectAliased", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectAliased_Il2CppObject_Il2CppObject0(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpectAliased", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectNotAliased_ByRefMut_ByRefMut1<A, B>(
        a: quest_hook::libil2cpp::ByRefMut<A>,
        b: quest_hook::libil2cpp::ByRefMut<B>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpectNotAliased", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectNotAliased_ByRefMut_Il2CppObject3<A>(
        a: quest_hook::libil2cpp::ByRefMut<A>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        A: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpectNotAliased", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectNotAliased_Il2CppObject_ByRefMut2<B>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::ByRefMut<B>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        B: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpectNotAliased", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpectNotAliased_Il2CppObject_Il2CppObject0(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpectNotAliased", (a, b))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+CompilerServices+Aliasing")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Burst::CompilerServices::Aliasing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
