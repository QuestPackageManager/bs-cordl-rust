#[cfg(feature = "TMPro+TMP_ObjectPool_1")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_ObjectPool_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Stack: *mut crate::System::Collections::Generic::Stack_1<T>,
    pub m_ActionOnGet: *mut crate::UnityEngine::Events::UnityAction_1<T>,
    pub m_ActionOnRelease: *mut crate::UnityEngine::Events::UnityAction_1<T>,
    pub _countAll_k__BackingField: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "TMPro+TMP_ObjectPool_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_ObjectPool_1 < T > => "TMPro"
    ."TMP_ObjectPool`1" < T >
);
#[cfg(feature = "TMPro+TMP_ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::TMPro::TMP_ObjectPool_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::TMPro::TMP_ObjectPool_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> crate::TMPro::TMP_ObjectPool_1<T> {
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
        actionOnGet: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<T>,
        >,
        actionOnRelease: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (actionOnGet, actionOnRelease))?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
        element: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", (element))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        actionOnGet: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<T>,
        >,
        actionOnRelease: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (actionOnGet, actionOnRelease))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_countActive(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_countActive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_countAll(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_countAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_countInactive(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_countInactive", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_countAll(
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
            .invoke("set_countAll", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_ObjectPool_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::TMPro::TMP_ObjectPool_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
