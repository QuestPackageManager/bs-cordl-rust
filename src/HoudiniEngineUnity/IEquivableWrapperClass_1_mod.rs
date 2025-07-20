#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IEquivableWrapperClass_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "IEquivableWrapperClass`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "HoudiniEngineUnity",
                        "IEquivableWrapperClass`1",
                    )
                    .unwrap()
                    .make_generic::<(T)>()
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
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    pub fn IsNull(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsNull")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "IsNull",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::HoudiniEngineUnity::IEquivable_1<T>>
for crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    fn as_ref(&self) -> &crate::HoudiniEngineUnity::IEquivable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HoudiniEngineUnity+IEquivableWrapperClass_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::HoudiniEngineUnity::IEquivable_1<T>>
for crate::HoudiniEngineUnity::IEquivableWrapperClass_1<T> {
    fn as_mut(&mut self) -> &mut crate::HoudiniEngineUnity::IEquivable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
