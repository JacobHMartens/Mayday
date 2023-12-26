unsafe trait Foo {
    fn method(&self, str: *mut String) -> String; 
}

struct Bar;

unsafe impl Foo for Bar {
    fn method(&self, str: *mut String) -> String {
        unsafe {
            return (*str).to_string();
        }
    }
}