#[cfg(feature = "UnityEngine+Assertions+Assert")]
#[repr(C)]
#[derive(Debug)]
pub struct _cordl_Assert {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Assertions+Assert")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Assertions::_cordl_Assert =>
    "UnityEngine.Assertions"."Assert"
);
#[cfg(feature = "UnityEngine+Assertions+Assert")]
impl std::ops::Deref for crate::UnityEngine::Assertions::_cordl_Assert {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Assertions+Assert")]
impl std::ops::DerefMut for crate::UnityEngine::Assertions::_cordl_Assert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Assertions+Assert")]
impl crate::UnityEngine::Assertions::_cordl_Assert {
    pub fn AreEqual_Gc_Gc_Gc3(
        expected: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        actual: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreEqual", (expected, actual, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreEqual_T_T0<T>(
        expected: T,
        actual: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreEqual", (expected, actual))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreEqual_T_T_Gc1<T>(
        expected: T,
        actual: T,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreEqual", (expected, actual, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreEqual_T_T_Gc_Gc2<T>(
        expected: T,
        actual: T,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        comparer: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreEqual", (expected, actual, message, comparer))?;
        Ok(__cordl_ret.into())
    }
    pub fn AreEqual_i32_i32_4(
        expected: i32,
        actual: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AreEqual", (expected, actual))?;
        Ok(__cordl_ret.into())
    }
    pub fn Fail(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Fail", (message, userMessage))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFalse(
        condition: bool,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsFalse", (condition, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNotNull_Gc_Gc2(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNotNull", (value, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNotNull_T0<T>(
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNotNull", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNotNull_T_Gc1<T>(
        value: T,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNotNull", (value, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNull_Gc_Gc2(
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNull", (value, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNull_T0<T>(
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNull", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNull_T_Gc1<T>(
        value: T,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNull", (value, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTrue_Gc1(
        condition: bool,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTrue", (condition, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTrue__cordl_bool0(
        condition: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTrue", (condition))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Assertions+Assert")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Assertions::_cordl_Assert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
