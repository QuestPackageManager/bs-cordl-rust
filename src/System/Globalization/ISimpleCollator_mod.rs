#[cfg(feature = "System+Globalization+ISimpleCollator")]
#[repr(C)]
#[derive(Debug)]
pub struct ISimpleCollator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Globalization+ISimpleCollator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Globalization::ISimpleCollator =>
    "System.Globalization"."ISimpleCollator"
);
#[cfg(feature = "System+Globalization+ISimpleCollator")]
impl std::ops::Deref for crate::System::Globalization::ISimpleCollator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+ISimpleCollator")]
impl std::ops::DerefMut for crate::System::Globalization::ISimpleCollator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+ISimpleCollator")]
impl crate::System::Globalization::ISimpleCollator {
    pub fn Compare(
        &mut self,
        s1: *mut crate::System::String,
        idx1: i32,
        len1: i32,
        s2: *mut crate::System::String,
        idx2: i32,
        len2: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Compare", (s1, idx1, len1, s2, idx2, len2, options))?;
        Ok(__cordl_ret)
    }
    pub fn GetSortKey(
        &mut self,
        source: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::SortKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::SortKey = __cordl_object
            .invoke("GetSortKey", (source, options))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOf", (s, target, start, length, opt))?;
        Ok(__cordl_ret)
    }
    pub fn IsPrefix(
        &mut self,
        src: *mut crate::System::String,
        target: *mut crate::System::String,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPrefix", (src, target, opt))?;
        Ok(__cordl_ret)
    }
    pub fn IsSuffix(
        &mut self,
        src: *mut crate::System::String,
        target: *mut crate::System::String,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSuffix", (src, target, opt))?;
        Ok(__cordl_ret)
    }
    pub fn LastIndexOf(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LastIndexOf", (s, target, start, length, opt))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Globalization+ISimpleCollator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Globalization::ISimpleCollator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
