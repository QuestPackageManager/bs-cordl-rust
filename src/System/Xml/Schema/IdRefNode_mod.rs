#[cfg(feature = "System+Xml+Schema+IdRefNode")]
#[repr(C)]
#[derive(Debug)]
pub struct IdRefNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub LineNo: i32,
    pub LinePos: i32,
    pub Next: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::IdRefNode>,
}
#[cfg(feature = "System+Xml+Schema+IdRefNode")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::IdRefNode {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "IdRefNode";
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
#[cfg(feature = "System+Xml+Schema+IdRefNode")]
impl std::ops::Deref for crate::System::Xml::Schema::IdRefNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+IdRefNode")]
impl std::ops::DerefMut for crate::System::Xml::Schema::IdRefNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+IdRefNode")]
impl crate::System::Xml::Schema::IdRefNode {
    pub fn New(
        next: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::IdRefNode>,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (next, id, lineNo, linePos))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        next: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::IdRefNode>,
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::IdRefNode>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (next, id, lineNo, linePos))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+IdRefNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::IdRefNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
