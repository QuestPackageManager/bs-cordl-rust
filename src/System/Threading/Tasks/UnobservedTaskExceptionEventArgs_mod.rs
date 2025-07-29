#[cfg(feature = "cordl_class_System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct UnobservedTaskExceptionEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub m_exception: quest_hook::libil2cpp::Gc<crate::System::AggregateException>,
    pub m_observed: bool,
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "UnobservedTaskExceptionEventArgs";
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
#[cfg(feature = "cordl_class_System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl std::ops::Deref
for crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl std::ops::DerefMut
for crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    pub fn New(
        exception: quest_hook::libil2cpp::Gc<crate::System::AggregateException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (exception))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        exception: quest_hook::libil2cpp::Gc<crate::System::AggregateException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::AggregateException>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (exception))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Threading+Tasks+UnobservedTaskExceptionEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::UnobservedTaskExceptionEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
