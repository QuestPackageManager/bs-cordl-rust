#[cfg(feature = "Newtonsoft+Json+Linq+IJEnumerable_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IJEnumerable_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Newtonsoft+Json+Linq+IJEnumerable_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::IJEnumerable_1 < T > =>
    "Newtonsoft.Json.Linq"."IJEnumerable`1" < T >
);
#[cfg(feature = "Newtonsoft+Json+Linq+IJEnumerable_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Newtonsoft::Json::Linq::IJEnumerable_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+IJEnumerable_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Newtonsoft::Json::Linq::IJEnumerable_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+IJEnumerable_1")]
impl<T: quest_hook::libil2cpp::Type> crate::Newtonsoft::Json::Linq::IJEnumerable_1<T> {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Item(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Linq::IJEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
            >,
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
            crate::Newtonsoft::Json::Linq::IJEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Linq::JToken>,
            >,
        > = __cordl_object.invoke("get_Item", (key))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+IJEnumerable_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::IJEnumerable_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+IJEnumerable_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::Newtonsoft::Json::Linq::IJEnumerable_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+IJEnumerable_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::Newtonsoft::Json::Linq::IJEnumerable_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+IJEnumerable_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::Newtonsoft::Json::Linq::IJEnumerable_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+IJEnumerable_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::Newtonsoft::Json::Linq::IJEnumerable_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
