#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ContinuationTaskFromResultTask_1<
    TAntecedentResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Threading::Tasks::Task,
    pub m_antecedent: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task_1<TAntecedentResult>,
    >,
    __cordl_phantom_TAntecedentResult: std::marker::PhantomData<TAntecedentResult>,
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
unsafe impl<TAntecedentResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Threading::Tasks::ContinuationTaskFromResultTask_1<
    TAntecedentResult,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Threading.Tasks";
    const CLASS_NAME: &'static str = "ContinuationTaskFromResultTask`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Threading.Tasks",
                        "ContinuationTaskFromResultTask`1",
                    )
                    .unwrap()
                    .make_generic::<(TAntecedentResult)>()
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
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
impl<TAntecedentResult: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Threading::Tasks::ContinuationTaskFromResultTask_1<
    TAntecedentResult,
> {
    type Target = crate::System::Threading::Tasks::Task;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
impl<TAntecedentResult: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Threading::Tasks::ContinuationTaskFromResultTask_1<
    TAntecedentResult,
> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
impl<
    TAntecedentResult: quest_hook::libil2cpp::Type,
> crate::System::Threading::Tasks::ContinuationTaskFromResultTask_1<TAntecedentResult> {
    pub fn InnerInvoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TAntecedentResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("InnerInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InnerInvoke", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        antecedent: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TAntecedentResult>,
        >,
        action: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TAntecedentResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (antecedent, action, state, creationOptions, internalOptions),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        antecedent: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TAntecedentResult>,
        >,
        action: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        creationOptions: crate::System::Threading::Tasks::TaskCreationOptions,
        internalOptions: crate::System::Threading::Tasks::InternalTaskOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TAntecedentResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Threading::Tasks::Task_1<TAntecedentResult>,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Delegate>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::Threading::Tasks::TaskCreationOptions,
                            crate::System::Threading::Tasks::InternalTaskOptions,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (antecedent, action, state, creationOptions, internalOptions),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Threading+Tasks+ContinuationTaskFromResultTask_1")]
impl<TAntecedentResult: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Threading::Tasks::ContinuationTaskFromResultTask_1<
    TAntecedentResult,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
