#[cfg(feature = "UnityEngine+UIElements+UIR+LinkedPool_1")]
#[repr(C)]
#[derive(Debug)]
pub struct LinkedPool_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_CreateFunc: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    pub m_ResetAction: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
    pub m_Limit: i32,
    pub m_PoolFirst: T,
    pub _Count_k__BackingField: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+LinkedPool_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::LinkedPool_1 < T >
    => "UnityEngine.UIElements.UIR"."LinkedPool`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+UIR+LinkedPool_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::UIR::LinkedPool_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+LinkedPool_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::LinkedPool_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+LinkedPool_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::UIR::LinkedPool_1<T> {
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Get(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("Get", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        createFunc: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
        resetAction: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (createFunc, resetAction, limit))?;
        Ok(__cordl_object.into())
    }
    pub fn Return(
        &mut self,
        item: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Return", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        createFunc: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
        resetAction: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
        limit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (createFunc, resetAction, limit))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Count(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Count", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+LinkedPool_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::LinkedPool_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
