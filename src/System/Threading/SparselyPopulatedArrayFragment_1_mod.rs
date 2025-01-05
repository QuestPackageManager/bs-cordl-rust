#[cfg(feature = "System+Threading+SparselyPopulatedArrayFragment_1")]
#[repr(C)]
#[derive(Debug)]
pub struct SparselyPopulatedArrayFragment_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _elements: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    pub _freeCount: i32,
    pub _next: quest_hook::libil2cpp::Gc<
        crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
    >,
    pub _prev: quest_hook::libil2cpp::Gc<
        crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Threading+SparselyPopulatedArrayFragment_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::SparselyPopulatedArrayFragment_1 < T > => "System.Threading"
    ."SparselyPopulatedArrayFragment`1" < T >
);
#[cfg(feature = "System+Threading+SparselyPopulatedArrayFragment_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::SparselyPopulatedArrayFragment_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SparselyPopulatedArrayFragment_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::SparselyPopulatedArrayFragment_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SparselyPopulatedArrayFragment_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Threading::SparselyPopulatedArrayFragment_1<T> {
    pub fn New_SparselyPopulatedArrayFragment_1_1(
        _cordl_size: i32,
        prev: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_size, prev))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_0(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_size))?;
        Ok(__cordl_object.into())
    }
    pub fn SafeAtomicRemove(
        &mut self,
        index: i32,
        expectedElement: T,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("SafeAtomicRemove", (index, expectedElement))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SparselyPopulatedArrayFragment_1_1(
        &mut self,
        _cordl_size: i32,
        prev: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
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
            .invoke(".ctor", (_cordl_size, prev))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Prev(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
        > = __cordl_object.invoke("get_Prev", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+SparselyPopulatedArrayFragment_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::SparselyPopulatedArrayFragment_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
