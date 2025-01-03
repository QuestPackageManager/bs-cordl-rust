#[cfg(feature = "TMPro+TMP_ListPool_1")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_ListPool_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "TMPro+TMP_ListPool_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_ListPool_1 < T > => "TMPro"
    ."TMP_ListPool`1" < T >
);
#[cfg(feature = "TMPro+TMP_ListPool_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::TMPro::TMP_ListPool_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_ListPool_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::TMPro::TMP_ListPool_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_ListPool_1")]
impl<T: quest_hook::libil2cpp::Type> crate::TMPro::TMP_ListPool_1<T> {
    #[cfg(feature = "TMPro+TMP_ListPool_1+__c")]
    pub type __c = crate::TMPro::TMP_ListPool_1___c<T>;
    pub fn Get() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Release(
        toRelease: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Release", (toRelease))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_ListPool_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::TMPro::TMP_ListPool_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
