#[cfg(feature = "System+Text+RegularExpressions+RegexCode")]
#[repr(C)]
#[derive(Debug)]
pub struct RegexCode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Codes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub Strings: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
    pub TrackCount: i32,
    pub Caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub CapSize: i32,
    pub FCPrefix: crate::System::Nullable_1<
        crate::System::Text::RegularExpressions::RegexPrefix,
    >,
    pub BMPrefix: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::RegexBoyerMoore,
    >,
    pub Anchors: i32,
    pub RightToLeft: bool,
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::RegularExpressions::RegexCode =>
    "System.Text.RegularExpressions"."RegexCode"
);
#[cfg(feature = "System+Text+RegularExpressions+RegexCode")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::RegexCode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCode")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::RegexCode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCode")]
impl crate::System::Text::RegularExpressions::RegexCode {
    pub const Back: i32 = 128i32;
    pub const Back2: i32 = 256i32;
    pub const Backjump: i32 = 35i32;
    pub const Beginning: i32 = 18i32;
    pub const Bol: i32 = 14i32;
    pub const Boundary: i32 = 16i32;
    pub const Branchcount: i32 = 28i32;
    pub const Branchmark: i32 = 24i32;
    pub const Capturemark: i32 = 32i32;
    pub const Ci: i32 = 512i32;
    pub const ECMABoundary: i32 = 41i32;
    pub const End: i32 = 21i32;
    pub const EndZ: i32 = 20i32;
    pub const Eol: i32 = 15i32;
    pub const Forejump: i32 = 36i32;
    pub const Getmark: i32 = 33i32;
    pub const Goto: i32 = 38i32;
    pub const Lazybranch: i32 = 23i32;
    pub const Lazybranchcount: i32 = 29i32;
    pub const Lazybranchmark: i32 = 25i32;
    pub const Mask: i32 = 63i32;
    pub const Multi: i32 = 12i32;
    pub const NonECMABoundary: i32 = 42i32;
    pub const Nonboundary: i32 = 17i32;
    pub const Nothing: i32 = 22i32;
    pub const Notone: i32 = 10i32;
    pub const Notonelazy: i32 = 7i32;
    pub const Notoneloop: i32 = 4i32;
    pub const Notonerep: i32 = 1i32;
    pub const Nullcount: i32 = 26i32;
    pub const Nullmark: i32 = 30i32;
    pub const One: i32 = 9i32;
    pub const Onelazy: i32 = 6i32;
    pub const Oneloop: i32 = 3i32;
    pub const Onerep: i32 = 0i32;
    pub const Prune: i32 = 39i32;
    pub const Ref: i32 = 13i32;
    pub const Rtl: i32 = 64i32;
    pub const Set: i32 = 11i32;
    pub const Setcount: i32 = 27i32;
    pub const Setjump: i32 = 34i32;
    pub const Setlazy: i32 = 8i32;
    pub const Setloop: i32 = 5i32;
    pub const Setmark: i32 = 31i32;
    pub const Setrep: i32 = 2i32;
    pub const Start: i32 = 19i32;
    pub const Stop: i32 = 40i32;
    pub const Testref: i32 = 37i32;
    pub fn New(
        codes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        stringlist: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        trackcount: i32,
        caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capsize: i32,
        bmPrefix: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexBoyerMoore,
        >,
        fcPrefix: crate::System::Nullable_1<
            crate::System::Text::RegularExpressions::RegexPrefix,
        >,
        anchors: i32,
        rightToLeft: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    codes,
                    stringlist,
                    trackcount,
                    caps,
                    capsize,
                    bmPrefix,
                    fcPrefix,
                    anchors,
                    rightToLeft,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn OpcodeBacktracks(Op: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpcodeBacktracks", (Op))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        codes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        stringlist: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
        trackcount: i32,
        caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capsize: i32,
        bmPrefix: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexBoyerMoore,
        >,
        fcPrefix: crate::System::Nullable_1<
            crate::System::Text::RegularExpressions::RegexPrefix,
        >,
        anchors: i32,
        rightToLeft: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    codes,
                    stringlist,
                    trackcount,
                    caps,
                    capsize,
                    bmPrefix,
                    fcPrefix,
                    anchors,
                    rightToLeft,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+RegularExpressions+RegexCode")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::RegexCode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
