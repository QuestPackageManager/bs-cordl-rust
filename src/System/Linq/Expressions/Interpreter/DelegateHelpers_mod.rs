#[cfg(feature = "System+Linq+Expressions+Interpreter+DelegateHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct DelegateHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DelegateHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::DelegateHelpers =>
    "System.Linq.Expressions.Interpreter"."DelegateHelpers"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+DelegateHelpers")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::DelegateHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DelegateHelpers")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::DelegateHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DelegateHelpers")]
impl crate::System::Linq::Expressions::Interpreter::DelegateHelpers {
    #[cfg(feature = "System+Linq+Expressions+Interpreter+DelegateHelpers+__c")]
    pub type __c = crate::System::Linq::Expressions::Interpreter::DelegateHelpers___c;
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+DelegateHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::DelegateHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
