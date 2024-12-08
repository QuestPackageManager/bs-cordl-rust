#[cfg(feature = "System+Buffers+SpanAction_2")]
#[repr(C)]
#[derive(Debug)]
pub struct SpanAction_2<
    T: quest_hook::libil2cpp::Type,
    TArg: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_T: std::marker::PhantomData<T>,
    __cordl_phantom_TArg: std::marker::PhantomData<TArg>,
}
#[cfg(feature = "System+Buffers+SpanAction_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Buffers::SpanAction_2 < T, TArg > =>
    "System.Buffers"."SpanAction`2" < T, TArg >
);
#[cfg(feature = "System+Buffers+SpanAction_2")]
impl<T: quest_hook::libil2cpp::Type, TArg: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Buffers::SpanAction_2<T, TArg> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+SpanAction_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    TArg: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::System::Buffers::SpanAction_2<T, TArg> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Buffers+SpanAction_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    TArg: quest_hook::libil2cpp::Type,
> crate::System::Buffers::SpanAction_2<T, TArg> {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TArg: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        span: crate::System::Span_1<T>,
        arg: TArg,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TArg: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (span, arg))?;
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
#[cfg(feature = "System+Buffers+SpanAction_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    TArg: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType for crate::System::Buffers::SpanAction_2<T, TArg> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
