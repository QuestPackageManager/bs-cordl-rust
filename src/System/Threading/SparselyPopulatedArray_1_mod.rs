#[cfg(feature = "System+Threading+SparselyPopulatedArray_1")]
#[repr(C)]
#[derive(Debug)]
pub struct SparselyPopulatedArray_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub _head: *mut crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
    pub _tail: *mut crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Threading+SparselyPopulatedArray_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::SparselyPopulatedArray_1 < T
    > => "System.Threading"."SparselyPopulatedArray`1" < T >
);
#[cfg(feature = "System+Threading+SparselyPopulatedArray_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::SparselyPopulatedArray_1<T> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SparselyPopulatedArray_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::SparselyPopulatedArray_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SparselyPopulatedArray_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Threading::SparselyPopulatedArray_1<T> {
    pub fn get_Tail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::SparselyPopulatedArrayFragment_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::SparselyPopulatedArrayFragment_1<
            T,
        > = __cordl_object.invoke("get_Tail", ())?;
        Ok(__cordl_ret)
    }
    pub fn Add(
        &mut self,
        element: T,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::SparselyPopulatedArrayAddInfo_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::SparselyPopulatedArrayAddInfo_1<T> = __cordl_object
            .invoke("Add", (element))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        initialSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialSize))?;
        Ok(__cordl_ret)
    }
    pub fn New(initialSize: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialSize))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+SparselyPopulatedArray_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::SparselyPopulatedArray_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
