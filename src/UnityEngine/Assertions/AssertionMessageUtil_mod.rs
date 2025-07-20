#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct AssertionMessageUtil {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Assertions::AssertionMessageUtil {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Assertions";
    const CLASS_NAME: &'static str = "AssertionMessageUtil";
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
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
impl std::ops::Deref for crate::UnityEngine::Assertions::AssertionMessageUtil {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Assertions::AssertionMessageUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("BooleanFailureMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Assertions::AssertionMessageUtil as
                    quest_hook::libil2cpp::Type > ::class(), "BooleanFailureMessage",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (expected))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEqualityMessage(
        actual: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        expected: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        expectEqual: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Assertions::AssertionMessageUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("GetEqualityMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Assertions::AssertionMessageUtil as
                    quest_hook::libil2cpp::Type > ::class(), "GetEqualityMessage", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (actual, expected, expectEqual))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMessage_Il2CppString0(
        failureMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Assertions::AssertionMessageUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Assertions::AssertionMessageUtil as
                    quest_hook::libil2cpp::Type > ::class(), "GetMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (failureMessage))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMessage_Il2CppString1(
        failureMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        expected: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Assertions::AssertionMessageUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Assertions::AssertionMessageUtil as
                    quest_hook::libil2cpp::Type > ::class(), "GetMessage", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (failureMessage, expected))? };
        Ok(__cordl_ret.into())
    }
    pub fn NullFailureMessage(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        expectNull: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::Assertions::AssertionMessageUtil as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, bool),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("NullFailureMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::Assertions::AssertionMessageUtil as
                    quest_hook::libil2cpp::Type > ::class(), "NullFailureMessage", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (value, expectNull))? };
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
