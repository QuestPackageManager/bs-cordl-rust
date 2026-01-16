#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+AssignableOf_2")]
#[repr(C)]
#[derive(Debug)]
pub struct AssignableOf_2<
    TBase: quest_hook::libil2cpp::Type,
    TDerived: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::OculusStudios::Platform::Core::AssignableOf_1<TBase>,
    __cordl_phantom_TBase: std::marker::PhantomData<TBase>,
    __cordl_phantom_TDerived: std::marker::PhantomData<TDerived>,
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+AssignableOf_2")]
unsafe impl<
    TBase: quest_hook::libil2cpp::Type,
    TDerived: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::OculusStudios::Platform::Core::AssignableOf_2<TBase, TDerived> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.Platform.Core";
    const CLASS_NAME: &'static str = "AssignableOf`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "OculusStudios.Platform.Core",
                        "AssignableOf`2",
                    )
                    .unwrap()
                    .make_generic::<(TBase, TDerived)>()
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
#[cfg(feature = "OculusStudios+Platform+Core+AssignableOf_2")]
impl<
    TBase: quest_hook::libil2cpp::Type,
    TDerived: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::OculusStudios::Platform::Core::AssignableOf_2<TBase, TDerived> {
    type Target = crate::OculusStudios::Platform::Core::AssignableOf_1<TBase>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+Platform+Core+AssignableOf_2")]
impl<
    TBase: quest_hook::libil2cpp::Type,
    TDerived: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::OculusStudios::Platform::Core::AssignableOf_2<TBase, TDerived> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+Platform+Core+AssignableOf_2")]
impl<
    TBase: quest_hook::libil2cpp::Type,
    TDerived: quest_hook::libil2cpp::Type,
> crate::OculusStudios::Platform::Core::AssignableOf_2<TBase, TDerived> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDerived: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TBase: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDerived: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OculusStudios+Platform+Core+AssignableOf_2")]
impl<
    TBase: quest_hook::libil2cpp::Type,
    TDerived: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::OculusStudios::Platform::Core::AssignableOf_2<TBase, TDerived> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
