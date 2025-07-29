#[cfg(feature = "cordl_class_System+Linq+Expressions+MemberListBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct MemberListBinding {
    __cordl_parent: crate::System::Linq::Expressions::MemberBinding,
    pub _Initializers_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
            quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ElementInit>,
        >,
    >,
}
#[cfg(feature = "cordl_class_System+Linq+Expressions+MemberListBinding")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Linq::Expressions::MemberListBinding {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq.Expressions";
    const CLASS_NAME: &'static str = "MemberListBinding";
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
#[cfg(feature = "System+Linq+Expressions+MemberListBinding")]
impl std::ops::Deref for crate::System::Linq::Expressions::MemberListBinding {
    type Target = crate::System::Linq::Expressions::MemberBinding;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberListBinding")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::MemberListBinding {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+MemberListBinding")]
impl crate::System::Linq::Expressions::MemberListBinding {
    pub fn get_Initializers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ElementInit>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Linq::Expressions::ElementInit,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_Initializers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Initializers", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::ObjectModel::ReadOnlyCollection_1<
                quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ElementInit>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+Expressions+MemberListBinding")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::MemberListBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
