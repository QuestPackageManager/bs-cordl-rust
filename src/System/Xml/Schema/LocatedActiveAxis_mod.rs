#[cfg(feature = "System+Xml+Schema+LocatedActiveAxis")]
#[repr(C)]
#[derive(Debug)]
pub struct LocatedActiveAxis {
    __cordl_parent: crate::System::Xml::Schema::ActiveAxis,
    pub column: i32,
    pub isMatched: bool,
    pub Ks: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::KeySequence>,
}
#[cfg(feature = "System+Xml+Schema+LocatedActiveAxis")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::LocatedActiveAxis {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "LocatedActiveAxis";
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
#[cfg(feature = "System+Xml+Schema+LocatedActiveAxis")]
impl std::ops::Deref for crate::System::Xml::Schema::LocatedActiveAxis {
    type Target = crate::System::Xml::Schema::ActiveAxis;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+LocatedActiveAxis")]
impl std::ops::DerefMut for crate::System::Xml::Schema::LocatedActiveAxis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+LocatedActiveAxis")]
impl crate::System::Xml::Schema::LocatedActiveAxis {
    pub fn New(
        astfield: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
        ks: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::KeySequence>,
        column: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (astfield, ks, column))?;
        Ok(__cordl_object.into())
    }
    pub fn Reactivate(
        &mut self,
        ks: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::KeySequence>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::KeySequence>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Reactivate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Reactivate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ks))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        astfield: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
        ks: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::KeySequence>,
        column: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::Asttree>,
                    quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::KeySequence>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (astfield, ks, column))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Column(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Column")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Column", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+LocatedActiveAxis")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::LocatedActiveAxis {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
