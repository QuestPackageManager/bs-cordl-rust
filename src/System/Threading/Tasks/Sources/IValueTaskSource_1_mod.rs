#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IValueTaskSource_1<TResult: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::Tasks::Sources::IValueTaskSource_1 < TResult > =>
    "System.Threading.Tasks.Sources"."IValueTaskSource`1" < TResult >
);
#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::Tasks::Sources::IValueTaskSource_1<TResult> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource_1")]
impl<TResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::Tasks::Sources::IValueTaskSource_1<TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource_1")]
impl<
    TResult: quest_hook::libil2cpp::Type,
> crate::System::Threading::Tasks::Sources::IValueTaskSource_1<TResult> {
    pub fn GetResult(&mut self, token: i16) -> quest_hook::libil2cpp::Result<TResult>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TResult = __cordl_object.invoke("GetResult", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStatus(
        &mut self,
        token: i16,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Threading::Tasks::Sources::ValueTaskSourceStatus,
    >
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Threading::Tasks::Sources::ValueTaskSourceStatus = __cordl_object
            .invoke("GetStatus", (token))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnCompleted(
        &mut self,
        continuation: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        token: i16,
        flags: crate::System::Threading::Tasks::Sources::ValueTaskSourceOnCompletedFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCompleted", (continuation, state, token, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Threading+Tasks+Sources+IValueTaskSource_1")]
impl<TResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::Sources::IValueTaskSource_1<TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
