#[cfg(feature = "AssertExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct AssertExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "AssertExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AssertExtensions => ""
    ."AssertExtensions"
);
#[cfg(feature = "AssertExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::AssertExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AssertExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::AssertExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AssertExtensions")]
impl crate::GlobalNamespace::AssertExtensions {
    pub const kUnityAssertions: &'static str = "UNITY_ASSERTIONS";
    pub fn GetMessage<T>(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: T,
        other: T,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMessage", (pattern, value, other, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn GreaterOrEqual(
        value: f32,
        expectedLessValue: f32,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GreaterOrEqual", (value, expectedLessValue, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn LessThan(
        value: f32,
        expectedGreaterValue: f32,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LessThan", (value, expectedGreaterValue, message))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AssertExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AssertExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
