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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexTree =>
    "System.Text.RegularExpressions"."RegexTree"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (root, caps, capNumList, capTop, capNames, capsList, options),
            )?;
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
