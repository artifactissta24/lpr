const FOO: isize = 10;
const ZST: &() =          std::mem::transmute(FOO   )  ;
    fn macro_rules(){
            match &{} {
        ZST => 9
    };
}
