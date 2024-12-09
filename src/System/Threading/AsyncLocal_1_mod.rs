#[cfg(feature = "System+Threading+AsyncLocal_1")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncLocal_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub m_valueChangedHandler: *mut crate::System::Action_1<
        crate::System::Threading::AsyncLocalValueChangedArgs_1<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Threading+AsyncLocal_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::AsyncLocal_1 < T > =>
    "System.Threading"."AsyncLocal`1" < T >
);
#[cfg(feature = "System+Threading+AsyncLocal_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::AsyncLocal_1<T> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+AsyncLocal_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::AsyncLocal_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+AsyncLocal_1")]
impl<T: quest_hook::libil2cpp::Type> crate::System::Threading::AsyncLocal_1<T> {
    pub fn New(
        valueChangedHandler: *mut crate::System::Action_1<
            crate::System::Threading::AsyncLocalValueChangedArgs_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (valueChangedHandler))?;
        Ok(__cordl_object)
    }
    pub fn System_Threading_IAsyncLocal_OnValueChanged(
        &mut self,
        previousValueObj: *mut crate::System::Object,
        currentValueObj: *mut crate::System::Object,
        contextChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Threading.IAsyncLocal.OnValueChanged",
                (previousValueObj, currentValueObj, contextChanged),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        valueChangedHandler: *mut crate::System::Action_1<
            crate::System::Threading::AsyncLocalValueChangedArgs_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (valueChangedHandler))?;
        Ok(__cordl_ret)
    }
    pub fn set_Value(
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
            .invoke("set_Value", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Threading+AsyncLocal_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::AsyncLocal_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
