#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IReadOnlyList_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::Generic::IReadOnlyList_1 <
    T > => "System.Collections.Generic"."IReadOnlyList`1" < T >
);
#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Collections::Generic::IReadOnlyList_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Collections::Generic::IReadOnlyList_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Collections::Generic::IReadOnlyList_1<T> {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
}
#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::IReadOnlyList_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::IReadOnlyList_1<T> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::IReadOnlyList_1<T> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::IReadOnlyList_1<T> {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<quest_hook::libil2cpp::Gc<T>>
for crate::System::Collections::Generic::IReadOnlyList_1<T> {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Collections::Generic::IReadOnlyList_1<T> {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+IReadOnlyList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::System::Collections::Generic::IReadOnlyList_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
