pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    // std::mem::size_of::<str>();
}

// impl<T, U> Into<U> for T 
// where U: From<T>, {
//     fn into(self) -> U {
//         U::from(self)
//     }    
// }

// impl From<&str> for String {
//     fn from(s: &str) -> String {
//         return String(s)
//     }
// }

// impl Into<String> for String {
//     fn into(self) -> &str {
//         return &str::from(self);
//     }
// }

// impl<&str, String> Into<String> for &str 
// where String: From<&str>, {
//     fn into(self) -> String {
//         String::from(self)
//     }    
// }