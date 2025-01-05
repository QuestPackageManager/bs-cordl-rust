#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct TMPro_ExtensionMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMPro_ExtensionMethods => "TMPro"
    ."TMPro_ExtensionMethods"
);
#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
impl std::ops::Deref for crate::TMPro::TMPro_ExtensionMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
impl std::ops::DerefMut for crate::TMPro::TMPro_ExtensionMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
impl crate::TMPro::TMPro_ExtensionMethods {
    pub fn ArrayToString(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ArrayToString", (chars))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareRGB_Color32_Color32_0(
        a: crate::UnityEngine::Color32,
        b: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareRGB", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareRGB_Color_Color1(
        a: crate::UnityEngine::Color,
        b: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareRGB", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Color32_Color32_0(
        a: crate::UnityEngine::Color32,
        b: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Color_Color1(
        a: crate::UnityEngine::Color,
        b: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Quaternion_Quaternion_i32_3(
        q1: crate::UnityEngine::Quaternion,
        q2: crate::UnityEngine::Quaternion,
        accuracy: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (q1, q2, accuracy))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compare_Vector3_Vector3_i32_2(
        v1: crate::UnityEngine::Vector3,
        v2: crate::UnityEngine::Vector3,
        accuracy: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compare", (v1, v2, accuracy))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindInstanceID<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        target: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindInstanceID", (list, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntToString_Il2CppArray0(
        unicodes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntToString", (unicodes))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntToString_i32_i32_1(
        unicodes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntToString", (unicodes, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn MinAlpha(
        c1: crate::UnityEngine::Color,
        c2: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MinAlpha", (c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Multiply(
        c1: crate::UnityEngine::Color32,
        c2: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Multiply", (c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Tint_Color32_0(
        c1: crate::UnityEngine::Color32,
        c2: crate::UnityEngine::Color32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Tint", (c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Tint_f32_1(
        c1: crate::UnityEngine::Color32,
        tint: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Tint", (c1, tint))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToIntArray(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToIntArray", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn UintToString(
        unicodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UintToString", (unicodes))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMPro_ExtensionMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMPro_ExtensionMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
