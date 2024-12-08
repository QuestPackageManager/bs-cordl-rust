#[cfg(feature = "System+Runtime+CompilerServices+TrueReadOnlyCollection_1")]
#[repr(C)]
#[derive(Debug)]
pub struct TrueReadOnlyCollection_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Runtime+CompilerServices+TrueReadOnlyCollection_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::TrueReadOnlyCollection_1 < T > =>
    "System.Runtime.CompilerServices"."TrueReadOnlyCollection`1" < T >
);
#[cfg(feature = "System+Runtime+CompilerServices+TrueReadOnlyCollection_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Runtime::CompilerServices::TrueReadOnlyCollection_1<T> {
    type Target = crate::System::Collections::ObjectModel::ReadOnlyCollection_1<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TrueReadOnlyCollection_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Runtime::CompilerServices::TrueReadOnlyCollection_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TrueReadOnlyCollection_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Runtime::CompilerServices::TrueReadOnlyCollection_1<T> {
    pub fn _ctor(
        &mut self,
        list: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (list))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        list: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (list))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+TrueReadOnlyCollection_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::TrueReadOnlyCollection_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
