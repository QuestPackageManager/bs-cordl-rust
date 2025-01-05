#[cfg(feature = "UnityEngine+NoAllocHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct NoAllocHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+NoAllocHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::NoAllocHelpers => "UnityEngine"
    ."NoAllocHelpers"
);
#[cfg(feature = "UnityEngine+NoAllocHelpers")]
impl std::ops::Deref for crate::UnityEngine::NoAllocHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NoAllocHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::NoAllocHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NoAllocHelpers")]
impl crate::UnityEngine::NoAllocHelpers {
    pub fn EnsureListElemCount<T>(
        list: quest_hook::libil2cpp::Gc<T>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureListElemCount", (list, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractArrayFromList(
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractArrayFromList", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractArrayFromListT<T>(
        list: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractArrayFromListT", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ResizeList(
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_ResizeList", (list, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResizeList<T>(
        list: quest_hook::libil2cpp::Gc<T>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResizeList", (list, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn SafeLength_Gc0(
        values: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SafeLength", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn SafeLength_Gc1<T>(
        values: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SafeLength", (values))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+NoAllocHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::NoAllocHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
