#[cfg(feature = "System+Text+RegularExpressions+RegexTree")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexTree {
    __cordl_parent: crate::System::Object,
    pub Root: *mut crate::System::Text::RegularExpressions::RegexNode,
    pub Caps: *mut crate::System::Collections::Hashtable,
    pub CapNumList: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub CapTop: i32,
    pub CapNames: *mut crate::System::Collections::Hashtable,
    pub CapsList: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub Options: crate::System::Text::RegularExpressions::RegexOptions,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexTree")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexTree =>
    "System.Text.RegularExpressions"."RegexTree"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexTree")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexTree {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        root: *mut crate::System::Text::RegularExpressions::RegexNode,
        caps: *mut crate::System::Collections::Hashtable,
        capNumList: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        capTop: i32,
        capNames: *mut crate::System::Collections::Hashtable,
        capsList: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
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
        Ok(__cordl_ret)
    }
    pub fn New(
        root: *mut crate::System::Text::RegularExpressions::RegexNode,
        caps: *mut crate::System::Collections::Hashtable,
        capNumList: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        capTop: i32,
        capNames: *mut crate::System::Collections::Hashtable,
        capsList: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        options: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (root, caps, capNumList, capTop, capNames, capsList, options),
            )?;
        Ok(__cordl_object)
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
