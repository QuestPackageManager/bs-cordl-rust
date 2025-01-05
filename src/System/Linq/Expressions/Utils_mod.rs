#[cfg(feature = "System+Linq+Expressions+Utils")]
#[repr(C)]
#[derive(Debug)]
pub struct Utils {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Linq+Expressions+Utils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Expressions::Utils =>
    "System.Linq.Expressions"."Utils"
);
#[cfg(feature = "System+Linq+Expressions+Utils")]
impl std::ops::Deref for crate::System::Linq::Expressions::Utils {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Utils")]
impl std::ops::DerefMut for crate::System::Linq::Expressions::Utils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Expressions+Utils")]
impl crate::System::Linq::Expressions::Utils {
    pub fn Constant__cordl_bool0(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ConstantExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConstantExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Constant", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Constant_i32_1(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::Expressions::ConstantExpression>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::ConstantExpression,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Constant", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Linq+Expressions+Utils")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Expressions::Utils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
