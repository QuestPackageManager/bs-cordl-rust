#[cfg(feature = "System+Globalization+Bootstring")]
#[repr(C)]
#[derive(Debug)]
pub struct Bootstring {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub delimiter: char,
    pub base_num: i32,
    pub tmin: i32,
    pub tmax: i32,
    pub skew: i32,
    pub damp: i32,
    pub initial_bias: i32,
    pub initial_n: i32,
}
#[cfg(feature = "System+Globalization+Bootstring")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Globalization::Bootstring {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Globalization";
    const CLASS_NAME: &'static str = "Bootstring";
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
#[cfg(feature = "System+Globalization+Bootstring")]
impl std::ops::Deref for crate::System::Globalization::Bootstring {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+Bootstring")]
impl std::ops::DerefMut for crate::System::Globalization::Bootstring {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Globalization+Bootstring")]
impl crate::System::Globalization::Bootstring {
    pub fn Adapt(
        &mut self,
        delta: i32,
        numPoints: i32,
        firstTime: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Adapt", (delta, numPoints, firstTime))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decode(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("Decode", (s, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeDigit(&mut self, c: char) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("DecodeDigit", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("Encode", (s, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncodeDigit(&mut self, d: i32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("EncodeDigit", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        delimiter: char,
        baseNum: i32,
        tmin: i32,
        tmax: i32,
        skew: i32,
        damp: i32,
        initialBias: i32,
        initialN: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (delimiter, baseNum, tmin, tmax, skew, damp, initialBias, initialN),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        delimiter: char,
        baseNum: i32,
        tmin: i32,
        tmax: i32,
        skew: i32,
        damp: i32,
        initialBias: i32,
        initialN: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (delimiter, baseNum, tmin, tmax, skew, damp, initialBias, initialN),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Globalization+Bootstring")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Globalization::Bootstring {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
