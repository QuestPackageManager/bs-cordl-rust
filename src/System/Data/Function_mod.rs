#[cfg(feature = "cordl_class_System+Data+Function")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct Function {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _id: crate::System::Data::FunctionId,
    pub _result: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _isValidateArguments: bool,
    pub _isVariantArgumentList: bool,
    pub _argumentCount: i32,
    pub _parameters: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<crate::System::Type>>,
    >,
}
#[cfg(feature = "cordl_class_System+Data+Function")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::Function {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "Function";
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
#[cfg(feature = "System+Data+Function")]
impl std::ops::Deref for crate::System::Data::Function {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Function")]
impl std::ops::DerefMut for crate::System::Data::Function {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+Function")]
impl crate::System::Data::Function {
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        id: crate::System::Data::FunctionId,
        result: quest_hook::libil2cpp::Gc<crate::System::Type>,
        IsValidateArguments: bool,
        IsVariantArgumentList: bool,
        argumentCount: i32,
        a1: quest_hook::libil2cpp::Gc<crate::System::Type>,
        a2: quest_hook::libil2cpp::Gc<crate::System::Type>,
        a3: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                name,
                id,
                result,
                IsValidateArguments,
                IsVariantArgumentList,
                argumentCount,
                a1,
                a2,
                a3,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        id: crate::System::Data::FunctionId,
        result: quest_hook::libil2cpp::Gc<crate::System::Type>,
        IsValidateArguments: bool,
        IsVariantArgumentList: bool,
        argumentCount: i32,
        a1: quest_hook::libil2cpp::Gc<crate::System::Type>,
        a2: quest_hook::libil2cpp::Gc<crate::System::Type>,
        a3: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::System::Data::FunctionId,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        bool,
                        bool,
                        i32,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                        quest_hook::libil2cpp::Gc<crate::System::Type>,
                    ), quest_hook::libil2cpp::Void, 9usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    name,
                    id,
                    result,
                    IsValidateArguments,
                    IsVariantArgumentList,
                    argumentCount,
                    a1,
                    a2,
                    a3,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Data+Function")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Data::Function {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
