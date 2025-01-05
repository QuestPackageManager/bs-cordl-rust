#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct AssertionMessageUtil {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Assertions::AssertionMessageUtil =>
    "UnityEngine.Assertions"."AssertionMessageUtil"
);
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
impl std::ops::Deref for crate::UnityEngine::Assertions::AssertionMessageUtil {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
impl std::ops::DerefMut for crate::UnityEngine::Assertions::AssertionMessageUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
impl crate::UnityEngine::Assertions::AssertionMessageUtil {
    pub fn BooleanFailureMessage(
        expected: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BooleanFailureMessage", (expected))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEqualityMessage(
        actual: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        expected: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        expectEqual: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEqualityMessage", (actual, expected, expectEqual))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMessage_Gc0(
        failureMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMessage", (failureMessage))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMessage_Gc1(
        failureMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expected: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMessage", (failureMessage, expected))?;
        Ok(__cordl_ret.into())
    }
    pub fn NullFailureMessage(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        expectNull: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NullFailureMessage", (value, expectNull))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Assertions::AssertionMessageUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
