#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
#[repr(C)]
#[derive(Debug)]
pub struct CaseInsensitiveAscii {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::CaseInsensitiveAscii {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "CaseInsensitiveAscii";
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
#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
impl std::ops::Deref for crate::System::Net::CaseInsensitiveAscii {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        firstObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secondObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Compare", (firstObject, secondObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        firstObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        secondObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Equals", (firstObject, secondObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn FastGetHashCode(
        &mut self,
        myString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FastGetHashCode", (myString))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        myObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (myObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
impl AsRef<crate::System::Collections::IComparer>
for crate::System::Net::CaseInsensitiveAscii {
    fn as_ref(&self) -> &crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
impl AsMut<crate::System::Collections::IComparer>
for crate::System::Net::CaseInsensitiveAscii {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
impl AsRef<crate::System::Collections::IEqualityComparer>
for crate::System::Net::CaseInsensitiveAscii {
    fn as_ref(&self) -> &crate::System::Collections::IEqualityComparer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+CaseInsensitiveAscii")]
impl AsMut<crate::System::Collections::IEqualityComparer>
for crate::System::Net::CaseInsensitiveAscii {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEqualityComparer {
        unsafe { std::mem::transmute(self) }
    }
}
