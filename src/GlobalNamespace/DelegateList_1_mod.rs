#[cfg(feature = "DelegateList_1")]
#[repr(C)]
#[derive(Debug)]
pub struct DelegateList_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_acquireFunc: *mut crate::System::Func_2<
        *mut crate::System::Action_1<T>,
        *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::System::Action_1<T>,
        >,
    >,
    pub m_releaseFunc: *mut crate::System::Action_1<
        *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::System::Action_1<T>,
        >,
    >,
    pub m_callbacks: *mut crate::System::Collections::Generic::LinkedList_1<
        *mut crate::System::Action_1<T>,
    >,
    pub m_invoking: bool,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "DelegateList_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DelegateList_1 < T > => ""
    ."DelegateList`1" < T >
);
#[cfg(feature = "DelegateList_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::DelegateList_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DelegateList_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::DelegateList_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DelegateList_1")]
impl<T: quest_hook::libil2cpp::Type> crate::GlobalNamespace::DelegateList_1<T> {
    pub fn Add(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (action))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn CreateWithGlobalCache() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DelegateList_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::DelegateList_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateWithGlobalCache", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        res: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (res))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        acquireFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut crate::System::Action_1<T>,
                *mut crate::System::Collections::Generic::LinkedListNode_1<
                    *mut crate::System::Action_1<T>,
                >,
            >,
        >,
        releaseFunc: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::System::Collections::Generic::LinkedListNode_1<
                    *mut crate::System::Action_1<T>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (acquireFunc, releaseFunc))?;
        Ok(__cordl_object.into())
    }
    pub fn Remove(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        acquireFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut crate::System::Action_1<T>,
                *mut crate::System::Collections::Generic::LinkedListNode_1<
                    *mut crate::System::Action_1<T>,
                >,
            >,
        >,
        releaseFunc: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::System::Collections::Generic::LinkedListNode_1<
                    *mut crate::System::Action_1<T>,
                >,
            >,
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
            .invoke(".ctor", (acquireFunc, releaseFunc))?;
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
}
#[cfg(feature = "DelegateList_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DelegateList_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
