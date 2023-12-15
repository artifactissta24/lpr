# ! [ unstable ( feature = "humans" ,
reason = "who ever let humans program computers, we're apparently really bad at it" ,
issue = "none" ) ]
# ! [ feature ( staged_api ) ]
    struct Foo ;
impl Foo {
# [ stable ( feature = "rust1" , since = "1.0.0" ) ]
    const        fn gated ( ) -> u32 { 42 }
}
