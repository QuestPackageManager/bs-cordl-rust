#[cfg(feature = "System+Runtime+CompilerServices+StrongBox_1")]
#[repr(C)]
#[derive(Debug)]
pub struct StrongBox_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Value: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Runtime+CompilerServices+StrongBox_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::StrongBox_1 <
    T > => "System.Runtime.CompilerServices"."StrongBox`1" < T >
);
#[cfg(feature = "System+Runtime+CompilerServices+StrongBox_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Runtime::CompilerServices::StrongBox_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+StrongBox_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Runtime::CompilerServices::StrongBox_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+StrongBox_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Runtime::CompilerServices::StrongBox_1<T> {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_T1(
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Runtime_CompilerServices_IStrongBox_get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("System.Runtime.CompilerServices.IStrongBox.get_Value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_CompilerServices_IStrongBox_set_Value(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Runtime.CompilerServices.IStrongBox.set_Value", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_T1(
        &mut self,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+StrongBox_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::StrongBox_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+StrongBox_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Runtime::CompilerServices::IStrongBox>
for crate::System::Runtime::CompilerServices::StrongBox_1<T> {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IStrongBox {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+StrongBox_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Runtime::CompilerServices::IStrongBox>
for crate::System::Runtime::CompilerServices::StrongBox_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::CompilerServices::IStrongBox {
        unsafe { std::mem::transmute(self) }
    }
}
