# fetchtest
I am trying to use the fetch api in Rust Wasm, but I cannot make it work.  
I am desperate. It is such an easy and frequent use case for frontend applications.  
The promises and Futures are very hard to understand here.  
I need the simplest possible fetch: take a simple webpage somewhere and show it on the screen.  
I don't really care about performance here.  
## javascript
First I will do this simple example in javascript.  
That was simple. It works. The coding style is not really nice, but is not terrible.  
## rust
Then I try to use wasm_bindgen promise. This is the javascript object.  
But as always with typeless languages like javascript you never know what is really inside.  
Rust is a typefull language and we must be aware of the type in any moment.  
Usig a reference to a Closure is terrible. It looks very unreadable.  
So after learning all the beauty of functions, parameters, function calls and returns,  
now we have something that does practically the same, but is completely unreadable.  
Functions are variables, but they contain Closures. The return value is always the same "promise", 
but inside of that is something else we don't know. From javascript we get always the JsValue,...  
Pretty terrible.  
It can work. Everything can work. But is this even coding or making a ball of strings for the cat?  
https://fundamentalsofcode.com/javascript-fetch-api-and-using-async-await/  




Then I will ask somebody to help me to do the same in Rust with wasm-bindgen.  

