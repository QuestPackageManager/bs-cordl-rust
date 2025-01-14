#[cfg(feature = "System+Xml+Linq+XObjectChangeEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct XObjectChangeEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _objectChange: crate::System::Xml::Linq::XObjectChange,
}
#[cfg(feature = "System+Xml+Linq+XObjectChangeEventArgs")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Linq::XObjectChangeEventArgs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Linq";
    const CLASS_NAME: &'static str = "XObjectChangeEventArgs";
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
#[cfg(feature = "System+Xml+Linq+XObjectChangeEventArgs")]
impl std::ops::Deref for crate::System::Xml::Linq::XObjectChangeEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XObjectChangeEventArgs")]
impl std::ops::DerefMut for crate::System::Xml::Linq::XObjectChangeEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Linq+XObjectChangeEventArgs")]
impl crate::System::Xml::Linq::XObjectChangeEventArgs {
    pub fn New(
        objectChange: crate::System::Xml::Linq::XObjectChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (objectChange))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        objectChange: crate::System::Xml::Linq::XObjectChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::Xml::Linq::XObjectChange),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (objectChange))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Linq+XObjectChangeEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Linq::XObjectChangeEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
