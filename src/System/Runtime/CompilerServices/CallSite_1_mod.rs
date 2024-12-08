#[cfg(feature = "System+Runtime+CompilerServices+CallSite_1")]
#[repr(C)]
#[derive(Debug)]
pub struct CallSite_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Runtime::CompilerServices::CallSite,
    pub Target: T,
    pub Rules: *mut quest_hook::libil2cpp::Il2CppArray<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::CallSite_1 <
    T > => "System.Runtime.CompilerServices"."CallSite`1" < T >
);
#[cfg(feature = "System+Runtime+CompilerServices+CallSite_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Runtime::CompilerServices::CallSite_1<T> {
    type Target = crate::System::Runtime::CompilerServices::CallSite;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Runtime::CompilerServices::CallSite_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Runtime::CompilerServices::CallSite_1<T> {
    #[cfg(feature = "System+Runtime+CompilerServices+CallSite_1+__c")]
    pub type __c = crate::System::Runtime::CompilerServices::CallSite_1___c<T>;
    pub fn AddRule(
        &mut self,
        newRule: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddRule", (newRule))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCustomNoMatchDelegate(
        &mut self,
        invoke: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("CreateCustomNoMatchDelegate", (invoke))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCustomUpdateDelegate(
        &mut self,
        invoke: *mut crate::System::Reflection::MethodInfo,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("CreateCustomUpdateDelegate", (invoke))?;
        Ok(__cordl_ret)
    }
    pub fn CreateMatchMaker(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::CompilerServices::CallSite_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::CompilerServices::CallSite_1<T> = __cordl_object
            .invoke("CreateMatchMaker", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUpdateDelegate_0(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetUpdateDelegate", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUpdateDelegate_ByRefMut1(
        &mut self,
        addr: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetUpdateDelegate", (addr))?;
        Ok(__cordl_ret)
    }
    pub fn MakeUpdateDelegate(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("MakeUpdateDelegate", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveRule(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveRule", (i))?;
        Ok(__cordl_ret)
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_CallSiteBinder0(
        binder: *mut crate::System::Runtime::CompilerServices::CallSiteBinder,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (binder))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
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
    pub fn _ctor_CallSiteBinder0(
        &mut self,
        binder: *mut crate::System::Runtime::CompilerServices::CallSiteBinder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (binder))?;
        Ok(__cordl_ret)
    }
    pub fn get_Update(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("get_Update", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CallSite_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::CallSite_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}