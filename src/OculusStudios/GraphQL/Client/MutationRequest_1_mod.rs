#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+MutationRequest_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MutationRequest_1<TInputModel: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::OculusStudios::GraphQL::Client::MutationRequest,
    __cordl_phantom_TInputModel: std::marker::PhantomData<TInputModel>,
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+MutationRequest_1")]
unsafe impl<TInputModel: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::OculusStudios::GraphQL::Client::MutationRequest_1<TInputModel> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.Client";
    const CLASS_NAME: &'static str = "MutationRequest`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "OculusStudios.GraphQL.Client",
                        "MutationRequest`1",
                    )
                    .unwrap()
                    .make_generic::<(TInputModel)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+MutationRequest_1")]
impl<TInputModel: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::OculusStudios::GraphQL::Client::MutationRequest_1<TInputModel> {
    type Target = crate::OculusStudios::GraphQL::Client::MutationRequest;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+MutationRequest_1")]
impl<TInputModel: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::OculusStudios::GraphQL::Client::MutationRequest_1<TInputModel> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+MutationRequest_1")]
impl<
    TInputModel: quest_hook::libil2cpp::Type,
> crate::OculusStudios::GraphQL::Client::MutationRequest_1<TInputModel> {
    pub fn New(
        mutation: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::GraphQLMutationOperation,
        >,
        inputObject: TInputModel,
        forceRequestWhenOffline: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TInputModel: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (mutation, inputObject, forceRequestWhenOffline))?;
        Ok(__cordl_object.into())
    }
    pub fn SupportsClientMutationId(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TInputModel: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("SupportsClientMutationId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SupportsClientMutationId", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        mutation: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::GraphQLMutationOperation,
        >,
        inputObject: TInputModel,
        forceRequestWhenOffline: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TInputModel: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::OculusStudios::GraphQL::Client::GraphQLMutationOperation,
                            >,
                            TInputModel,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (mutation, inputObject, forceRequestWhenOffline),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+MutationRequest_1")]
impl<TInputModel: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::OculusStudios::GraphQL::Client::MutationRequest_1<TInputModel> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
