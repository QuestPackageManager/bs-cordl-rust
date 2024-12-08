#[cfg(feature = "System+Collections+Generic+ObjectComparer_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectComparer_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Collections::Generic::Comparer_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Collections+Generic+ObjectComparer_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::Generic::ObjectComparer_1 <
    T > => "System.Collections.Generic"."ObjectComparer`1" < T >
);
#[cfg(feature = "System+Collections+Generic+ObjectComparer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Collections::Generic::ObjectComparer_1<T> {
    type Target = crate::System::Collections::Generic::Comparer_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+ObjectComparer_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Collections::Generic::ObjectComparer_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+ObjectComparer_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Collections::Generic::ObjectComparer_1<T> {
    pub fn Compare(&mut self, x: T, y: T) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Compare", (x, y))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Collections+Generic+ObjectComparer_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::ObjectComparer_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}