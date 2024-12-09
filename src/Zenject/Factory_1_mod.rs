#[cfg(feature = "Zenject+Factory_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Factory_1<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::PlaceholderFactory_1<TValue>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Zenject+Factory_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Factory_1 < TValue > => "Zenject"
    ."Factory`1" < TValue >
);
#[cfg(feature = "Zenject+Factory_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::Factory_1<TValue> {
    type Target = crate::Zenject::PlaceholderFactory_1<TValue>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Factory_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::Factory_1<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Factory_1")]
impl<TValue: quest_hook::libil2cpp::Type> crate::Zenject::Factory_1<TValue> {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
#[cfg(feature = "Zenject+Factory_1")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::Factory_1<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
