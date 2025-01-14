#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
#[repr(C)]
#[derive(Debug)]
pub struct DoubleLinkAxis {
    __cordl_parent: crate::MS::Internal::Xml::XPath::Axis,
    pub next: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
}
#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::DoubleLinkAxis {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "DoubleLinkAxis";
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
#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
impl std::ops::Deref for crate::System::Xml::Schema::DoubleLinkAxis {
    type Target = crate::MS::Internal::Xml::XPath::Axis;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
impl std::ops::DerefMut for crate::System::Xml::Schema::DoubleLinkAxis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
impl crate::System::Xml::Schema::DoubleLinkAxis {
    pub fn ConvertTree(
        axis: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>),
                quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
                1usize,
            >("ConvertTree")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ConvertTree", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DoubleLinkAxis,
        > = unsafe { method.invoke_unchecked((), (axis)) };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        axis: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
        inputaxis: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (axis, inputaxis))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        axis: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
        inputaxis: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
                    quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (axis, inputaxis))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Next(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
                0usize,
            >("get_Next")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Next", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::Axis,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_Next(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Next")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_Next", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::DoubleLinkAxis {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
