#[cfg(feature = "cordl_class_System+Threading+SystemThreading_ThreadLocalDebugView_1")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemThreading_ThreadLocalDebugView_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "cordl_class_System+Threading+SystemThreading_ThreadLocalDebugView_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Threading::SystemThreading_ThreadLocalDebugView_1<T> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading";
    const CLASS_NAME: &'static str = "SystemThreading_ThreadLocalDebugView`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Threading",
                        "SystemThreading_ThreadLocalDebugView`1",
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
#[cfg(feature = "cordl_class_System+Threading+SystemThreading_ThreadLocalDebugView_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::SystemThreading_ThreadLocalDebugView_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Threading+SystemThreading_ThreadLocalDebugView_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::SystemThreading_ThreadLocalDebugView_1<T> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+SystemThreading_ThreadLocalDebugView_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Threading::SystemThreading_ThreadLocalDebugView_1<T> {}
#[cfg(feature = "cordl_class_System+Threading+SystemThreading_ThreadLocalDebugView_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::SystemThreading_ThreadLocalDebugView_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
