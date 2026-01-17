#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+IRequestWithResultConversion_2")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct IRequestWithResultConversion_2<
    TBackend: quest_hook::libil2cpp::Type,
    TFrontend: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TBackend: std::marker::PhantomData<TBackend>,
    __cordl_phantom_TFrontend: std::marker::PhantomData<TFrontend>,
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+IRequestWithResultConversion_2")]
unsafe impl<TBackend: quest_hook::libil2cpp::Type, TFrontend: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type
    for crate::OculusStudios::GraphQL::ClientInterface::IRequestWithResultConversion_2<
        TBackend,
        TFrontend,
    >
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.ClientInterface";
    const CLASS_NAME: &'static str = "IRequestWithResultConversion`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "OculusStudios.GraphQL.ClientInterface",
                "IRequestWithResultConversion`2",
            )
            .unwrap()
            .make_generic::<(TBackend, TFrontend)>()
            .unwrap()
            .unwrap()
        })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "OculusStudios+GraphQL+ClientInterface+IRequestWithResultConversion_2")]
impl<TBackend: quest_hook::libil2cpp::Type, TFrontend: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::OculusStudios::GraphQL::ClientInterface::IRequestWithResultConversion_2<
        TBackend,
        TFrontend,
    >
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+ClientInterface+IRequestWithResultConversion_2")]
impl<TBackend: quest_hook::libil2cpp::Type, TFrontend: quest_hook::libil2cpp::Type>
    std::ops::DerefMut
    for crate::OculusStudios::GraphQL::ClientInterface::IRequestWithResultConversion_2<
        TBackend,
        TFrontend,
    >
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+ClientInterface+IRequestWithResultConversion_2")]
impl<TBackend: quest_hook::libil2cpp::Type, TFrontend: quest_hook::libil2cpp::Type>
    crate::OculusStudios::GraphQL::ClientInterface::IRequestWithResultConversion_2<
        TBackend,
        TFrontend,
    >
{
    pub fn ConvertResponseModel(
        &mut self,
        backendModel: TBackend,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TFrontend>>,
    >
    where
        TBackend: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFrontend: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (TBackend),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TFrontend>,
                        >,
                        1usize,
                    >("ConvertResponseModel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertResponseModel", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TFrontend>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (backendModel))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+IRequestWithResultConversion_2")]
impl<TBackend: quest_hook::libil2cpp::Type, TFrontend: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ObjectType
    for crate::OculusStudios::GraphQL::ClientInterface::IRequestWithResultConversion_2<
        TBackend,
        TFrontend,
    >
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
