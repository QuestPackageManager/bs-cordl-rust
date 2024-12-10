#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IEquivableWrapperClass_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::IEquivableWrapperClass_1 < T
    > => "HoudiniEngineUnity"."IEquivableWrapperClass`1" < T >
);
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    pub fn IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNull", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::HoudiniEngineUnity::IEquivable_1<T>>
for crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    fn as_ref(&self) -> &crate::HoudiniEngineUnity::IEquivable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::HoudiniEngineUnity::IEquivable_1<T>>
for crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    fn as_mut(&mut self) -> &mut crate::HoudiniEngineUnity::IEquivable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
