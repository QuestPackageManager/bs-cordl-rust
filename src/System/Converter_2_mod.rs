#[cfg(feature = "System+Converter_2")]
#[repr(C)]
#[derive(Debug)]
pub struct Converter_2<
    TInput: quest_hook::libil2cpp::Type,
    TOutput: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
    __cordl_phantom_TInput: std::marker::PhantomData<TInput>,
    __cordl_phantom_TOutput: std::marker::PhantomData<TOutput>,
}
#[cfg(feature = "System+Converter_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Converter_2 < TInput, TOutput > =>
    "System"."Converter`2" < TInput, TOutput >
);
#[cfg(feature = "System+Converter_2")]
impl<
    TInput: quest_hook::libil2cpp::Type,
    TOutput: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::System::Converter_2<TInput, TOutput> {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Converter_2")]
impl<
    TInput: quest_hook::libil2cpp::Type,
    TOutput: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::System::Converter_2<TInput, TOutput> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Converter_2")]
impl<
    TInput: quest_hook::libil2cpp::Type,
    TOutput: quest_hook::libil2cpp::Type,
> crate::System::Converter_2<TInput, TOutput> {
    pub fn Invoke(&mut self, input: TInput) -> quest_hook::libil2cpp::Result<TOutput>
    where
        TInput: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TOutput: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TOutput = __cordl_object.invoke("Invoke", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TInput: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TOutput: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TInput: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TOutput: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Converter_2")]
impl<
    TInput: quest_hook::libil2cpp::Type,
    TOutput: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType for crate::System::Converter_2<TInput, TOutput> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
