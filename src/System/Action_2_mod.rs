#[cfg(feature = "System+Action_2")]
#[repr(C)]
#[derive(Debug)]
pub struct Action_2<T1: quest_hook::libil2cpp::Type, T2: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
    __cordl_phantom_T2: std::marker::PhantomData<T2>,
}
#[cfg(feature = "System+Action_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Action_2 < T1, T2 > => "System"
    ."Action`2" < T1, T2 >
);
#[cfg(feature = "System+Action_2")]
impl<T1: quest_hook::libil2cpp::Type, T2: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Action_2<T1, T2> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Action_2")]
impl<T1: quest_hook::libil2cpp::Type, T2: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Action_2<T1, T2> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Action_2")]
impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> crate::System::Action_2<T1, T2> {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        arg1: T1,
        arg2: T2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (arg1, arg2))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Action_2")]
impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType for crate::System::Action_2<T1, T2> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
