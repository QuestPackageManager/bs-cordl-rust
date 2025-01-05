#[cfg(feature = "System+Linq+Expressions+Interpreter+ConvertHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ConvertHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ConvertHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::ConvertHelper =>
    "System.Linq.Expressions.Interpreter"."ConvertHelper"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+ConvertHelper")]
impl std::ops::Deref for crate::System::Linq::Expressions::Interpreter::ConvertHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ConvertHelper")]
impl std::ops::DerefMut
for crate::System::Linq::Expressions::Interpreter::ConvertHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ConvertHelper")]
impl crate::System::Linq::Expressions::Interpreter::ConvertHelper {
    pub fn ToInt32NoNull(
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToInt32NoNull", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+ConvertHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Linq::Expressions::Interpreter::ConvertHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
