#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
#[repr(C)]
#[derive(Debug)]
pub struct CaseInsensitiveAscii {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CaseInsensitiveAscii =>
    "System.Net"."CaseInsensitiveAscii"
);
#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
impl std::ops::Deref for crate::System::Net::CaseInsensitiveAscii {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
impl std::ops::DerefMut for crate::System::Net::CaseInsensitiveAscii {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
impl crate::System::Net::CaseInsensitiveAscii {
    pub fn Compare(
        &mut self,
        firstObject: *mut crate::System::Object,
        secondObject: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Compare", (firstObject, secondObject))?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        firstObject: *mut crate::System::Object,
        secondObject: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Equals", (firstObject, secondObject))?;
        Ok(__cordl_ret)
    }
    pub fn FastGetHashCode(
        &mut self,
        myString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FastGetHashCode", (myString))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(
        &mut self,
        myObject: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (myObject))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::CaseInsensitiveAscii {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
