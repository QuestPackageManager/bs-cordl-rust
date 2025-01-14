#[cfg(feature = "System+Text+RegularExpressions+RegexTree")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexTree {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Root: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::RegexNode,
    >,
    pub Caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub CapNumList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub CapTop: i32,
    pub CapNames: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub CapsList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub Options: crate::System::Text::RegularExpressions::RegexOptions,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexTree")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Text::RegularExpressions::RegexTree {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text.RegularExpressions";
    const CLASS_NAME: &'static str = "RegexTree";
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
#[cfg(feature = "System+Text+RegularExpressions+RegexTree")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexTree {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexTree")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::RegexTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexTree")]
impl crate::System::Text::RegularExpressions::RegexTree {
    pub fn New(
        root: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexNode,
        >,
        caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capNumList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        capTop: i32,
        capNames: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capsList: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        options: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (root, caps, capNumList, capTop, capNames, capsList, options),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        root: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexNode,
        >,
        caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capNumList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        capTop: i32,
        capNames: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capsList: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        options: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Text::RegularExpressions::RegexNode,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        >,
                    >,
                    crate::System::Text::RegularExpressions::RegexOptions,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (root, caps, capNumList, capTop, capNames, capsList, options),
                )
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexTree")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexTree {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
