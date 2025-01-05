#[cfg(feature = "Unity+Properties+IPropertyBag_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IPropertyBag_1<TContainer: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
}
#[cfg(feature = "Unity+Properties+IPropertyBag_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Properties::IPropertyBag_1 < TContainer >
    => "Unity.Properties"."IPropertyBag`1" < TContainer >
);
#[cfg(feature = "Unity+Properties+IPropertyBag_1")]
impl<TContainer: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Unity::Properties::IPropertyBag_1<TContainer> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IPropertyBag_1")]
impl<TContainer: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Unity::Properties::IPropertyBag_1<TContainer> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IPropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::IPropertyBag_1<TContainer> {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Unity+Properties+IPropertyBag_1")]
impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::IPropertyBag_1<TContainer> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+IPropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag>>
for crate::Unity::Properties::IPropertyBag_1<TContainer> {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IPropertyBag_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag>>
for crate::Unity::Properties::IPropertyBag_1<TContainer> {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag> {
        unsafe { std::mem::transmute(self) }
    }
}
